use rocket::{Route, serde::json::Json};

use crate::{Database, models::tasks::{Task, TaskInsert}};


#[get("/")]
pub async fn list_tasks(conn: Database) -> Json<Vec<Task>> {
    Json(Task::get_all(&conn).await)
}

#[post("/", data = "<todo>")]
pub async fn create_task(conn: Database, todo: Json<TaskInsert>) -> Json<Task> {
    Json(todo.save(&conn).await)
}

#[put("/", data = "<todo>")]
pub async fn update_task(conn: Database, todo: Json<Task>) {
    todo.update(&conn).await
}

#[delete("/<id>")]
pub async fn delete_task(conn: Database, id: i32) {
    Task::delete(&conn, id).await
}

pub fn get_routes() -> Vec<Route> {
    routes![list_tasks, create_task, update_task, delete_task]
}