use dotenvy_macro::dotenv;
use gloo_net::http::Request;

use crate::models::tasks::{Task, TaskInsert};

const BACKEND_URL: &str = dotenv!("BACKEND_URL");

pub async fn get_tasks() -> Vec<Task> {
    let url = format!("{BACKEND_URL}/tasks/");
    Request::get(&url)
        .send()
        .await
        .unwrap()
        .json::<_>().await.unwrap()
}

pub async fn update_task(todo: Task) {
    let url = format!("{BACKEND_URL}/tasks/");
    Request::put(&url)
        .json(&todo)
        .unwrap()
        .send()
        .await
        .unwrap();
}

pub async fn new_task(todo: TaskInsert) -> Task {
    let url = format!("{BACKEND_URL}/tasks/");
    Request::post(&url)
        .json(&todo)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn delete_task(id: i32) {
    let url = format!("{BACKEND_URL}/tasks/{id}");
    Request::delete(&url)
        .send()
        .await
        .unwrap();
}