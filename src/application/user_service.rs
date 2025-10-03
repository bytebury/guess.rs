use std::sync::Arc;

use sqlx::SqlitePool;

use crate::{
    domain::{
        User,
        user::{AuditUser, NewUser, UpdateUser},
    },
    infrastructure::db::UserRepository,
    util::pagination::{PaginatedResponse, Pagination},
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

    pub async fn find_by_id(&self, user_id: i64) -> Result<AuditUser, sqlx::Error> {
        self.user_repository.find_by_id(user_id).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        self.user_repository.find_by_email(email).await
    }

    pub async fn update(&self, user: &UpdateUser) -> Result<AuditUser, sqlx::Error> {
        self.user_repository.update(user).await
    }

    pub async fn search(
        &self,
        pagination: &Pagination,
        search: &str,
    ) -> PaginatedResponse<AuditUser> {
        self.user_repository.search(pagination, search).await
    }

    pub async fn create(&self, user: &NewUser) -> Result<User, sqlx::Error> {
        self.user_repository.create(user).await
    }
}
