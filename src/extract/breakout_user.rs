use axum::response::IntoResponse;
use axum::response::Response;
use axum::{extract::FromRequestParts, http::request::Parts, response::Redirect};

use crate::{SharedState, domain::user::User, extract::BaseUser};

#[derive(Clone, PartialEq, Eq)]
pub struct BreakoutUser(pub User);

impl FromRequestParts<SharedState> for BreakoutUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        // TODO: if a person doesn't exist, we need to create them,
        // or at the very least handle gracefully.
        let user = BaseUser::from_request_parts(parts, state)
            .await
            .map_err(|_| Redirect::to("/").into_response())?;

        let user = match user {
            BaseUser::User(user) => BreakoutUser(user),
            _ => return Err(Redirect::to("/").into_response()),
        };

        Ok(user)
    }
}
