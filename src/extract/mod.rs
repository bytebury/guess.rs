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

        if let Some(cookie) = jar.get("whoami")
            && let Ok(user) = state
                .user_service
                .find_by_lookup_id(cookie.value())
                .await
        {
            return Ok(BaseUser::User(user));
        }

        // No cookie, or the cookie points to a user that no longer exists
        // (e.g. after a local DB reset). Mint a fresh user so the caller
        // doesn't bounce the request to "/".
        match state.user_service.create(&NewUser::default()).await {
            Ok(user) => Ok(BaseUser::User(user)),
            Err(_) => Ok(BaseUser::None),
        }
    }
}
