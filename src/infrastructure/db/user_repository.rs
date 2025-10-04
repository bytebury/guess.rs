use std::sync::Arc;

use sqlx::{SqlitePool, query_as};

use crate::domain::user::{NewUser, UpdateUser, User};

pub struct UserRepository {
    db: Arc<SqlitePool>,
}
impl UserRepository {
    pub fn new(db: &Arc<SqlitePool>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find_by_lookup_id(&self, lookup_id: &str) -> Result<User, sqlx::Error> {
        query_as(r#"SELECT * FROM users WHERE lookup_id = ?"#)
            .bind(lookup_id)
            .fetch_one(self.db.as_ref())
            .await
    }

    pub async fn create(&self, user: &NewUser) -> Result<User, sqlx::Error> {
        query_as(r#"INSERT INTO users (lookup_id) VALUES (?) RETURNING *"#)
            .bind(&user.lookup_id)
            .fetch_one(self.db.as_ref())
            .await
    }

    pub async fn update(&self, user: &UpdateUser) -> Result<User, sqlx::Error> {
        query_as(r#"UPDATE users SET display_name = ? WHERE id = ? RETURNING *"#)
            .bind(&user.display_name)
            .bind(&user.id)
            .fetch_one(self.db.as_ref())
            .await
    }
}
