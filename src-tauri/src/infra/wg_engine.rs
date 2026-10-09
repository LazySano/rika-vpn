use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use boringtun::noise::{Tunn, TunnResult};
use boringtun::x25519::{PublicKey, StaticSecret};
use serde::Serialize;

use crate::core::wg_config::{prefix_to_mask, WgConfig};

const BUF: usize = 65535;

#[derive(Debug, Clone, Serialize, Default)]
pub struct EngineStats {
    pub received_bytes: u64,
    pub sent_bytes: u64,
    pub handshake_secs: Option<u64>,
    pub connected: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct EngineInfo {
    pub assigned_ip: String,
    pub endpoint: String,
    pub adapter_index: u32,
}

pub struct EngineHandle {
    running: Arc<AtomicBool>,
    stats: Arc<Mutex<EngineStats>>,
    info: EngineInfo,
    routes: Vec<String>,
    thread: Option<JoinHandle<()>>,
}

fn find_wintun(preferred: Option<std::path::PathBuf>) -> Result<std::path::PathBuf, String> {
    if let Some(p) = preferred {
        if p.exists() {
            return Ok(p);
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            for candidate in [dir.join("wintun.dll"), dir.join("openvpn").join("wintun.dll")] {
                if candidate.exists() {
                    return Ok(candidate);
                }
            }
        }
    }
    let local = std::path::PathBuf::from("wintun.dll");
    if local.exists() {
        return Ok(local);
    }
    Err("لم يتم العثور على wintun.dll".into())
}

fn run_cmd(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn physical_gateway() -> Option<String> {
    let out = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "(Get-NetRoute -DestinationPrefix '0.0.0.0/0' | Select-Object -First 1 -ExpandProperty NextHop)",
        ])
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    let gw = text.trim();
    if gw.is_empty() {
        None
    } else {
        Some(gw.to_string())
    }
}

fn add_route(prefix: &str, if_index: u32) -> Option<String> {
    let (ip, mask) = match prefix.split_once('/') {
        Some((ip, p)) if !ip.contains(':') => {
            let px: u8 = p.parse().unwrap_or(32);
            (ip.to_string(), prefix_to_mask(px.min(32)).to_string())
        }
        Some(_) => return None,
        None if !prefix.contains(':') => (prefix.to_string(), "255.255.255.255".to_string()),
        None => return None,
    };
    if run_cmd(
        "route",
        &["add", &ip, "mask", &mask, "0.0.0.0", "if", &if_index.to_string(), "metric", "1"],
    ) {
        Some(format!("{ip}/{mask}"))
    } else {
        None
    }
}

impl EngineHandle {
    pub fn start(
        cfg: WgConfig,
        wintun_path: Option<std::path::PathBuf>,
    ) -> Result<EngineHandle, String> {
        let dll = find_wintun(wintun_path)?;
        let wintun = unsafe { wintun::load_from_path(&dll) }
            .map_err(|e| format!("فشل تحميل wintun.dll: {e}"))?;

        let adapter = match wintun::Adapter::open(&wintun, "RikaVPN") {
            Ok(a) => a,
            Err(_) => wintun::Adapter::create(&wintun, "RikaVPN", "RikaVPN", None)
                .map_err(|e| format!("فشل إنشاء محوّل الشبكة: {e}"))?,
        };

        adapter
            .set_address(cfg.address)
            .map_err(|e| format!("فشل ضبط العنوان: {e}"))?;
        let _ = adapter.set_netmask(cfg.netmask);
        let _ = adapter.set_mtu(cfg.mtu);
        if !cfg.dns.is_empty() {
            let _ = adapter.set_dns_servers(&cfg.dns);
        }
        let if_index = adapter
            .get_adapter_index()
            .map_err(|e| format!("فشل قراءة فهرس الواجهة: {e}"))?;

        let mut routes = Vec::new();
        if let Some(gw) = physical_gateway() {
            let ep = cfg.endpoint.ip();
            if run_cmd(
                "route",
                &["add", &ep.to_string(), "mask", "255.255.255.255", &gw, "metric", "1"],
            ) {
                routes.push(format!("{}/255.255.255.255", ep));
            }
        }
        for prefix in &cfg.allowed_ips {
            if let Some(r) = add_route(prefix, if_index) {
                routes.push(r);
            }
        }

        let session = Arc::new(
            adapter
                .start_session(wintun::MAX_RING_CAPACITY)
                .map_err(|e| format!("فشل بدء الجلسة: {e}"))?,
        );

        let running = Arc::new(AtomicBool::new(true));
        let stats = Arc::new(Mutex::new(EngineStats {
            connected: false,
            ..Default::default()
        }));

        let info = EngineInfo {
            assigned_ip: cfg.address.to_string(),
            endpoint: cfg.endpoint.to_string(),
            adapter_index: if_index,
        };

        let running_t = running.clone();
        let stats_t = stats.clone();
        let thread = std::thread::spawn(move || {
            run_loop(cfg, session, running_t, stats_t);
        });

        Ok(EngineHandle {
            running,
            stats,
            info,
            routes,
            thread: Some(thread),
        })
    }

    pub fn info(&self) -> EngineInfo {
        self.info.clone()
    }

    pub fn stats(&self) -> EngineStats {
        self.stats.lock().map(|s| s.clone()).unwrap_or_default()
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        for r in &self.routes {
            let parts: Vec<&str> = r.splitn(2, '/').collect();
            if parts.len() == 2 {
                run_cmd("route", &["delete", parts[0], "mask", parts[1]]);
            }
        }
        self.routes.clear();
    }
}

impl Drop for EngineHandle {
    fn drop(&mut self) {
        self.stop();
    }
}

fn write_tunnel(session: &Arc<wintun::Session>, data: &[u8]) {
    if data.is_empty() || data.len() > u16::MAX as usize {
        return;
    }
    if let Ok(mut packet) = session.allocate_send_packet(data.len() as u16) {
        packet.bytes_mut().copy_from_slice(data);
        session.send_packet(packet);
    }
}

fn handle_datagram(
    tunn: &mut Tunn,
    datagram: &[u8],
    outbuf: &mut [u8],
    session: &Arc<wintun::Session>,
    udp: &std::net::UdpSocket,
) {
    let mut owned = datagram.to_vec();
    loop {
        let res = tunn.decapsulate(None, &owned, outbuf);
        match res {
            TunnResult::WriteToNetwork(b) => {
                let _ = udp.send(b);
                owned.clear();
            }
            TunnResult::WriteToTunnelV4(b, _) => {
                write_tunnel(session, b);
                break;
            }
            TunnResult::WriteToTunnelV6(b, _) => {
                write_tunnel(session, b);
                break;
            }
            TunnResult::Done => break,
            TunnResult::Err(e) => {
                tracing::debug!(error = ?e, "decapsulate error");
                break;
            }
        }
    }
}

fn run_loop(cfg: WgConfig, session: Arc<wintun::Session>, running: Arc<AtomicBool>, stats: Arc<Mutex<EngineStats>>) {
    let udp = match std::net::UdpSocket::bind("0.0.0.0:0") {
        Ok(u) => u,
        Err(e) => {
            tracing::error!(error = %e, "bind udp failed");
            return;
        }
    };
    if udp.connect(cfg.endpoint).is_err() {
        tracing::error!("connect udp failed");
        return;
    }
    let _ = udp.set_nonblocking(true);

    let mut tunn = Tunn::new(
        StaticSecret::from(cfg.private_key),
        PublicKey::from(cfg.peer_public_key),
        cfg.preshared_key,
        cfg.keepalive,
        0,
        None,
    );

    let mut netbuf = vec![0u8; BUF];
    let mut outbuf = vec![0u8; BUF];

    if let TunnResult::WriteToNetwork(b) = tunn.format_handshake_initiation(&mut outbuf, false) {
        let _ = udp.send(b);
    }
    tracing::info!(endpoint = %cfg.endpoint, ip = %cfg.address, "WireGuard engine started");

    let mut last_timer = Instant::now();

    while running.load(Ordering::SeqCst) {
        loop {
            match session.try_receive() {
                Ok(Some(packet)) => {
                    let data = packet.bytes();
                    match tunn.encapsulate(data, &mut outbuf) {
                        TunnResult::WriteToNetwork(b) => {
                            let _ = udp.send(b);
                        }
                        TunnResult::Err(e) => tracing::debug!(error = ?e, "encapsulate error"),
                        _ => {}
                    }
                }
                Ok(None) => break,
                Err(_) => break,
            }
        }

        loop {
            match udp.recv(&mut netbuf) {
                Ok(n) => handle_datagram(&mut tunn, &netbuf[..n], &mut outbuf, &session, &udp),
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }

        if last_timer.elapsed() >= Duration::from_millis(250) {
            loop {
                match tunn.update_timers(&mut outbuf) {
                    TunnResult::WriteToNetwork(b) => {
                        let _ = udp.send(b);
                    }
                    _ => break,
                }
            }
            last_timer = Instant::now();
        }

        let (handshake, rx, tx, _loss, _rtt) = tunn.stats();
        if let Ok(mut s) = stats.lock() {
            s.received_bytes = rx as u64;
            s.sent_bytes = tx as u64;
            s.handshake_secs = handshake.map(|d| d.as_secs());
            s.connected = handshake.map(|d| d.as_secs() < 180).unwrap_or(false);
        }

        std::thread::sleep(Duration::from_millis(1));
    }

    let _ = session.shutdown();
    tracing::info!("WireGuard engine stopped");
}

#[allow(dead_code)]
fn gateway_type_check(_: IpAddr, _: Ipv4Addr) {}
