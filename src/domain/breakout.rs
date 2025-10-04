use crate::{BreakoutChannel, domain::user::User};
use askama::Template;
use std::collections::HashMap;
use tokio::sync::broadcast;

pub struct NewBreakout {
    pub lookup_id: String,
}
impl NewBreakout {
    pub fn new() -> Self {
        Self {
            lookup_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

#[derive(Template)]
#[template(path = "breakout_voters.html")]
pub struct VotersTemplate<'a> {
    breakout: &'a BreakoutChannel,
    users: Vec<&'a User>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Breakout {
    pub id: i64,
    pub lookup_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
impl Breakout {
    pub fn find_or_create<'a>(
        channels: &'a mut HashMap<String, BreakoutChannel>,
        lookup_id: &str,
    ) -> &'a mut BreakoutChannel {
        channels
            .entry(lookup_id.to_string())
            .or_insert_with(|| BreakoutChannel {
                tx: broadcast::channel(100).0,
                users: vec![],
                show_votes: false,
                lookup_id: lookup_id.to_string(),
            })
    }

    pub fn toggle_votes(breakout: &mut BreakoutChannel) {
        breakout.show_votes = !breakout.show_votes;

        if !breakout.show_votes {
            breakout.users.iter_mut().for_each(|u| u.vote = None);
            let _ = breakout
                .tx
                .send(Breakout::sse_event("enable_voting", "start voting"));
        } else {
            let _ = breakout
                .tx
                .send(Breakout::sse_event("disable_voting", "votes are in"));
        }

        let _ = breakout.tx.send(Self::voters_html(breakout));
    }

    pub fn vote(breakout: &mut BreakoutChannel, user: &User, value: Option<i64>) {
        if let Some(update_user) = breakout
            .users
            .iter_mut()
            .find(|u| u.lookup_id == user.lookup_id)
        {
            if update_user.vote == value {
                update_user.vote = None;
            } else {
                update_user.vote = value;
            }
        }
        let _ = breakout.tx.send(Self::voters_html(breakout));
    }

    pub fn user_changed_name(breakout: &mut BreakoutChannel, user: &User) {
        Self::remove_user(breakout, &user.lookup_id);
        Self::add_user(breakout, &user);
        let _ = breakout.tx.send(Self::voters_html(breakout));
    }

    pub fn add_user(breakout: &mut BreakoutChannel, user: &User) {
        if !breakout.users.iter().any(|u| u.lookup_id == user.lookup_id) {
            breakout.users.push(user.clone());
        }
        let _ = breakout.tx.send(Self::voters_html(breakout));
    }

    pub fn remove_user(breakout: &mut BreakoutChannel, user_lookup_id: &str) {
        breakout.users.retain(|u| u.lookup_id != user_lookup_id);
        let _ = breakout.tx.send(Self::voters_html(breakout));
    }

    pub fn is_empty(breakout: &mut BreakoutChannel) -> bool {
        breakout.users.is_empty()
    }

    fn sse_event(name: &str, data: &str) -> String {
        format!("event: {}\ndata: {}\n\n", name, data)
    }

    fn voters_html(breakout: &BreakoutChannel) -> String {
        let mut user_refs: Vec<&User> = breakout.users.iter().collect();

        user_refs.sort_by(|a, b| {
            a.display_name
                .to_lowercase()
                .cmp(&b.display_name.to_lowercase())
        });

        VotersTemplate {
            breakout,
            users: user_refs,
        }
        .render()
        .unwrap()
    }
}
