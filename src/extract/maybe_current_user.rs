use crate::{AppState, domain::User, extract::BaseUser};
use axum::response::{IntoResponse, Redirect, Response};
use axum::{extract::FromRequestParts, http::request::Parts};
use std::sync::Arc;

pub struct MaybeCurrentUser(pub Option<Box<User>>);

impl FromRequestParts<Arc<AppState>> for MaybeCurrentUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let user = BaseUser::from_request_parts(parts, state)
            .await
            .map_err(|_| Redirect::to("/").into_response())?;

        let user = match user {
            BaseUser::User(user) => Some(user),
            _ => None,
        };

        Ok(MaybeCurrentUser(user))
    }
}
