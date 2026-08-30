use dotenvy_macro::dotenv;
use gloo_net::http::Request;

use crate::models::todos::{Todo, TodoInsert};

const BACKEND_URL: &str = dotenv!("BACKEND_URL");

pub async fn get_todos() -> Vec<Todo> {
    let url = format!("{BACKEND_URL}/tasks/");
    Request::get(&url)
        .send()
        .await
        .unwrap()
        .json::<_>().await.unwrap()
}

pub async fn update_todo(todo: Todo) {
    let url = format!("{BACKEND_URL}/tasks/");
    Request::put(&url)
        .json(&todo)
        .unwrap()
        .send()
        .await
        .unwrap();
}

pub async fn new_todo(todo: TodoInsert) -> Todo {
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

pub async fn delete_todo(id: i32) {
    let url = format!("{BACKEND_URL}/tasks/{id}");
    Request::delete(&url)
        .send()
        .await
        .unwrap();
}