use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow, Clone)]
pub struct Country {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub locked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(FromRow, Clone)]
pub struct CountryRegion {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct CountryWithRegion {
    pub country: Country,
    pub region: CountryRegion,
}
