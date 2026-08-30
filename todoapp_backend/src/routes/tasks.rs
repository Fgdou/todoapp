use rocket::serde::json::Json;

use crate::{Database, models::todos::{Todo, TodoInsert}};


#[get("/")]
pub async fn list_todos(conn: Database) -> Json<Vec<Todo>> {
    Json(Todo::get_all(&conn).await)
}

#[post("/", data = "<todo>")]
pub async fn create_todo(conn: Database, todo: Json<TodoInsert>) -> Json<Todo> {
    Json(todo.save(&conn).await)
}

#[put("/", data = "<todo>")]
pub async fn update_todo(conn: Database, todo: Json<Todo>) {
    todo.update(&conn).await
}

#[delete("/<id>")]
pub async fn delete_todo(conn: Database, id: i32) {
    Todo::delete(&conn, id).await
}