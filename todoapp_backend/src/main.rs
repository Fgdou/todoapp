#[macro_use] extern crate rocket;

use dotenvy::dotenv;
use rocket::{response::status, serde::json::Json};
use rocket_sync_db_pools::database;

use crate::models::todos::Todo;

pub mod schema;
pub mod models;

#[get("/")]
async fn list_todos(conn: Database) -> status::Accepted<Json<Vec<Todo>>> {
    status::Accepted(Json(Todo::get_all(&conn).await))
}

#[database("sqlite")]
pub struct Database(diesel::SqliteConnection);

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(Database::fairing())
        .mount("/", routes![list_todos])
}
