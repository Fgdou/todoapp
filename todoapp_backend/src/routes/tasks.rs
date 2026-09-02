use rocket::{Route, serde::json::Json};

use crate::{Database, core::auth::Auth, models::tasks::{Task, TaskInsert, TaskUpdate}};


#[get("/")]
pub async fn list_tasks(conn: Database, auth: Auth) -> Json<Vec<Task>> {
    Json(Task::get_all(&conn, auth.user.id).await)
}

#[post("/", data = "<todo>")]
pub async fn create_task(conn: Database, todo: Json<TaskInsert>, auth: Auth) -> Json<Task> {
    Json(todo.save(&conn, auth.user.id).await)
}

#[put("/", data = "<todo>")]
pub async fn update_task(conn: Database, todo: Json<TaskUpdate>, auth: Auth) -> Result<(), String> {
    let task = Task::get(&conn, todo.id).await.ok_or("Task not found")?;
    if task.user_id != auth.user.id {
        return Err("The task does not belong to this user".into());
    }
    todo.update(&conn).await;
    Ok(())
}

#[delete("/<id>")]
pub async fn delete_task(conn: Database, id: i32, auth: Auth) -> Result<(), String> {
    let task = Task::get(&conn, id).await.ok_or("Task not found")?;
    if task.user_id != auth.user.id {
        return Err("The task does not belong to this user".into());
    }
    Task::delete(&conn, id).await;
    Ok(())
}

pub fn get_routes() -> Vec<Route> {
    routes![list_tasks, create_task, update_task, delete_task]
}