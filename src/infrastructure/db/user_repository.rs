use std::sync::Arc;

use sqlx::SqlitePool;

pub struct UserRepository {
    db: Arc<SqlitePool>,
}
impl UserRepository {
    pub fn new(db: &Arc<SqlitePool>) -> Self {
        Self { db: db.clone() }
    }
}
