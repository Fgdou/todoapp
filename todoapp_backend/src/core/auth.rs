use rocket::{Request, http::Status, request::{self, FromRequest}};

use crate::{Database, models::users::{Token, User}};

pub struct Auth {
    pub user: User,
    pub token: Token,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let conn = match req.guard::<Database>().await {
            request::Outcome::Success(conn) => conn,
            request::Outcome::Error((status, _)) => {
                return request::Outcome::Error((status, "Database connection failed".into()))
            }
            request::Outcome::Forward(status) => return request::Outcome::Forward(status),
        };

        let header_token = match req.headers().get_one("Token") {
            Some(token) => token,
            None => return request::Outcome::Error((Status::Unauthorized, "No Token provided".to_string())),
        };

        let token = match Token::get_token(header_token.to_string(), &conn).await {
            Some(token) => token,
            None => return request::Outcome::Error((Status::Unauthorized, "Token is not valid".to_string()))
        };

        let user = User::get_user(token.user_id, &conn).await.unwrap();

        request::Outcome::Success(Self {
            user,
            token
        })
    }
}

#[catch(401)]
pub fn unauthorized() -> String {
    "Unauthorized".to_string()
}