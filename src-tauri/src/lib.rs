mod commands;
mod db;

use std::sync::Mutex;
use tauri::Manager;

pub struct AppState {
    pub db: Mutex<db::Database>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcut("CommandOrControl+Shift+Space", |app, shortcut, event| {
                    if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        if shortcut.matches(tauri_plugin_global_shortcut::Modifiers::SHIFT | tauri_plugin_global_shortcut::Modifiers::CONTROL | tauri_plugin_global_shortcut::Modifiers::SUPER, tauri_plugin_global_shortcut::Code::Space) ||
                           shortcut.matches(tauri_plugin_global_shortcut::Modifiers::SHIFT | tauri_plugin_global_shortcut::Modifiers::CONTROL, tauri_plugin_global_shortcut::Code::Space) {
                               let Some(window) = app.get_webview_window("main") else { return };
                               
                               let is_visible = window.is_visible().unwrap_or(false);
                               let is_focused = window.is_focused().unwrap_or(false);

                               if is_visible && is_focused {
                                   let _ = window.hide();
                               } else {
                                   let _ = window.show();
                                   let _ = window.set_focus();
                                   let _ = app.emit("quick-capture-triggered", ());
                               }
                           }
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Get app data directory
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            // Ensure directory exists
            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");

            // Initialize database
            let db_path = app_data_dir.join("taskflow.db");
            let database = db::Database::new(db_path).expect("Failed to initialize database");

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
