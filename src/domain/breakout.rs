use crate::{BreakoutChannel, domain::user::User};
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
            })
    }

    pub fn vote(breakout: &mut BreakoutChannel, user: &User, value: i64) {
        todo!("Indicate that the user has voted. Should add it to the user.");
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

    fn voters_html(breakout: &BreakoutChannel) -> String {
        let mut users = breakout.users.clone();
        users.sort_by(|a, b| {
            a.display_name
                .to_lowercase()
                .cmp(&b.display_name.to_lowercase())
        });
        users
            .iter()
            .map(|u| format!(r#"<li id="user-{}">{}</li>"#, u.lookup_id, u.display_name))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
