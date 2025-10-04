use crate::{AppInfo, domain::user::User};

pub mod breakout;
pub mod homepage;

pub struct SharedContext {
    pub app_info: AppInfo,
    pub user: Option<User>,
}
impl SharedContext {
    pub fn new(app_info: &AppInfo, user: Option<User>) -> Self {
        Self {
            app_info: app_info.clone(),
            user,
        }
    }
}
