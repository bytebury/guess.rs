use crate::domain::rbac::Role;
use chrono::NaiveDateTime;
use sqlx::FromRow;

pub struct UpdateUser {
    pub id: i64,
    pub locked: bool,
    pub role: Role,
}

pub struct NewUser {
    pub id: i64,
    pub email: String,
    pub verified: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub full_name: String,
    pub image_url: String,
    pub country_id: Option<i64>,
    pub region_id: Option<i64>,
    pub locked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub verified: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub full_name: String,
    pub image_url: String,
    pub role: Role,
    pub stripe_customer_id: Option<String>,
    pub country_id: Option<i64>,
    pub region_id: Option<i64>,
    pub locked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
impl User {
    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }
}
