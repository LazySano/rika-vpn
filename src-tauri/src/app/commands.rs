use crate::app::state::AppState;
use crate::core::profile::{self, ProfileMeta};
use crate::core::wg_config;
use crate::infra::elevate;
use crate::infra::geoip::{self, GeoInfo};
use crate::infra::google_auth::{self, GoogleUser};
use crate::infra::net_monitor::{self, NetCounters, ProbeResult};
use crate::infra::ovpn_engine::OvpnHandle;
use crate::infra::system;
use crate::infra::vpngate::{self, VpnGateServer};
use crate::infra::wg_engine::{EngineHandle, EngineInfo, EngineStats};
use tauri::Manager;

fn resource_file(app: &tauri::AppHandle, name: &str) -> Option<std::path::PathBuf> {
    app.path()
        .resource_dir()
        .ok()
        .map(|dir| dir.join("openvpn").join(name))
}

#[tauri::command]
pub fn parse_profile(name: Option<String>, text: String) -> Result<ProfileMeta, String> {
    profile::parse(name.as_deref(), &text)
}

#[tauri::command]
pub fn is_elevated() -> bool {
    elevate::is_elevated()
}

#[tauri::command]
pub fn relaunch_elevated() -> Result<(), String> {
    elevate::relaunch_elevated()
}

#[tauri::command]
pub fn connect_wireguard(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    text: String,
) -> Result<EngineInfo, String> {
    if !elevate::is_elevated() {
        return Err("ELEVATION_REQUIRED".into());
    }
    let cfg = wg_config::parse(&text)?;
    let handle = EngineHandle::start(cfg, resource_file(&app, "wintun.dll"))?;
    let info = handle.info();
    let mut guard = state.engine.lock().map_err(|_| "state lock poisoned".to_string())?;
    if let Some(mut old) = guard.take() {
        old.stop();
    }
    *guard = Some(handle);
    Ok(info)
}

#[tauri::command]
pub fn connect_openvpn(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    text: String,
) -> Result<String, String> {
    if !elevate::is_elevated() {
        return Err("ELEVATION_REQUIRED".into());
    }
    let handle = OvpnHandle::start(
        &text,
        resource_file(&app, "openvpn.exe"),
        resource_file(&app, "wintun.dll"),
    )?;
    let mut guard = state.ovpn.lock().map_err(|_| "state lock poisoned".to_string())?;
    if let Some(mut old) = guard.take() {
        old.stop();
    }
    *guard = Some(handle);
    Ok("openvpn".into())
}

#[tauri::command]
pub fn disconnect(state: tauri::State<AppState>) -> Result<(), String> {
    if let Ok(mut guard) = state.engine.lock() {
        if let Some(mut handle) = guard.take() {
            handle.stop();
        }
    }
    if let Ok(mut guard) = state.ovpn.lock() {
        if let Some(mut handle) = guard.take() {
            handle.stop();
        }
    }
    Ok(())
}

#[tauri::command]
pub fn google_login(client_id: String, client_secret: String) -> Result<GoogleUser, String> {
    google_auth::login(&client_id, &client_secret)
}

#[tauri::command]
pub fn geoip(host: String) -> Result<GeoInfo, String> {
    geoip::lookup(&host)
}

#[tauri::command]
pub fn fetch_vpngate(
    limit: Option<usize>,
    country: Option<String>,
) -> Result<Vec<VpnGateServer>, String> {
    vpngate::fetch(limit.unwrap_or(200), country)
}

#[tauri::command]
pub fn network_counters() -> NetCounters {
    net_monitor::counters()
}

#[tauri::command]
pub fn network_probe(host: String, port: u16, count: u32) -> Result<ProbeResult, String> {
    net_monitor::probe(&host, port, count)
}

#[tauri::command]
pub fn set_autostart(enabled: bool) -> Result<(), String> {
    system::set_autostart(enabled)
}

#[tauri::command]
pub fn firewall_block(block: bool) -> Result<(), String> {
    system::firewall_block(block)
}

#[tauri::command]
pub fn connection_stats(state: tauri::State<AppState>) -> Option<EngineStats> {
    if let Ok(guard) = state.engine.lock() {
        if let Some(handle) = guard.as_ref() {
            return Some(handle.stats());
        }
    }
    if let Ok(guard) = state.ovpn.lock() {
        if let Some(handle) = guard.as_ref() {
            return Some(EngineStats {
                received_bytes: 0,
                sent_bytes: 0,
                handshake_secs: None,
                connected: handle.is_running(),
            });
        }
    }
    None
}
