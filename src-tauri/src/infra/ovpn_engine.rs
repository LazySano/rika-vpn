use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

const ADAPTER_NAME: &str = "RikaVPN OVPN";

pub struct OvpnHandle {
    child: Arc<Mutex<Option<Child>>>,
    config_path: PathBuf,
    adapter_alive: Arc<AtomicBool>,
    adapter_thread: Option<std::thread::JoinHandle<()>>,
}

fn find_openvpn(preferred: Option<PathBuf>) -> Option<PathBuf> {
    if let Some(p) = preferred {
        if p.exists() {
            return Some(p);
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            for candidate in [dir.join("openvpn.exe"), dir.join("openvpn").join("openvpn.exe")] {
                if candidate.exists() {
                    return Some(candidate);
                }
            }
        }
    }
    if let Ok(path) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path) {
            let candidate = dir.join("openvpn.exe");
            if candidate.exists() {
                return Some(candidate);
            }
        }
    }
    None
}

impl OvpnHandle {
    pub fn start(
        text: &str,
        preferred: Option<PathBuf>,
        wintun_path: Option<PathBuf>,
    ) -> Result<OvpnHandle, String> {
        let bin = find_openvpn(preferred).ok_or_else(|| "OPENVPN_MISSING".to_string())?;

        let dir = std::env::temp_dir().join("rikavpn");
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let config_path = dir.join("client.ovpn");
        std::fs::write(&config_path, text).map_err(|e| e.to_string())?;

        // Keep a Wintun adapter alive for the whole session so OpenVPN can use it.
        let adapter_alive = Arc::new(AtomicBool::new(true));
        let (ready_tx, ready_rx) = mpsc::channel::<Result<(), String>>();
        let alive = adapter_alive.clone();
        let adapter_name = ADAPTER_NAME.to_string();
        let adapter_thread = std::thread::spawn(move || {
            let dll = match wintun_path {
                Some(p) if p.exists() => p,
                _ => {
                    let _ = ready_tx.send(Err("لم يتم العثور على wintun.dll".into()));
                    return;
                }
            };
            let wintun = match unsafe { wintun::load_from_path(&dll) } {
                Ok(w) => w,
                Err(e) => {
                    let _ = ready_tx.send(Err(format!("فشل تحميل wintun.dll: {e}")));
                    return;
                }
            };
            let adapter = match wintun::Adapter::open(&wintun, &adapter_name) {
                Ok(a) => a,
                Err(_) => match wintun::Adapter::create(&wintun, &adapter_name, &adapter_name, None) {
                    Ok(a) => a,
                    Err(e) => {
                        let _ = ready_tx.send(Err(format!("فشل إنشاء محوّل Wintun: {e}")));
                        return;
                    }
                },
            };
            let _ = ready_tx.send(Ok(()));
            while alive.load(Ordering::SeqCst) {
                std::thread::sleep(Duration::from_millis(400));
            }
            drop(adapter);
            let _ = &wintun;
        });

        match ready_rx.recv_timeout(Duration::from_secs(15)) {
            Ok(Ok(())) => {}
            Ok(Err(e)) => return Err(e),
            Err(_) => return Err("انتهت مهلة إنشاء محوّل Wintun".into()),
        }
        std::thread::sleep(Duration::from_millis(900));

        let working_dir = bin.parent().map(|p| p.to_path_buf());
        let mut command = Command::new(&bin);
        command
            .arg("--config")
            .arg(&config_path)
            .arg("--windows-driver")
            .arg("wintun")
            .arg("--dev-node")
            .arg(ADAPTER_NAME)
            .arg("--block-outside-dns")
            .arg("--block-ipv6")
            .arg("--tun-mtu")
            .arg("1380")
            .arg("--mssfix")
            .arg("1360")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if let Some(d) = working_dir {
            command.current_dir(d);
        }

        let mut child = command
            .spawn()
            .map_err(|e| format!("فشل تشغيل OpenVPN: {e}"))?;

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        let (tx, rx) = mpsc::channel::<String>();

        if let Some(out) = stdout {
            let tx = tx.clone();
            std::thread::spawn(move || {
                for line in BufReader::new(out).lines().map_while(Result::ok) {
                    let _ = tx.send(line);
                }
            });
        }
        if let Some(err) = stderr {
            let tx = tx.clone();
            std::thread::spawn(move || {
                for line in BufReader::new(err).lines().map_while(Result::ok) {
                    let _ = tx.send(line);
                }
            });
        }

        let started = Instant::now();
        let mut tail: Vec<String> = Vec::new();
        loop {
            match rx.recv_timeout(Duration::from_millis(500)) {
                Ok(line) => {
                    tracing::info!(target: "openvpn", "{line}");
                    let low = line.to_ascii_lowercase();
                    if line.contains("Initialization Sequence Completed") {
                        break;
                    }
                    let fatal = low.contains("exiting due to fatal error")
                        || low.starts_with("error")
                        || low.contains("options error")
                        || low.contains("cannot open")
                        || low.contains("all tap-windows")
                        || low.contains("no tap-windows")
                        || low.contains("no adapters")
                        || low.contains("auth_failed")
                        || low.contains("authentication failed")
                        || low.contains("cannot load")
                        || low.contains("tls error")
                        || low.contains("fatal");
                    if fatal {
                        let _ = child.kill();
                        adapter_alive.store(false, Ordering::SeqCst);
                        let mut context = tail.join("\n");
                        if !context.is_empty() {
                            context.push('\n');
                        }
                        context.push_str(&line);
                        return Err(format!("فشل OpenVPN:\n{context}"));
                    }
                    tail.push(line);
                    if tail.len() > 40 {
                        tail.remove(0);
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    adapter_alive.store(false, Ordering::SeqCst);
                    return Err(format!("توقف OpenVPN. آخر السجل:\n{}", tail.join("\n")));
                }
            }
            if let Ok(Some(_)) = child.try_wait() {
                adapter_alive.store(false, Ordering::SeqCst);
                return Err(format!("توقف OpenVPN مبكرًا. آخر السجل:\n{}", tail.join("\n")));
            }
            if started.elapsed() > Duration::from_secs(45) {
                let _ = child.kill();
                adapter_alive.store(false, Ordering::SeqCst);
                return Err(format!(
                    "انتهت مهلة الاتصال بـ OpenVPN. آخر السجل:\n{}",
                    tail.join("\n")
                ));
            }
        }

        std::thread::spawn(move || {
            while let Ok(line) = rx.recv() {
                tracing::info!(target: "openvpn", "{line}");
            }
        });

        tracing::info!("OpenVPN connected");

        Ok(OvpnHandle {
            child: Arc::new(Mutex::new(Some(child))),
            config_path,
            adapter_alive,
            adapter_thread: Some(adapter_thread),
        })
    }

    pub fn is_running(&self) -> bool {
        if let Ok(mut guard) = self.child.lock() {
            if let Some(child) = guard.as_mut() {
                return child.try_wait().map(|s| s.is_none()).unwrap_or(false);
            }
        }
        false
    }

    pub fn stop(&mut self) {
        if let Ok(mut guard) = self.child.lock() {
            if let Some(mut child) = guard.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
        self.adapter_alive.store(false, Ordering::SeqCst);
        if let Some(t) = self.adapter_thread.take() {
            let _ = t.join();
        }
        let _ = std::fs::remove_file(&self.config_path);
    }
}

impl Drop for OvpnHandle {
    fn drop(&mut self) {
        self.stop();
    }
}
