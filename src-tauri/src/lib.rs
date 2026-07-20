mod commands;
mod db;
mod i18n;
mod models;
mod services;

use commands::locale::LocaleState;
use commands::refresh::RefreshState;
use commands::vpn::VpnState;
use db::Database;
use std::sync::Arc;
use tauri::Manager;

pub struct AppState {
    pub db: Arc<Database>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_dir).ok();

            let db_path = app_dir.join("vpn-hizb.db");
            let db = Database::new(&db_path).expect("Failed to initialize database");

            app.manage(AppState {
                db: Arc::new(db),
            });
            app.manage(VpnState::new());
            app.manage(RefreshState::new());
            app.manage(LocaleState::new());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::vpn::get_status,
            commands::vpn::get_servers,
            commands::vpn::connect,
            commands::vpn::disconnect,
            commands::refresh::refresh_servers,
            commands::refresh::get_refresh_status,
            commands::locale::set_locale,
            commands::locale::get_translations,
            commands::sudoers::check_sudoers,
            commands::sudoers::setup_sudoers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
