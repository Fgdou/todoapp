#[macro_use] extern crate rocket;

use dotenvy::dotenv;
use rocket::{serde::json::Json};
use rocket_sync_db_pools::database;

use crate::models::todos::{Todo, TodoInsert};

pub mod schema;
pub mod models;

#[get("/")]
async fn list_todos(conn: Database) -> Json<Vec<Todo>> {
    Json(Todo::get_all(&conn).await)
}

#[post("/", data = "<todo>")]
async fn create_todo(conn: Database, todo: Json<TodoInsert>) -> Json<Todo> {
    Json(todo.save(&conn).await)
}

#[put("/", data = "<todo>")]
async fn update_todo(conn: Database, todo: Json<Todo>) {
    todo.update(&conn).await
}

#[delete("/<id>")]
async fn delete_todo(conn: Database, id: i32) {
    Todo::delete(&conn, id).await
}

#[database("sqlite")]
pub struct Database(diesel::SqliteConnection);

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(Database::fairing())
        .mount("/", routes![list_todos, create_todo, update_todo, delete_todo])
}
