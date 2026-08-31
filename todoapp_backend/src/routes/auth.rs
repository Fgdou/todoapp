use rand::distr::{Alphanumeric, SampleString};
use rocket::{Route, serde::json::Json};

use crate::{Database, models::users::{Token, User, UserLogin, UserRegister}};

pub fn get_routes() -> Vec<Route> {
    routes![register_user, login, user_logout]
}

#[post("/register", data = "<user>")]
pub async fn register_user(conn: Database, user: Json<UserRegister>) -> Json<Result<User, String>> {
    let username = user.username.clone();
    Json(
        User::save(user.into_inner(), &conn)
            .await
            .ok_or(format!("User {username} already exists"))
    )
}

#[post("/login", data = "<user>")]
pub async fn login(conn: Database, user: Json<UserLogin>) -> Json<Result<Token, String>> {
    let user = user.into_inner();
    let id = user.id;
    let token = Token {
        user_id: id,
        token: generate_random_token()
    };
    
    Json(
        match User::get_user(id, &conn).await {
            None => Err(format!("Username {id} does not exist")),
            Some(_) => Ok(token.save(&conn).await)
        }
    )
}

fn generate_random_token() -> String {
    Alphanumeric.sample_string(&mut rand::rng(), 50)
}

#[get("/logout")]
pub async fn user_logout(conn: Database) {

}