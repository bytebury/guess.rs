use crate::SharedState;
use crate::routes::SharedContext;

use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, extract::State, routing::get};

pub fn routes() -> Router<SharedState> {
    Router::new().route("/", get(homepage))
}

#[derive(Template, WebTemplate)]
#[template(path = "homepage.html")]
struct HomepageTemplate {
    shared: SharedContext,
}

async fn homepage(State(state): State<SharedState>) -> HomepageTemplate {
    HomepageTemplate {
        shared: SharedContext::new(&state.app_info),
    }
}
