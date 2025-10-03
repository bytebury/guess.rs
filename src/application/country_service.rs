use std::sync::Arc;

use sqlx::SqlitePool;

use crate::{
    domain::{Country, country::CountryWithRegion},
    infrastructure::{audit::geolocation::CountryDetails, db::CountryRepository},
};

pub struct CountryService {
    country_repository: CountryRepository,
}
impl CountryService {
    pub fn new(db: &Arc<SqlitePool>) -> Self {
        Self {
            country_repository: CountryRepository::new(db),
        }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Country, sqlx::Error> {
        self.country_repository.find_by_id(id).await
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Country, sqlx::Error> {
        self.country_repository.find_by_name(name).await
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Country, sqlx::Error> {
        self.country_repository.find_by_code(code).await
    }

    pub async fn search(&self, value: &str) -> Vec<Country> {
        self.country_repository.search(value).await
    }

    pub async fn lock(&self, id: i64) -> Result<(), sqlx::Error> {
        self.country_repository.lock(id).await
    }

    pub async fn unlock(&self, id: i64) -> Result<(), sqlx::Error> {
        self.country_repository.unlock(id).await
    }

    pub async fn create_or_get(
        &self,
        country: &CountryDetails,
    ) -> Result<CountryWithRegion, sqlx::Error> {
        self.country_repository.create(country).await
    }
}
