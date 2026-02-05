mod db;
mod commands;

use tauri::Manager;
use std::sync::Mutex;

pub struct AppState {
    pub db: Mutex<db::Database>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Get app data directory
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to get app data directory");
            
            // Ensure directory exists
            std::fs::create_dir_all(&app_data_dir)
                .expect("Failed to create app data directory");
            
            // Initialize database
            let db_path = app_data_dir.join("taskflow.db");
            let database = db::Database::new(db_path)
                .expect("Failed to initialize database");
            
            // Set up app state
            app.manage(AppState {
                db: Mutex::new(database),
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::quick_capture,
            commands::get_dashboard_data,
            commands::update_task_status,
            commands::delete_task,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
