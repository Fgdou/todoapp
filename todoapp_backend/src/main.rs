#[macro_use] extern crate rocket;

use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;
use rocket::fairing::AdHoc;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_sync_db_pools::database;

use crate::routes::tasks;

pub mod schema;
pub mod models;
pub mod routes;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();


#[database("sqlite")]
pub struct Database(diesel::SqliteConnection);

#[launch]
fn rocket() -> _ {
    println!("Starting backend");

    let allowed_origins = AllowedOrigins::all();

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![
                rocket::http::Method::Get,
                rocket::http::Method::Put,
                rocket::http::Method::Delete,
                rocket::http::Method::Post,
            ].into_iter().map(From::from).collect(),
            allow_credentials: true,
            ..Default::default()
    }.to_cors().unwrap();

    dotenv().ok();
    rocket::build()
        .attach(cors)
        .attach(Database::fairing())
        .attach(AdHoc::on_ignite("Database Migrations", |rocket| async move {
            println!("Running database Migrations...");
            // 1. Extract the SQLite database connection string from Rocket's config
            let db_url = rocket.figment()
                .extract_inner::<String>("databases.sqlite.url")
                .expect("Missing database URL in config");

            // 2. Spawn a blocking thread safely outside Rocket's async pool
            rocket::tokio::task::spawn_blocking(move || {
                use diesel::prelude::*;

                let mut conn = SqliteConnection::establish(&db_url)
                    .expect("Failed to connect to SQLite database for migrations");

                conn.run_pending_migrations(MIGRATIONS)
                    .expect("Failed to run SQLite migrations");
            })
            .await
            .expect("Migration task failed");

            println!("Done !");

            rocket
        }))
        .mount("/tasks", tasks::get_routes())
}
