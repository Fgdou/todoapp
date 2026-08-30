use dotenvy_macro::dotenv;

use crate::models::todos::{Todo, TodoInsert};

const BACKEND_URL: &str = dotenv!("BACKEND_URL");

pub async fn get_todos() -> Vec<Todo> {
    let url = BACKEND_URL;
    reqwest::get(url)
        .await.unwrap()
        .json::<_>().await.unwrap()
}

pub async fn update_todo(todo: Todo) {
    let url = BACKEND_URL;
    reqwest::Client::new()
        .put(url)
        .json(&todo)
        .send()
        .await
        .unwrap();
}

pub async fn new_todo(todo: TodoInsert) -> Todo {
    let url = BACKEND_URL;
    reqwest::Client::new()
        .post(url)
        .json(&todo)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}