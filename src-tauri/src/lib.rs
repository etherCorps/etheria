use crate::checker::colima::{
    check_prerequisites, connect_to_docker, start_colima_service, stop_colima_service,
};
use crate::commands::container::{fetch_container_info, get_container_by_id, get_containers};
use crate::commands::images::{delete_image, get_image_history, get_image_info, get_images};
use crate::state::{AppState, SharedAppState};
use std::sync::Arc;
use tokio::sync::Mutex;

mod checker;
mod commands;
mod constants;
mod docker;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let app_state: SharedAppState = Arc::new(Mutex::new(AppState::new()));
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            check_prerequisites,
            start_colima_service,
            stop_colima_service,
            connect_to_docker,
            // Containers
            get_containers,
            get_container_by_id,
            fetch_container_info,
            // Images
            get_images,
            get_image_info,
            get_image_history,
            delete_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
