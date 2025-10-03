use crate::AppInfo;

pub mod breakout;
pub mod homepage;

pub struct SharedContext {
    pub app_info: AppInfo,
}
impl SharedContext {
    pub fn new(app_info: &AppInfo) -> Self {
        Self {
            app_info: app_info.clone(),
        }
    }
}
