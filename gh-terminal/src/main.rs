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
        .manage(app_state.clone())
        .setup(move |app| {
            let handle = app.handle().clone();
            let state = app_state.clone();

            tauri::async_runtime::spawn(async move {
                serial_port::run(handle, state).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::send_command,
            commands::get_status,
            commands::get_params,
            commands::get_meter,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
