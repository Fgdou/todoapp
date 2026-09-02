use dotenvy_macro::dotenv;
use gloo_net::http::Request;

use crate::models::{tasks::{Task, TaskInsert}, users::{Login, User}};

const BACKEND_URL: &str = dotenv!("BACKEND_URL");

pub async fn get_tasks(token: &str) -> Vec<Task> {
    let url = format!("{BACKEND_URL}/tasks/");
    Request::get(&url)
        .header("Token", token)
        .send()
        .await
        .unwrap()
        .json::<_>().await.unwrap()
}

pub async fn update_task(todo: Task, token: &str) {
    let url = format!("{BACKEND_URL}/tasks/");
    Request::put(&url)
        .header("Token", token)
        .json(&todo)
        .unwrap()
        .send()
        .await
        .unwrap();
}

pub async fn new_task(todo: TaskInsert, token: &str) -> Task {
    let url = format!("{BACKEND_URL}/tasks/");
    Request::post(&url)
        .header("Token", token)
        .json(&todo)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn delete_task(id: i32, token: &str) {
    let url = format!("{BACKEND_URL}/tasks/{id}");
    Request::delete(&url)
        .header("Token", token)
        .send()
        .await
        .unwrap();
}

pub async fn login(login: &Login) -> Result<User, String> {
    let url = format!("{BACKEND_URL}/auth/login");
    Request::post(&url)
        .json(login)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn logout(token: &str) {
    let url = format!("{BACKEND_URL}/auth/logout");
    Request::post(&url)
        .header("Token", token)
        .send()
        .await
        .unwrap();
}