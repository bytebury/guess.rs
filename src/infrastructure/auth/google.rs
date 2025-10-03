use axum::http::StatusCode;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    RedirectUrl, RevocationUrl, Scope, TokenResponse, TokenUrl, basic::BasicClient,
};
use reqwest::Client;
use serde::Deserialize;
use std::env;

use crate::{domain::user::NewUser, infrastructure::auth::OAuthProvider};

#[derive(Deserialize)]
pub struct GoogleUser {
    pub sub: String,
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub picture: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub locale: Option<String>,
}

#[derive(Default)]
pub struct GoogleOAuth {}
impl GoogleOAuth {
    pub fn new() -> Self {
        GoogleOAuth {}
    }

    fn client(
        &self,
    ) -> BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointSet, EndpointSet> {
        let client_id = ClientId::new(
            env::var("GOOGLE_CLIENT_ID")
                .expect("Missing the GOOGLE_CLIENT_ID environment variables."),
        );
        let client_secret = ClientSecret::new(
            env::var("GOOGLE_CLIENT_SECRET")
                .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
        );
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid auth endpoint URL.");
        let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
            .expect("Invalid token endpoint URL.");
        let redirect_url = RedirectUrl::new(env::var("GOOGLE_CALLBACK_URL").unwrap().to_string())
            .expect("Invalid redirect URL");
        let revocation_url = RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
            .expect("Invalid revocation endpoint URL");

        BasicClient::new(client_id)
            .set_client_secret(client_secret)
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url)
            .set_revocation_url(revocation_url)
    }

    async fn fetch_google_user_info(&self, token: &str) -> Result<GoogleUser, reqwest::Error> {
        let client = Client::new();
        let google_user = client
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .bearer_auth(token)
            .send()
            .await?
            .error_for_status()?
            .json::<GoogleUser>()
            .await?;

        Ok(google_user)
    }
}

impl OAuthProvider for GoogleOAuth {
    fn get_auth_url(&self) -> String {
        let (authorize_url, _csrf_state) = self
            .client()
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.profile".to_string(),
            ))
            .url();
        authorize_url.as_str().to_string()
    }

    async fn exchange_code_for_user(&self, code: &str) -> Result<NewUser, StatusCode> {
        let http_client = oauth2::reqwest::ClientBuilder::new()
            .redirect(oauth2::reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let token_result = self
            .client()
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(&http_client)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let access_token = token_result.access_token().secret().clone();

        let google_user = self
            .fetch_google_user_info(&access_token)
            .await
            .map_err(|_| StatusCode::BAD_GATEWAY)?;

        Ok(NewUser::from(google_user))
    }
}
