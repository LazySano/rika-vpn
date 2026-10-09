use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

pub struct OvpnHandle {
    child: Arc<Mutex<Option<Child>>>,
    config_path: PathBuf,
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
    pub fn start(text: &str, preferred: Option<PathBuf>) -> Result<OvpnHandle, String> {
        let bin = find_openvpn(preferred).ok_or_else(|| "OPENVPN_MISSING".to_string())?;
        let dir = std::env::temp_dir().join("rikavpn");
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let config_path = dir.join("client.ovpn");
        std::fs::write(&config_path, text).map_err(|e| e.to_string())?;

        let working_dir = bin.parent().map(|p| p.to_path_buf());
        let mut command = Command::new(&bin);
        command
            .arg("--config")
            .arg(&config_path)
            .arg("--windows-driver")
            .arg("wintun")
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        if let Some(dir) = working_dir {
            command.current_dir(dir);
        }
        let child = command
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
