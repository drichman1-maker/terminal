// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod ipc_bus;
mod websocket;
mod orderbook;
mod analytics;
mod news;
mod risk;
mod api;
mod window_manager;

use tauri::Manager;
use tracing_subscriber;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tauri::Builder::default()
        .setup(|app| {
            // Initialize IPC bus
            let ipc_bus = ipc_bus::IPCBus::new();
            app.manage(ipc_bus);

            // Initialize window manager
            let window_manager = window_manager::WindowManager::new();
            app.manage(window_manager);

            tracing::info!("Kinetic Terminal initialized");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::subscribe_orderbook,
            commands::unsubscribe_orderbook,
            commands::get_correlation_matrix,
            commands::get_liquidation_map,
            commands::execute_command,
            commands::store_api_key,
            commands::get_api_key,
            commands::trigger_killswitch,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
