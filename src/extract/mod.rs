use axum::extract::FromRef;
use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
use std::sync::Arc;

use crate::domain::user::NewUser;
use crate::{SharedState, domain::user::User};

pub mod breakout;
pub mod breakout_user;

pub enum BaseUser {
    User(User),
    None,
}

impl FromRequestParts<SharedState> for BaseUser {
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let state = Arc::from_ref(state);
        let jar = CookieJar::from_headers(&parts.headers);

        let whoami = match jar.get("whoami") {
            Some(cookie) => cookie.value().to_string(),
            _ => {
                // User does not exist, so create one.
                match state.user_service.create(&NewUser::new()).await {
                    Ok(user) => user.lookup_id,
                    Err(_) => return Ok(BaseUser::None),
                }
            }
        };

        match state.user_service.find_by_lookup_id(&whoami).await {
            Ok(user) => Ok(BaseUser::User(user)),
            Err(_) => Ok(BaseUser::None),
        }
    }
}
