use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::User;

#[derive(Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: String,
    pub exp: usize,
}

impl From<User> for UserClaims {
    fn from(user: User) -> Self {
        let exp = Utc::now()
            .checked_add_signed(Duration::days(1))
            .expect("valid timestamp")
            .timestamp() as usize;

        UserClaims {
            sub: user.email,
            exp,
        }
    }
}
