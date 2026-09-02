use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Database, schema::tasks};

#[derive(Queryable, Selectable, Deserialize, Serialize, Clone, Identifiable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub user_id: i32,
}

#[derive(Deserialize, Serialize, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TaskUpdate {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[derive(Deserialize, Clone)]
pub struct TaskInsert {
    pub title: String,
    pub description: String,
}
#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = crate::schema::tasks)]
pub struct TaskInsertWithUser {
    pub title: String,
    pub description: String,
    pub user_id: i32,
}


impl Task {
    pub async fn get_all(conn: &Database, user_id: i32) -> Vec<Self> {
        conn.run(move |conn| {
            tasks::table
                .filter(tasks::user_id.eq(user_id))
                .get_results(conn)
                .unwrap()
        }).await
    }
    pub async fn get(conn: &Database, id: i32) -> Option<Self> {
        conn.run(move |conn| {
            tasks::table
                .filter(tasks::id.eq(id))
                .first::<Self>(conn)
                .ok()
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
impl TaskUpdate {

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
}

impl TaskInsert {
    pub async fn save(&self, conn: &Database, user_id: i32) -> Task {
        let task = TaskInsertWithUser {
            user_id,
            title: self.title.clone(),
            description: self.description.clone(),
        };
        conn.run(move |conn| {
            diesel::insert_into(tasks::table)
                .values(task)
                .returning(Task::as_returning())
                .get_result(conn)
                .unwrap()
        }).await
    }
}