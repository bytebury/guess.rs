use crate::{domain::rbac::Action, util::rbac::Can};
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

use crate::{domain::rbac::Role, infrastructure::auth::GoogleUser, util::pagination::Paginatable};

pub struct UpdateUser {
    pub id: i64,
    pub locked: bool,
    pub role: Role,
}
impl From<AuditUser> for UpdateUser {
    fn from(user: AuditUser) -> Self {
        Self {
            id: user.id,
            locked: user.locked,
            role: user.role,
        }
    }
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
impl From<GoogleUser> for NewUser {
    fn from(google_user: GoogleUser) -> Self {
        Self {
            id: 0,
            email: google_user.email,
            verified: google_user.email_verified,
            first_name: google_user.given_name.unwrap_or(google_user.name.clone()),
            last_name: google_user.family_name,
            full_name: google_user.name,
            image_url: google_user.picture,
            country_id: None,
            region_id: None,
            locked: false,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
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

impl Can<AuditUser> for User {
    fn can(&self, _: Action, _: &AuditUser) -> bool {
        matches!(self.role, Role::Admin)
    }
}

#[derive(Serialize, FromRow, Clone)]
pub struct AuditUser {
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
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub country_locked: bool,
    pub region_id: Option<i64>,
    pub country_region: Option<String>,
    pub locked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Paginatable for AuditUser {
    fn count_query() -> &'static str {
        r#"SELECT COUNT(*) FROM audit_users"#
    }

    fn page_query() -> &'static str {
        r#"SELECT * FROM audit_users"#
    }
}
