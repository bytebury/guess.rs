use std::sync::Arc;

use sqlx::SqlitePool;

use crate::{domain::breakout::NewBreakout, infrastructure::db::BreakoutRepository};

pub struct BreakoutService {
    breakout_repository: BreakoutRepository,
}
impl BreakoutService {
    pub fn new(db: &Arc<SqlitePool>) -> Self {
        Self {
            breakout_repository: BreakoutRepository::new(db),
        }
    }

    pub async fn find_by_id(&self, _breakout_id: i64) {
        todo!();
    }

    pub async fn create(&self, _breakout: &NewBreakout) {
        todo!();
    }
}
