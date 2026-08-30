use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[derive(Serialize)]
pub struct TaskInsert {
    pub title: String,
    pub description: String,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}