use axum::http::StatusCode;

use crate::domain::user::NewUser;

pub mod google;

pub use google::GoogleUser;

pub trait OAuthProvider {
    fn get_auth_url(&self) -> String;
    fn exchange_code_for_user(
        &self,
        code: &str,
    ) -> impl Future<Output = Result<NewUser, StatusCode>>;
}
