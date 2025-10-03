use crate::domain::rbac::Role;
use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, extract::State, routing::get};
use std::sync::Arc;

use crate::{AppState, routes::SharedContext};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(homepage))
}

#[derive(Template, WebTemplate)]
#[template(path = "homepage.html")]
struct HomepageTemplate {
    shared: SharedContext,
}

async fn homepage(State(state): State<Arc<AppState>>) -> HomepageTemplate {
    HomepageTemplate {
        shared: SharedContext::new(&state.app_info, None),
    }
}
