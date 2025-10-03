use std::sync::Arc;

use sqlx::SqlitePool;

use crate::{domain::user::NewUser, infrastructure::db::UserRepository};

pub struct UserService {
    user_repository: UserRepository,
}
impl UserService {
    pub fn new(db: &Arc<SqlitePool>) -> Self {
        Self {
            user_repository: UserRepository::new(db),
        }
    }

    pub async fn find_by_id(&self, _user_id: i64) {
        todo!();
    }

    pub async fn create(&self, _user: &NewUser) {
        todo!();
    }
}
