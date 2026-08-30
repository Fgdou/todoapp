use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Database, schema::todos};

#[derive(Queryable, Selectable, Deserialize, Serialize, AsChangeset, Clone, Identifiable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = crate::schema::todos)]
pub struct TodoInsert {
    pub title: String,
    pub description: String,
}


impl Todo {
    pub async fn get_all(conn: &Database) -> Vec<Self> {
        conn.run(move |conn| {
            todos::table.load::<Self>(conn).unwrap()
        }).await
    }
    pub async fn update(&self, conn: &Database) {
        let me = self.clone();
        conn.run(move |conn| {
            diesel::update(todos::table)
                .filter(todos::id.eq(me.id))
                .set(me)
                .execute(conn)
                .unwrap();
        }).await
    }
    pub async fn delete(conn: &Database, id: i32) {
        conn.run(move |conn| {
            diesel::delete(todos::table)
                .filter(todos::id.eq(id))
                .execute(conn)
                .unwrap();
        }).await
    }
}

impl TodoInsert {
    pub async fn save(&self, conn: &Database) -> Todo {
        let me = self.clone();
        conn.run(move |conn| {
            diesel::insert_into(todos::table)
                .values(me)
                .returning(Todo::as_returning())
                .get_result(conn)
                .unwrap()
        }).await
    }
}