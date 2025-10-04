use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::response::Response;
use axum::{extract::FromRequestParts, http::request::Parts};

use crate::SharedState;
use crate::domain::breakout::Breakout;

pub struct BreakoutRoom(pub Breakout);

impl FromRequestParts<SharedState> for BreakoutRoom {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        use axum::extract::Path;

        let lookup_id = match Path::<String>::from_request_parts(parts, state).await {
            Ok(Path(id)) => id,
            Err(_) => return Err(Redirect::to("/").into_response()),
        };

        match state.breakout_service.find_by_lookup_id(lookup_id).await {
            Ok(breakout) => Ok(BreakoutRoom(breakout)),
            Err(_) => Err(Redirect::to("/").into_response()),
        }
    }
}
