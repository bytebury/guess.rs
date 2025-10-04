use sqlx::{SqlitePool, query_as};
use std::sync::Arc;

use crate::domain::breakout::{Breakout, NewBreakout};

pub struct BreakoutRepository {
    db: Arc<SqlitePool>,
}
impl BreakoutRepository {
    pub fn new(db: &Arc<SqlitePool>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find_by_lookup_id(&self, lookup_id: &str) -> Result<Breakout, sqlx::Error> {
        query_as(r#"SELECT * FROM breakouts WHERE lookup_id = ?"#)
            .bind(lookup_id)
            .fetch_one(self.db.as_ref())
            .await
    }

    pub async fn create(&self, breakout: &NewBreakout) -> Result<Breakout, sqlx::Error> {
        query_as(r#"INSERT INTO breakouts (lookup_id) VALUES (?) RETURNING *"#)
            .bind(&breakout.lookup_id)
            .fetch_one(self.db.as_ref())
            .await
    }
}
