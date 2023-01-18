use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use super::dao::user_entity::UserEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct User {
    id: Option<i64>, // None if created manually, Some(id) if retrieved from db
    login: String,
    password: String,
}

impl User {
    pub fn new(login: String, password: String) -> User {
        User {
            id: None,
            login,
            password,
        }
    }
}

impl From<UserEntity> for User {
    fn from(user_entity: UserEntity) -> Self {
        User {
            id: Some(user_entity.id),
            login: user_entity.login,
            password: user_entity.password,
        }
    }
}
