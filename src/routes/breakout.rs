use crate::{
    SharedState,
    domain::{
        breakout::{Breakout, NewBreakout},
        breakout_channel::BreakoutChannel,
        user::{UpdateUser, User},
    },
    extract::{breakout::BreakoutRoom, breakout_user::BreakoutUser},
    routes::SharedContext,
    util::htmx::HTMX,
};
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::ws::Message;
use axum::{
    Form, Router,
    extract::{Path, State, WebSocketUpgrade, ws::WebSocket},
    http::HeaderMap,
    response::{IntoResponse, Redirect},
    routing::{get, patch, put},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use reqwest::StatusCode;
use serde::Deserialize;
use time::Duration;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/breakout", put(create_breakout))
        .route("/breakout/{lookup_id}", get(breakout))
        .route("/breakout/{lookup_id}/ws", get(breakout_ws))
        .route("/breakout/{lookup_id}/user", patch(update_user))
        .route("/breakout/{lookup_id}/user", get(user_form))
}

#[derive(Deserialize)]
struct ClientMessage {
    action: String,
    vote: Option<String>,
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

async fn user_form(
    BreakoutRoom(breakout): BreakoutRoom,
    BreakoutUser(_): BreakoutUser,
) -> UpdateUserTemplate {
    UpdateUserTemplate { breakout }
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

async fn breakout_ws(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
    Path(_): Path<String>,
    BreakoutUser(user): BreakoutUser,
    BreakoutRoom(breakout): BreakoutRoom,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, user, breakout))
}

async fn handle_socket(socket: WebSocket, state: SharedState, user: User, breakout: Breakout) {
    let tx = {
        let mut channels = state.breakout_channels.lock().await;
        let channel = BreakoutChannel::find_or_create(&mut channels, &breakout.lookup_id);
        channel.add_user(&user);
        channel.tx.clone()
    };

    let mut rx = tx.subscribe();
    let (mut sender, mut receiver) = socket.split();

    {
        let channels = state.breakout_channels.lock().await;
        if let Some(channel) = channels.get(&breakout.lookup_id) {
            let _ = sender
                .send(Message::Text(channel.voters_html().into()))
                .await;
        }
    }

    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(msg.into()).await.is_err() {
                break;
            }
        }
    });

    let state_clone = state.clone();
    let user_clone = user.clone();
    let breakout_clone = breakout.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(event) = serde_json::from_str::<ClientMessage>(&text) {
                        let mut channels = state_clone.breakout_channels.lock().await;
                        let channel = BreakoutChannel::find_or_create(
                            &mut channels,
                            &breakout_clone.lookup_id,
                        );
                        handle_event(&event, &user_clone, channel);
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    let mut channels = state.breakout_channels.lock().await;
    if let Some(channel) = channels.get_mut(&breakout.lookup_id) {
        channel.remove_user(&user.lookup_id);
    }
}

async fn create_breakout(State(state): State<SharedState>) -> impl IntoResponse {
    let breakout = NewBreakout::default();
    match state.breakout_service.create(&breakout).await {
        Ok(breakout) => {
            HTMX::redirect(&format!("/breakout/{}", breakout.lookup_id)).into_response()
        }
        Err(_) => HTMX::refresh().into_response(),
    }
}

fn handle_event(event: &ClientMessage, user: &User, channel: &mut BreakoutChannel) {
    match event.action.as_str() {
        "toggle_votes" => channel.toggle_votes(),
        "vote" => channel.vote(&user.lookup_id, &event.vote),
        _ => {}
    }
}
