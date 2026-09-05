use diesel::{prelude::*, result::{DatabaseErrorKind::UniqueViolation, Error::DatabaseError}};
use serde::{Deserialize, Serialize};

use crate::{Database, schema};

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, PartialEq, Clone)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
}

#[derive(Serialize, Deserialize, Queryable, Selectable, PartialEq, Clone, Insertable)]
#[diesel(table_name = schema::user_token)]
#[diesel(belongs_to(schema::users))]
pub struct Token {
    pub token: String,
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Insertable, Clone, PartialEq)]
#[diesel(table_name = schema::users)]
pub struct UserRegister {
    pub username: String,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserLoginResponse {
    pub user_id: i32,
    pub username: String,
    pub token: String,
}

impl User {
    pub async fn delete(id: i32, conn: &Database) {
        conn.run(move |c| {
            diesel::delete(schema::users::table)
                .filter(schema::users::id.eq(id))
                .execute(c)
                .unwrap()
        }).await;
    }
    pub async fn get_user(id: i32, conn:&Database) -> Option<Self> {
        conn.run(move |c| {
            schema::users::table
                .filter(schema::users::id.eq(id))
                .first::<User>(c)
                .ok()
        }).await
    }
    
    pub async fn get_by_username(username: String, conn: &Database) -> Option<Self> {
        conn.run(move |c| {
            schema::users::table
                .filter(schema::users::username.eq(username))
                .first::<User>(c)
                .ok()
        }).await
    }
}

impl Token {
    pub async fn save(self, conn: &Database) -> Self {
        conn.run(move |c| {
            diesel::insert_into(schema::user_token::table)
                .values(self)
                .returning(Self::as_returning())
                .get_result(c)
                .unwrap()
        }).await
    }
    pub async fn get_token(token: String, conn: &Database) -> Option<Token> {
        conn.run(move |c| {
            schema::user_token::table
                .filter(schema::user_token::token.eq(token))
                .first::<Token>(c)
                .ok()
        }).await
    }
    pub async fn invalidate(token: String, conn: &Database) {
        conn.run(move |c| {
            diesel::delete(schema::user_token::table)
                .filter(schema::user_token::token.eq(token))
                .execute(c)
                .unwrap()
        }).await;
    }
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
        }
    }
}

impl UserRegister {

    pub async fn save(&self, conn: &Database) -> Option<User> {
        let user = self.clone();
        conn.run(move |c| {
            let res = diesel::insert_into(schema::users::table)
                .values(user)
                .returning(User::as_returning())
                .get_result(c);

            match res {
                Ok(u) => Some(u),
                Err(DatabaseError(UniqueViolation, _)) => None,
                Err(e) => panic!("{:?}", e),
            }
        }).await
    }
}