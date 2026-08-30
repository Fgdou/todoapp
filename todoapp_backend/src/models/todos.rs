use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Database, schema::todos};

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}


impl Todo {
    pub async fn get_all(conn: &Database) -> Vec<Self> {
        conn.run(move |conn| {
            todos::table.load::<Self>(conn).unwrap()
        }).await
    }
}