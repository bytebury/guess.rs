pub struct NewBreakout {
    pub lookup_id: String,
}
impl Default for NewBreakout {
    fn default() -> Self {
        Self {
            lookup_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Breakout {
    pub id: i64,
    pub lookup_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
