use dotenvy_macro::dotenv;
use gloo_net::http::Request;
use web_sys::RequestCredentials;

use crate::models::{tasks::{Task, TaskInsert}, users::{Login, User}};

const BACKEND_URL: &str = dotenv!("BACKEND_URL");

pub async fn get_tasks() -> Vec<Task> {
    let url = format!("{BACKEND_URL}/tasks/");
    Request::get(&url)
        .credentials(RequestCredentials::Include)
        .send()
        .await
        .unwrap()
        .json::<_>().await.unwrap()
}

pub async fn update_task(todo: Task) {
    let url = format!("{BACKEND_URL}/tasks/");
    Request::put(&url)
        .credentials(RequestCredentials::Include)
        .json(&todo)
        .unwrap()
        .send()
        .await
        .unwrap();
}

pub async fn new_task(todo: TaskInsert) -> Task {
    let url = format!("{BACKEND_URL}/tasks/");
    Request::post(&url)
        .credentials(RequestCredentials::Include)
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
        .credentials(RequestCredentials::Include)
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

pub async fn logout() {
    let url = format!("{BACKEND_URL}/auth/logout");
    Request::post(&url)
        .credentials(RequestCredentials::Include)
        .send()
        .await
        .unwrap();
}

pub async fn oidc_redirect(code: &str) -> User {
    let url = format!("{BACKEND_URL}/auth/oidc/redirect?code={code}");
    Request::get(&url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn oidc_get_url() -> String {
    let url = format!("{BACKEND_URL}/auth/oidc/authorize");
    Request::get(&url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

pub async fn oidc_exists() -> bool {
    let url = format!("{BACKEND_URL}/auth/oidc");
    let res = Request::get(&url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    return res.to_lowercase() == "true"
}