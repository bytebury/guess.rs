use axum::{
    Router,
    extract::State,
    response::{IntoResponse, Redirect},
    routing::{delete, get},
};
use axum_extra::extract::{
    CookieJar, Query,
    cookie::{self, Cookie},
};
use log::error;
use reqwest::StatusCode;
use serde::Deserialize;
use std::{net::IpAddr, sync::Arc};

use crate::{
    AppState,
    extract::real_ip::RealIp,
    infrastructure::{
        audit,
        auth::{OAuthProvider, google::GoogleOAuth},
        jwt::{JwtService, user_claims::UserClaims},
    },
    util::htmx::HTMX,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/google", get(signin_with_google))
        .route("/auth/google/callback", get(google_callback))
        .route("/auth/signout", delete(signout))
}

#[derive(Debug, Deserialize)]
struct AuthRequest {
    code: String,
}

async fn signin_with_google() -> impl IntoResponse {
    Redirect::to(GoogleOAuth::default().get_auth_url().as_str())
}

async fn google_callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AuthRequest>,
    RealIp(ip): RealIp,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    let mut user = GoogleOAuth::default()
        .exchange_code_for_user(&params.code)
        .await?;

    let ip: IpAddr = ip.parse().unwrap();
    let country_details = audit::geolocation::get_country_details(ip).unwrap_or_default();

    if let Ok(location) = state.country_service.create_or_get(&country_details).await {
        user.country_id = Some(location.country.id);
        user.region_id = Some(location.region.id);
        user.locked = location.country.locked;
    }

    let user = match state.user_service.find_by_email(&user.email).await {
        Ok(Some(user)) => user,
        Ok(None) => state
            .user_service
            .create(&user)
            .await
            .inspect_err(|e| {
                error!(
                    "Something happened while creating user ({}): {e}",
                    user.email
                )
            })
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        Err(e) => {
            error!("Unable to find email({}): {e}", user.email);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let token = JwtService::generate(&UserClaims::from(user))
        .inspect_err(|e| error!("Unable to generate JWT: {e}"))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let auth_cookie = Cookie::build(("auth_token", token))
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::None)
        .secure(true);

    let cookies = cookies.add(auth_cookie);

    Ok((cookies, Redirect::to("/")))
}

async fn signout(State(_state): State<Arc<AppState>>, cookies: CookieJar) -> impl IntoResponse {
    let cookies = cookies.remove(
        Cookie::build(("auth_token", ""))
            .path("/")
            .http_only(true)
            .same_site(cookie::SameSite::Strict),
    );
    (cookies, HTMX::redirect("/")).into_response()
}
