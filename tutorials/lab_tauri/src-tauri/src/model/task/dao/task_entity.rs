use chrono::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TaskEntity {
    pub id: i64,
    pub description: String,
    pub done: bool,
    pub due_time: DateTime<Utc>,
    pub created_by: i64,
}
