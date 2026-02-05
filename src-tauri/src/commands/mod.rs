use crate::{
    db::{DashboardData, Task, TaskStatus},
    nlp, AppState,
};
use tauri::State;

#[tauri::command]
pub fn quick_capture(title: String, state: State<AppState>) -> Result<Task, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    let parsed = nlp::parse_task_input(&title);

    db.create_task(
        parsed.title,
        TaskStatus::Next,
        parsed.context,
        parsed.scheduled_for,
        Some(title),
        "quick_capture".to_string(),
        None,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_dashboard_data(state: State<AppState>) -> Result<DashboardData, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_dashboard_data().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_task_status(
    id: String,
    status: String,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    let task_status =
        TaskStatus::from_str(&status).ok_or_else(|| format!("Invalid status: {}", status))?;

    db.update_task_status(&id, task_status)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_task(id: String, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.delete_task(&id).map_err(|e| e.to_string())
}
