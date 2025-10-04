use axum::{
    Router,
    http::{HeaderValue, header::CACHE_CONTROL},
};
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;
use std::{env, net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::{
    compression::CompressionLayer, services::ServeDir, set_header::SetResponseHeaderLayer,
};

use crate::{
    application::{BreakoutService, UserService},
    domain::breakout_channel::BreakoutChannel,
    infrastructure::db::Database,
};

pub mod application;
pub mod domain;
pub mod extract;
pub mod filter;
pub mod infrastructure;
pub mod routes;
pub mod util;

pub async fn start() {
    let app = initialize().await;
    let port = env::var("APP_PORT").unwrap_or_else(|_| "8080".to_string());

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn initialize() -> Router {
    let db = Arc::new(Database::initialize().await);
    let app_info = AppInfo::new();
    let state = Arc::new(AppState::new(&db, app_info.clone()));
    let serve_static = Router::new()
        .nest_service("/assets", ServeDir::new("public"))
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=31536000"),
        ));

    Router::new()
        .merge(serve_static)
        .merge(routes::homepage::routes())
        .merge(routes::breakout::routes())
        .with_state(state)
        .layer(CompressionLayer::new())
}

#[derive(Clone, Default)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub website_url: String,
}
impl AppInfo {
    pub fn new() -> Self {
        Self {
            name: env::var("APP_NAME").expect("APP_NAME not defined"),
            version: env::var("APP_VERSION").unwrap_or("local".to_string()),
            website_url: env::var("APP_WEBSITE_URL").expect("APP_WEBSITE_URL not defined"),
        }
    }
}

pub type BreakoutChannels = Arc<Mutex<HashMap<String, BreakoutChannel>>>;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub app_info: AppInfo,
    pub breakout_service: BreakoutService,
    pub user_service: UserService,
    pub breakout_channels: BreakoutChannels,
}
impl AppState {
    pub fn new(db: &Arc<Pool<Sqlite>>, app_info: AppInfo) -> Self {
        let breakout_channels: BreakoutChannels = Arc::new(Mutex::new(HashMap::new()));
        Self {
            app_info: app_info.clone(),
            breakout_service: BreakoutService::new(db),
            user_service: UserService::new(db),
            breakout_channels,
        }
    }
}
