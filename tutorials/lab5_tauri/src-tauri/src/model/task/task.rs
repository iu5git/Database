use chrono::prelude::*;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use super::dao::task_entity::TaskEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Task {
    id: Option<i64>, // None if created manually, Some(id) if retrieved from db
    description: String,
    done: bool,
    due_time: DateTime<Utc>,
    created_by: i64,
}

impl Task {
    pub fn new(description: String, done: bool, due_time: DateTime<Utc>, created_by: i64) -> Task {
        Task {
            id: None,
            description,
            done,
            due_time,
            created_by,
        }
    }
}

impl From<TaskEntity> for Task {
    fn from(task_entity: TaskEntity) -> Self {
        Task {
            id: Some(task_entity.id),
            description: task_entity.description,
            done: task_entity.done,
            due_time: task_entity.due_time,
            created_by: task_entity.created_by,
        }
    }
}
