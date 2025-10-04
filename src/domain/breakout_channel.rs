use askama::Template;
use std::collections::HashMap;
use tokio::sync::broadcast;

use crate::domain::user::User;

#[derive(Template)]
#[template(path = "breakout_voters.html")]
pub struct VotersTemplate<'a> {
    breakout: &'a BreakoutChannel,
    users: Vec<&'a User>,
}

#[derive(Clone)]
pub struct BreakoutChannel {
    pub tx: broadcast::Sender<String>,
    pub lookup_id: String,
    pub users: Vec<User>,
    pub show_votes: bool,
}
impl BreakoutChannel {
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

    pub fn toggle_votes(&mut self) {
        self.show_votes = !self.show_votes;

        if !self.show_votes {
            self.users.iter_mut().for_each(|u| u.vote = None);
            let _ = self
                .tx
                .send(BreakoutChannel::send("enable_voting", "start voting"));
        } else {
            let _ = self
                .tx
                .send(BreakoutChannel::send("disable_voting", "votes are in"));
        }

        let _ = self.tx.send(Self::voters_html(self));
    }

    pub fn vote(&mut self, user: &User, value: Option<i64>) {
        if let Some(update_user) = self
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
        let _ = self.tx.send(Self::voters_html(self));
    }

    pub fn user_changed_name(&mut self, user: &User) {
        Self::remove_user(self, &user.lookup_id);
        Self::add_user(self, &user);
        let _ = self.tx.send(Self::voters_html(self));
    }

    pub fn add_user(&mut self, user: &User) {
        if !self.users.iter().any(|u| u.lookup_id == user.lookup_id) {
            self.users.push(user.clone());
        }
        let _ = self.tx.send(Self::voters_html(self));
    }

    pub fn remove_user(&mut self, user_lookup_id: &str) {
        self.users.retain(|u| u.lookup_id != user_lookup_id);
        let _ = self.tx.send(Self::voters_html(self));
    }

    pub fn is_empty(&self) -> bool {
        self.users.is_empty()
    }

    fn send(name: &str, data: &str) -> String {
        format!("event: {}\ndata: {}\n\n", name, data)
    }

    fn voters_html(&self) -> String {
        let mut user_refs: Vec<&User> = self.users.iter().collect();

        user_refs.sort_by(|a, b| {
            a.display_name
                .to_lowercase()
                .cmp(&b.display_name.to_lowercase())
        });

        VotersTemplate {
            breakout: self,
            users: user_refs,
        }
        .render()
        .unwrap()
    }
}
