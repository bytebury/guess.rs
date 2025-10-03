use crate::SharedState;
use axum::{Router, extract::State, routing::put};

pub fn routes() -> Router<SharedState> {
    Router::new().route("/breakout", put(create_breakout))
}

async fn create_breakout(State(_state): State<SharedState>) {
    todo!();
}
