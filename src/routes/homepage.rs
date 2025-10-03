use crate::domain::rbac::Role;
use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, extract::State, routing::get};
use std::sync::Arc;

use crate::{
    AppState,
    extract::{current_user::CurrentUser, no_user::NoUser},
    routes::SharedContext,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(homepage))
        .route("/dashboard", get(dashboard))
}

#[derive(Template, WebTemplate)]
#[template(path = "homepage.html")]
struct HomepageTemplate {
    shared: SharedContext,
}

#[derive(Template, WebTemplate)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    shared: SharedContext,
}

async fn homepage(State(state): State<Arc<AppState>>, NoUser: NoUser) -> HomepageTemplate {
    HomepageTemplate {
        shared: SharedContext::new(&state.app_info, None),
    }
}

async fn dashboard(
    State(state): State<Arc<AppState>>,
    CurrentUser(current_user): CurrentUser,
) -> DashboardTemplate {
    DashboardTemplate {
        shared: SharedContext::new(&state.app_info, Some(*current_user)),
    }
}
