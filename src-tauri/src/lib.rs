mod app;
mod core;
mod infra;
mod logging;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .manage(app::state::AppState::default())
        .setup(|app| {
            let dir = app
                .path()
                .app_log_dir()
                .unwrap_or_else(|_| std::env::temp_dir());
            logging::init(dir);
            tracing::info!(version = env!("CARGO_PKG_VERSION"), "RikaVPN starting");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            app::commands::parse_profile,
            app::commands::is_elevated,
            app::commands::relaunch_elevated,
            app::commands::connect_wireguard,
            app::commands::connect_openvpn,
            app::commands::disconnect,
            app::commands::network_counters,
            app::commands::network_probe,
            app::commands::set_autostart,
            app::commands::firewall_block,
            app::commands::connection_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
