use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Database, schema::tasks};

#[derive(Queryable, Selectable, Deserialize, Serialize, AsChangeset, Clone, Identifiable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = crate::schema::tasks)]
pub struct TaskInsert {
    pub title: String,
    pub description: String,
}


impl Task {
    pub async fn get_all(conn: &Database) -> Vec<Self> {
        conn.run(move |conn| {
            tasks::table.load::<Self>(conn).unwrap()
        }).await
    }
    pub async fn update(&self, conn: &Database) {
        let me = self.clone();
        conn.run(move |conn| {
            diesel::update(tasks::table)
                .filter(tasks::id.eq(me.id))
                .set(me)
                .execute(conn)
                .unwrap();
        }).await
    }
    pub async fn delete(conn: &Database, id: i32) {
        conn.run(move |conn| {
            diesel::delete(tasks::table)
                .filter(tasks::id.eq(id))
                .execute(conn)
                .unwrap();
        }).await
    }
}

impl TaskInsert {
    pub async fn save(&self, conn: &Database) -> Task {
        let me = self.clone();
        conn.run(move |conn| {
            diesel::insert_into(tasks::table)
                .values(me)
                .returning(Task::as_returning())
                .get_result(conn)
                .unwrap()
        }).await
    }
}