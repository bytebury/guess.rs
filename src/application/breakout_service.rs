use std::sync::Arc;

use sqlx::SqlitePool;

use crate::{
    domain::breakout::{Breakout, NewBreakout},
    infrastructure::db::BreakoutRepository,
};

pub struct BreakoutService {
    breakout_repository: BreakoutRepository,
}
impl BreakoutService {
    pub fn new(db: &Arc<SqlitePool>) -> Self {
        Self {
            breakout_repository: BreakoutRepository::new(db),
        }
    }

    pub async fn find_by_lookup_id(&self, lookup_id: String) -> Result<Breakout, sqlx::Error> {
        self.breakout_repository.find_by_lookup_id(&lookup_id).await
    }

    pub async fn create(&self, breakout: &NewBreakout) -> Result<Breakout, sqlx::Error> {
        self.breakout_repository.create(breakout).await
    }
}
