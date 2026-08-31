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
        let fail = |msg: &'static str| {
            req.local_cache(|| msg.to_string());
            request::Outcome::Error((Status::Unauthorized, msg.to_string()))
        };
        
        let conn = match req.guard::<Database>().await {
            request::Outcome::Success(conn) => conn,
            request::Outcome::Error((status, _)) => {
                return request::Outcome::Error((status, "Database connection failed".into()))
            }
            request::Outcome::Forward(status) => return request::Outcome::Forward(status),
        };

        let header_token = match req.headers().get_one("Token") {
            Some(token) => token,
            None => return fail("No Token provided"),
        };

        let token = match Token::get_token(header_token.to_string(), &conn).await {
            Some(token) => token,
            None => return fail("Token is not valid")
        };

        let user = User::get_user(token.user_id, &conn).await.unwrap();

        request::Outcome::Success(Self {
            user,
            token
        })
    }
}

#[catch(401)]
pub fn unauthorized(req: &Request) -> String {
    let msg = req.local_cache(|| "Unauthorized access".to_string()).clone();
    format!("Unauthorized: {msg}")
}