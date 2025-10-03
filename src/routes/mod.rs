use crate::{AppInfo, domain::User};

pub mod admin;
pub mod auth;
pub mod homepage;
pub mod payments;
pub mod webhooks;

pub struct SharedContext {
    pub app_info: AppInfo,
    pub current_user: Option<User>,
}
impl SharedContext {
    pub fn new(app_info: &AppInfo, user: Option<User>) -> Self {
        Self {
            app_info: app_info.clone(),
            current_user: user,
        }
    }
}
