#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod handler;
mod serial_port;
mod state;

use std::sync::Arc;
use tracing::info;

use crate::state::AppState;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting GH-Terminal...");

    let app_state = Arc::new(AppState::new());

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::list_serial_ports,
            commands::connect_serial,
            commands::disconnect_serial,
            commands::send_command,
            commands::get_status,
            commands::get_params,
            commands::get_meter,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
