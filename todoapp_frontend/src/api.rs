use dotenvy_macro::dotenv;

use crate::models::todos::Todo;

const BACKEND_URL: &str = dotenv!("BACKEND_URL");

pub async fn get_todos() -> Vec<Todo> {
    let url = format!("{BACKEND_URL}/");
    reqwest::get(url)
        .await.unwrap()
        .json::<_>().await.unwrap()
}