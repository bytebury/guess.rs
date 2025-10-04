#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub lookup_id: String,
    pub display_name: String,
}

pub struct NewUser {
    pub lookup_id: String,
}
impl NewUser {
    pub fn new() -> Self {
        Self {
            lookup_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

pub struct UpdateUser {
    pub id: i64,
    pub lookup_id: String,
    pub display_name: String,
}
impl From<&User> for UpdateUser {
    fn from(value: &User) -> Self {
        Self {
            id: value.id,
            lookup_id: value.lookup_id.clone(),
            display_name: value.display_name.clone(),
        }
    }
}
