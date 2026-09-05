use rand::distr::{Alphanumeric, SampleString};
use rocket::{Route, State, serde::json::Json};
use sha2::{Digest, Sha256};

use crate::{Database, core::{auth::Auth, oidc::Oidc}, models::users::{Token, User, UserLogin, UserLoginResponse, UserRegister, UserResponse}};

pub fn get_routes() -> Vec<Route> {
    routes![register_user, login, user_logout, oidc_authorize, oidc_redirect, oidc_exists]
}

#[post("/register", data = "<user>")]
pub async fn register_user(conn: Database, user: Json<UserRegister>) -> Json<Result<UserResponse, String>> {
    let username = user.username.clone();
    let mut user = user.into_inner();
    user.password = Some(hash_password(&user.username, &user.password.unwrap()));
    Json(
        user.save(&conn)
            .await
            .map(|u| u.into())
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

    match user.password {
        None => return Json(Err("This user uses OIDC".into())),
        Some(password) => {
            if password != hashed_password {
                return Json(Err("Password is incorrect".into()))
            }
        }
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

#[get("/oidc/authorize")]
pub async fn oidc_authorize(oidc: &State<Option<Oidc>>) -> String {
    let oidc = oidc.as_ref().unwrap();
    oidc.authorize_new().0
}

#[get("/oidc")]
pub async fn oidc_exists(oidc: &State<Option<Oidc>>) -> String {
    oidc.is_some().to_string()
}

#[get("/oidc/redirect?<code>")]
pub async fn oidc_redirect(conn: Database, code: String, oidc: &State<Option<Oidc>>) -> Json<UserLoginResponse> {
    let oidc = oidc.as_ref().unwrap();
    let username = oidc.validate_authorization(code).await;
    let user = User::get_by_username(username.clone(), &conn).await;

    let user = match user {
        Some(u) => u,
        None => {
            rocket::info!("User {} does not exist, creating it...", &username);
            let user = UserRegister {
                username: username.clone(),
                password: None,
            };
            let user = user.save(&conn).await.unwrap();
            user
        }
    };

    let token = Token {
        user_id: user.id,
        token: generate_random_token(),
    };
    let token = token.save(&conn).await;

    let response = UserLoginResponse {
        user_id: user.id,
        username: user.username,
        token: token.token,
    };

    Json(response)
}

fn generate_random_token() -> String {
    Alphanumeric.sample_string(&mut rand::rng(), 50)
}

#[get("/logout")]
pub async fn user_logout(conn: Database, auth: Auth) {
    Token::invalidate(auth.token.token, &conn).await;
}