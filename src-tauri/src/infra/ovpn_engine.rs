use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

pub struct OvpnHandle {
    child: Arc<Mutex<Option<Child>>>,
    config_path: PathBuf,
}

fn find_openvpn() -> Option<PathBuf> {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidate = dir.join("openvpn.exe");
            if candidate.exists() {
                return Some(candidate);
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
    pub fn start(text: &str) -> Result<OvpnHandle, String> {
        let bin = find_openvpn().ok_or_else(|| "OPENVPN_MISSING".to_string())?;
        let dir = std::env::temp_dir().join("rikavpn");
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let config_path = dir.join("client.ovpn");
        std::fs::write(&config_path, text).map_err(|e| e.to_string())?;

        let child = Command::new(&bin)
            .arg("--config")
            .arg(&config_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("فشل تشغيل OpenVPN: {e}"))?;

        tracing::info!("OpenVPN process started");

        Ok(OvpnHandle {
            child: Arc::new(Mutex::new(Some(child))),
            config_path,
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
        let _ = std::fs::remove_file(&self.config_path);
    }
}

impl Drop for OvpnHandle {
    fn drop(&mut self) {
        self.stop();
    }
}
