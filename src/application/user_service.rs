use sqlx::SqlitePool;
use std::sync::Arc;

use crate::{
    domain::user::{NewUser, UpdateUser, User},
    infrastructure::db::UserRepository,
};

pub struct UserService {
    user_repository: UserRepository,
}
impl UserService {
    pub fn new(db: &Arc<SqlitePool>) -> Self {
        Self {
            user_repository: UserRepository::new(db),
        }
    }

    pub async fn find_by_lookup_id(&self, lookup_id: &str) -> Result<User, sqlx::Error> {
        let user = self.user_repository.find_by_lookup_id(lookup_id).await?;
        Ok(User::from(user))
    }

    pub async fn create(&self, user: &NewUser) -> Result<User, sqlx::Error> {
        let user = self.user_repository.create(user).await?;
        Ok(User::from(user))
    }

    pub async fn update(&self, user: &UpdateUser) -> Result<User, sqlx::Error> {
        let user = self.user_repository.update(user).await?;
        Ok(User::from(user))
    }
}
