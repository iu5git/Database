use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserEntity {
    pub id: i64,
    pub login: String,
    pub password: String,
}