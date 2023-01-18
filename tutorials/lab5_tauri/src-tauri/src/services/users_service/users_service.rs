use tauri::State;

use crate::{
    db::{connection::DbConnectionPool, users_controller},
    model::user::user::User,
};

#[tauri::command]
pub fn create_user(login: String, password: String) -> User {
    User::new(login, password)
}

#[tauri::command]
pub async fn get_all_users<'r>(
    connection: State<'r, DbConnectionPool>,
) -> Result<Vec<User>, String> {
    let pool = &*connection.connection.lock().await;
    let users = users_controller::get_all_users(pool)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    Ok(users)
}

#[tauri::command]
pub async fn add_user<'r>(
    user: User,
    connection: State<'r, DbConnectionPool>,
) -> Result<i64, String> {
    let pool = &*connection.connection.lock().await;
    match users_controller::get_user_by_login(pool, user.login()).await {
        Ok(v) => match v {
            Some(_) => return Err(String::from("Already registered")),
            None => (),
        },
        Err(_) => (),
    };

    let user_id = users_controller::add_user(pool, user)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    Ok(user_id)
}

#[tauri::command]
pub async fn remove_user<'r>(
    user_id: i64,
    connection: State<'r, DbConnectionPool>,
) -> Result<u64, String> {
    let pool = &*connection.connection.lock().await;
    let rows_affected = users_controller::remove_user(pool, user_id)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    Ok(rows_affected)
}

#[tauri::command]
pub async fn try_login_user<'r>(
    login: String,
    password: String,
    connection: State<'r, DbConnectionPool>,
) -> Result<Option<User>, String> {
    let pool = &*connection.connection.lock().await;
    let user = users_controller::try_login_user(pool, login, password)
        .await
        .map_err(|e| format!("DB error: {}", e))?;
    Ok(user)
}
