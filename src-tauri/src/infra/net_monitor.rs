use serde::Serialize;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize)]
pub struct NetCounters {
    pub rx: u64,
    pub tx: u64,
}

#[cfg(windows)]
pub fn counters() -> NetCounters {
    use windows_sys::Win32::NetworkManagement::IpHelper::{
        FreeMibTable, GetIfTable2, MIB_IF_TABLE2,
    };
    use windows_sys::Win32::NetworkManagement::Ndis::IfOperStatusUp;
    unsafe {
        let mut table: *mut MIB_IF_TABLE2 = std::ptr::null_mut();
        if GetIfTable2(&mut table) != 0 || table.is_null() {
            return NetCounters { rx: 0, tx: 0 };
        }
        let data = &*table;
        let rows = std::slice::from_raw_parts(data.Table.as_ptr(), data.NumEntries as usize);
        let mut rx = 0u64;
        let mut tx = 0u64;
        for row in rows {
            if row.OperStatus == IfOperStatusUp {
                rx += row.InOctets;
                tx += row.OutOctets;
            }
        }
        FreeMibTable(table as *const core::ffi::c_void);
        NetCounters { rx, tx }
    }
}

#[cfg(not(windows))]
pub fn counters() -> NetCounters {
    NetCounters { rx: 0, tx: 0 }
}

#[derive(Debug, Clone, Serialize)]
pub struct ProbeResult {
    pub avg_ms: f64,
    pub jitter_ms: f64,
    pub loss_pct: f64,
    pub samples: Vec<f64>,
}

pub fn probe(host: &str, port: u16, count: u32) -> Result<ProbeResult, String> {
    use std::net::{TcpStream, ToSocketAddrs};
    let addr = format!("{host}:{port}")
        .to_socket_addrs()
        .map_err(|e| e.to_string())?
        .next()
        .ok_or("تعذّر تحليل المضيف")?;

    let total = count.max(1);
    let mut samples: Vec<f64> = Vec::new();
    for _ in 0..total {
        let start = Instant::now();
        if TcpStream::connect_timeout(&addr, Duration::from_secs(2)).is_ok() {
            samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        std::thread::sleep(Duration::from_millis(60));
    }

    let avg = if samples.is_empty() {
        0.0
    } else {
        samples.iter().sum::<f64>() / samples.len() as f64
    };
    let jitter = if samples.len() < 2 {
        0.0
    } else {
        let diffs: Vec<f64> = samples.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
        diffs.iter().sum::<f64>() / diffs.len() as f64
    };
    let loss = (total as f64 - samples.len() as f64) / total as f64 * 100.0;

    Ok(ProbeResult {
        avg_ms: (avg * 10.0).round() / 10.0,
        jitter_ms: (jitter * 10.0).round() / 10.0,
        loss_pct: (loss * 10.0).round() / 10.0,
        samples,
    })
}
