use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Login {
    pub username: String,
    pub password: String,
}