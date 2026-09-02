use rand::distr::{Alphanumeric, SampleString};
use rocket::{Route, serde::json::Json};
use sha2::{Digest, Sha256};

use crate::{Database, core::auth::Auth, models::users::{Token, User, UserLogin, UserLoginResponse, UserRegister}};

pub fn get_routes() -> Vec<Route> {
    routes![register_user, login, user_logout]
}

#[post("/register", data = "<user>")]
pub async fn register_user(conn: Database, user: Json<UserRegister>) -> Json<Result<User, String>> {
    let username = user.username.clone();
    let mut user = user.into_inner();
    user.password = hash_password(&user.username, &user.password);
    Json(
        User::save(user, &conn)
            .await
            .ok_or(format!("User {username} already exists"))
    )
}

fn hash_password(username: &str, password: &str) -> String {
    let mut hash = Sha256::new();
    hash.update(username);
    hash.update(password);
    let result = hash.finalize();
    hex::encode(result)
}

#[post("/login", data = "<user>")]
pub async fn login(conn: Database, user: Json<UserLogin>) -> Json<Result<UserLoginResponse, String>> {
    let user = user.into_inner();

    let username = user.username;
    let password = user.password;
    let hashed_password = hash_password(&username, &password);

    let user: User = match User::get_by_username(username.clone(), &conn).await {
            None => return Json(Err(format!("Username {username} does not exist"))),
            Some(user) => user
    };

    if hashed_password != user.password {
        return Json(Err("Password is incorrect".into()))
    }

    let token = Token {
        user_id: user.id,
        token: generate_random_token()
    };

    let saved_token = token.save(&conn).await;

    let response = UserLoginResponse {
        user_id: user.id,
        username: user.username,
        token: saved_token.token,
    };

    Json(Ok(response))
}

fn generate_random_token() -> String {
    Alphanumeric.sample_string(&mut rand::rng(), 50)
}

#[get("/logout")]
pub async fn user_logout(conn: Database, auth: Auth) {
    Token::invalidate(auth.token.token, &conn).await;
}