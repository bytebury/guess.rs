use crate::domain::User;
use crate::{AppState, extract::BaseUser};
use axum::response::{IntoResponse, Redirect, Response};
use axum::{extract::FromRequestParts, http::request::Parts};
use std::sync::Arc;

pub struct CurrentUser(pub Box<User>);

impl FromRequestParts<Arc<AppState>> for CurrentUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let user = BaseUser::from_request_parts(parts, state)
            .await
            .map_err(|_| Redirect::to("/").into_response())?;

        match user {
            BaseUser::User(user) => Ok(CurrentUser(user)),
            _ => Err(Redirect::to("/").into_response()),
        }
    }
}
