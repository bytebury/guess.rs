use axum::extract::{ConnectInfo, FromRequestParts};
use axum::http::StatusCode;
use std::net::SocketAddr;
use std::sync::Arc;

pub struct RealIp(pub String);

impl<S> FromRequestParts<Arc<S>> for RealIp
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &Arc<S>,
    ) -> Result<Self, Self::Rejection> {
        // Get TCP peer address
        let connect_info = ConnectInfo::<SocketAddr>::from_request_parts(parts, _state)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Check for X-Forwarded-For
        let ip = if let Some(forwarded) = parts.headers.get("x-forwarded-for") {
            match forwarded.to_str() {
                Ok(forwarded) => forwarded.split(',').next().unwrap().trim().to_string(),
                Err(_) => connect_info.ip().to_string(),
            }
        } else {
            connect_info.ip().to_string()
        };

        Ok(RealIp(ip))
    }
}
