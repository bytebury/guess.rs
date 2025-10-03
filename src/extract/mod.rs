use crate::{
    AppState,
    domain::User,
    infrastructure::jwt::{JwtService, user_claims::UserClaims},
};
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::extract::CookieJar;
use std::sync::Arc;

pub mod admin_user;
pub mod current_user;
pub mod maybe_current_user;
pub mod no_user;
pub mod real_ip;

#[derive(Clone)]
pub enum BaseUser {
    User(Box<User>),
    None,
}

impl FromRequestParts<Arc<AppState>> for BaseUser {
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, axum::http::StatusCode> {
        let state = Arc::from_ref(state);
        let jar = CookieJar::from_headers(&parts.headers);

        let token = match jar.get("auth_token") {
            Some(cookie) => cookie.value(),
            None => return Ok(BaseUser::None),
        };

        // Check to see if they are a user first
        if let Ok(token_data) = JwtService::verify::<UserClaims>(token) {
            let user = state
                .user_service
                .find_by_email(&token_data.claims.sub)
                .await
                .ok()
                .flatten();

            if let Some(user) = user {
                return Ok(BaseUser::User(Box::new(user)));
            }
        }

        Ok(BaseUser::None)
    }
}
