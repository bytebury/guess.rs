use std::sync::Arc;

use sqlx::SqlitePool;

pub struct BreakoutRepository {
    db: Arc<SqlitePool>,
}
impl BreakoutRepository {
    pub fn new(db: &Arc<SqlitePool>) -> Self {
        Self { db: db.clone() }
    }
}
