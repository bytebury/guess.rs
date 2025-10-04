use crate::{
    SharedState,
    domain::{
        breakout::{Breakout, NewBreakout},
        breakout_channel::BreakoutChannel,
        user::UpdateUser,
    },
    extract::{breakout::BreakoutRoom, breakout_user::BreakoutUser},
    routes::SharedContext,
    util::htmx::HTMX,
};
use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Form, Router,
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Redirect, Sse},
    routing::{get, patch, put},
};
use axum::{response::sse::Event, routing::post};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use reqwest::StatusCode;
use serde::Deserialize;
use time::Duration;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/breakout", put(create_breakout))
        .route("/breakout/{lookup_id}", get(breakout))
        .route("/breakout/{lookup_id}/leave", post(leave_breakout))
        .route("/breakout/{lookup_id}/sse", get(breakout_sse))
        .route("/breakout/{lookup_id}/user", patch(update_user))
        .route("/breakout/{lookup_id}/user", get(user_form))
        .route("/breakout/{lookup_id}/vote", post(vote))
        .route("/breakout/{lookup_id}/toggle-votes", post(toggle_votes))
}

#[derive(Template, WebTemplate)]
#[template(path = "breakout.html")]
struct BreakoutTemplate {
    shared: SharedContext,
    breakout: Breakout,
}
impl BreakoutTemplate {
    pub fn new(shared: SharedContext, breakout: Breakout) -> Self {
        Self { shared, breakout }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "update_user.html")]
struct UpdateUserTemplate {
    breakout: Breakout,
}

#[derive(Deserialize)]
struct UpdateUserForm {
    display_name: String,
}

#[derive(Deserialize)]
struct VoteQuery {
    vote: Option<i64>,
}

async fn user_form(
    BreakoutRoom(breakout): BreakoutRoom,
    BreakoutUser(_): BreakoutUser,
) -> UpdateUserTemplate {
    UpdateUserTemplate { breakout }
}

async fn toggle_votes(
    State(state): State<SharedState>,
    Path(lookup_id): Path<String>,
    BreakoutUser(_): BreakoutUser,
    BreakoutRoom(_): BreakoutRoom,
) {
    let mut channels = state.breakout_channels.lock().await;
    BreakoutChannel::find_or_create(&mut channels, &lookup_id).toggle_votes();
}

async fn vote(
    State(state): State<SharedState>,
    Path(lookup_id): Path<String>,
    BreakoutUser(user): BreakoutUser,
    BreakoutRoom(_): BreakoutRoom,
    Query(params): Query<VoteQuery>,
) {
    let mut channels = state.breakout_channels.lock().await;
    BreakoutChannel::find_or_create(&mut channels, &lookup_id).vote(&user, params.vote);
}

async fn update_user(
    State(state): State<SharedState>,
    Path(lookup_id): Path<String>,
    BreakoutUser(user): BreakoutUser,
    BreakoutRoom(_): BreakoutRoom,
    cookies: CookieJar,
    Form(form): Form<UpdateUserForm>,
) -> impl IntoResponse {
    let mut user = UpdateUser::from(&user);
    let mut channels = state.breakout_channels.lock().await;

    let channel = BreakoutChannel::find_or_create(&mut channels, &lookup_id);
    user.display_name = form.display_name;

    let user = match state.user_service.update(&user).await {
        Ok(user) => user,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    channel.user_changed_name(&user);

    let display_name_cookie = Cookie::build(("guess_rs_display_name", user.display_name.clone()))
        .path("/")
        .same_site(SameSite::None)
        .max_age(Duration::days(365));
    let cookies = cookies.add(display_name_cookie);

    let mut headers = HeaderMap::new();
    headers.insert("HX-Trigger", "closeModal".parse().unwrap());

    (StatusCode::OK, headers, cookies).into_response()
}

async fn breakout(
    State(state): State<SharedState>,
    Path(lookup_id): Path<String>,
    BreakoutUser(user): BreakoutUser,
    BreakoutRoom(_): BreakoutRoom,
    cookies: CookieJar,
) -> impl IntoResponse {
    let whoami_cookie = Cookie::build(("whoami", user.lookup_id.clone()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::None)
        .max_age(Duration::days(365))
        .secure(true);
    let display_name_cookie = Cookie::build(("guess_rs_display_name", user.display_name.clone()))
        .path("/")
        .same_site(SameSite::None)
        .max_age(Duration::days(365));
    let cookies = cookies.add(display_name_cookie);
    let cookies = cookies.add(whoami_cookie);

    match state.breakout_service.find_by_lookup_id(lookup_id).await {
        Ok(breakout) => (
            cookies,
            BreakoutTemplate::new(SharedContext::new(&state.app_info, Some(user)), breakout),
        )
            .into_response(),
        Err(_) => Redirect::to("/").into_response(),
    }
}

async fn breakout_sse(
    State(state): State<SharedState>,
    Path(lookup_id): Path<String>,
    BreakoutUser(user): BreakoutUser,
    BreakoutRoom(_): BreakoutRoom,
) -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    let mut channels = state.breakout_channels.lock().await;
    let channel = BreakoutChannel::find_or_create(&mut channels, &lookup_id);
    let tx = &channel.tx;
    let rx = tx.subscribe();

    channel.add_user(&user);

    let stream = BroadcastStream::new(rx).filter_map(|msg| match msg {
        Ok(msg) => Some(Ok(Event::default().data(msg))),
        Err(_) => None,
    });
    Sse::new(stream)
}

async fn create_breakout(State(state): State<SharedState>) -> impl IntoResponse {
    let breakout = NewBreakout::new();
    match state.breakout_service.create(&breakout).await {
        Ok(breakout) => {
            HTMX::redirect(&format!("/breakout/{}", breakout.lookup_id)).into_response()
        }
        Err(_) => HTMX::refresh().into_response(),
    }
}

async fn leave_breakout(
    State(state): State<SharedState>,
    Path(lookup_id): Path<String>,
    BreakoutUser(user): BreakoutUser,
    BreakoutRoom(_): BreakoutRoom,
) {
    let mut channels = state.breakout_channels.lock().await;
    let channel = BreakoutChannel::find_or_create(&mut channels, &lookup_id);

    channel.remove_user(&user.lookup_id);

    if channel.is_empty() {
        channels.remove(&lookup_id);
    }
}
