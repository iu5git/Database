use chrono::{DateTime, NaiveDateTime, Utc};
use tauri::State;

use crate::{
    db::{connection::DbConnectionPool, tasks_controller},
    model::task::task::Task,
};

#[tauri::command]
pub fn create_task(description: String, done: bool, due_time: String, created_by: i64) -> Task {
    let due_time = DateTime::from_utc(
        NaiveDateTime::parse_from_str(&due_time, "%Y-%m-%dT%H:%M").unwrap(),
        Utc,
    );
    Task::new(description, done, due_time, created_by)
}

#[tauri::command]
pub async fn get_all_tasks<'r>(
    connection: State<'r, DbConnectionPool>,
) -> Result<Vec<Task>, String> {
    let pool = &*connection.connection.lock().await;
    let tasks = tasks_controller::get_all_tasks(pool)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    Ok(tasks)
}

#[tauri::command]
pub async fn get_user_tasks<'r>(
    user_id: i64,
    connection: State<'r, DbConnectionPool>,
) -> Result<Vec<Task>, String> {
    let pool = &*connection.connection.lock().await;
    let tasks = tasks_controller::get_user_tasks(pool, user_id)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    Ok(tasks)
}

#[tauri::command]
pub async fn add_task<'r>(
    task: Task,
    connection: State<'r, DbConnectionPool>,
) -> Result<i64, String> {
    let pool = &*connection.connection.lock().await;
    let task_id = tasks_controller::add_task(pool, task)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    Ok(task_id)
}

#[tauri::command]
pub async fn set_task_done<'r>(
    task_id: i64,
    done: bool,
    connection: State<'r, DbConnectionPool>,
) -> Result<u64, String> {
    let pool = &*connection.connection.lock().await;
    let rows_affected = tasks_controller::set_task_done(pool, task_id, done)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    Ok(rows_affected)
}

#[tauri::command]
pub async fn remove_task<'r>(
    task_id: i64,
    connection: State<'r, DbConnectionPool>,
) -> Result<u64, String> {
    let pool = &*connection.connection.lock().await;
    let rows_affected = tasks_controller::remove_task(pool, task_id)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    Ok(rows_affected)
}
