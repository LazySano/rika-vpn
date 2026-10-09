use std::process::{Command, Stdio};

const RUN_KEY: &str = "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const KS_RULE: &str = "RikaVPN Kill Switch";

pub fn set_autostart(enabled: bool) -> Result<(), String> {
    if enabled {
        let exe = std::env::current_exe().map_err(|e| e.to_string())?;
        let status = Command::new("reg")
            .args([
                "add",
                RUN_KEY,
                "/v",
                "RikaVPN",
                "/t",
                "REG_SZ",
                "/d",
                &exe.to_string_lossy(),
                "/f",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|e| format!("فشل تفعيل التشغيل التلقائي: {e}"))?;
        if status.success() {
            Ok(())
        } else {
            Err("تعذّر تفعيل التشغيل التلقائي".into())
        }
    } else {
        let _ = Command::new("reg")
            .args(["delete", RUN_KEY, "/v", "RikaVPN", "/f"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        Ok(())
    }
}

pub fn firewall_block(block: bool) -> Result<(), String> {
    let rule = format!("name={KS_RULE}");
    if block {
        let status = Command::new("netsh")
            .args([
                "advfirewall",
                "firewall",
                "add",
                "rule",
                &rule,
                "dir=out",
                "action=block",
                "enable=yes",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err("يتطلب صلاحيات مدير".into())
        }
    } else {
        let _ = Command::new("netsh")
            .args(["advfirewall", "firewall", "delete", "rule", &rule])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        Ok(())
    }
}
