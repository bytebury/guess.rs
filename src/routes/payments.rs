use std::env;
use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::Redirect;
use axum::{Router, extract::State, routing::get};

use crate::{AppState, extract::current_user::CurrentUser};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/checkout", get(checkout))
        .route("/manage-subscription", get(manage_subscription))
        .route("/payments/successful", get(payment_successful))
        .route("/payments/cancelled", get(payment_cancelled))
}

async fn payment_successful() {
    todo!();
}

async fn payment_cancelled() {
    todo!();
}

async fn checkout(
    State(state): State<Arc<AppState>>,
    CurrentUser(user): CurrentUser,
) -> Result<Redirect, (StatusCode, String)> {
    let price_id = env::var("STRIPE_PRICE_ID").expect("STRIPE_PRICE_ID must be set");
    let session = state
        .stripe
        .checkout(&user, &price_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let url = session.url.ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Missing checkout URL".to_string(),
    ))?;

    Ok(Redirect::to(&url))
}

async fn manage_subscription(
    State(state): State<Arc<AppState>>,
    CurrentUser(user): CurrentUser,
) -> Result<Redirect, (StatusCode, String)> {
    let session = state
        .stripe
        .manage_subscription(&user)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Redirect::to(&session.url))
}
