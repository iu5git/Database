#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod db;
pub mod model;
pub mod services;

use db::connection::establish_connection_pool;
use services::tasks_service::tasks_service::*;
use services::users_service::users_service::*;
use tauri::Manager;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let connections = establish_connection_pool().await.unwrap();
    tauri::Builder::default()
        .manage(connections) // Makes connection pool available in all #[tauri::command]
        .invoke_handler(tauri::generate_handler![
            create_task,
            create_user,
            get_all_tasks,
            get_user_tasks,
            add_task,
            set_task_done,
            remove_task,
            get_all_users,
            add_user,
            remove_user,
            try_login_user
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
