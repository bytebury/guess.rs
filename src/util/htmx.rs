use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

pub struct HTMX {}
impl HTMX {
    pub fn redirect(url: &str) -> impl IntoResponse {
        let mut headers = HeaderMap::new();
        headers.insert("HX-Redirect", url.parse().unwrap());
        (StatusCode::OK, headers, "").into_response()
    }

    pub fn trigger(action: &str) -> impl IntoResponse {
        let mut headers = HeaderMap::new();
        headers.insert("HX-Trigger", action.parse().unwrap());
        (StatusCode::OK, headers, "").into_response()
    }

    pub fn refresh() -> impl IntoResponse {
        let mut headers = HeaderMap::new();
        headers.insert("HX-Refresh", "true".parse().unwrap());
        (StatusCode::OK, headers, "").into_response()
    }
}
