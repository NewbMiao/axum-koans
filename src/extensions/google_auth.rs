use std::{collections::HashMap, sync::Arc};

use hyper::{header::CONTENT_TYPE, http::HeaderValue};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, url::Url, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RevocationUrl,
    Scope, TokenResponse, TokenUrl,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tokio::sync::Mutex;
use tracing::error;

use crate::config::OauthClientConfig;

pub struct GoogleAuth {
    client: BasicClient,
    csrf_pkces: Arc<Mutex<HashMap<String, PkceCodeVerifier>>>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TokenInfo {
    pub refresh_token: String,
    pub access_token: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Userinfo {
    pub sub: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub picture: Option<String>,
}

impl GoogleAuth {
    pub fn new(config: OauthClientConfig) -> Self {
        let auth_url =
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap();
        let token_url =
            TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string()).unwrap();
        let revocation_url =
            RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string()).unwrap();
        let redirect_url = RedirectUrl::new(config.redirect_url).unwrap();
        let client = BasicClient::new(
            ClientId::new(config.client_id),
            Some(ClientSecret::new(config.client_secret)),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(redirect_url)
        .set_revocation_uri(revocation_url);

        Self {
            client,
            csrf_pkces: Arc::new(Mutex::new(HashMap::default())),
        }
    }
    pub async fn auth_url(&self) -> Url {
        let pkce_challenge = PkceCodeChallenge::new_random_sha256();
        let (url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.profile".to_string(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/plus.business.manage".to_string(),
            ))
            .add_extra_param("prompt", "consent")
            .add_extra_param("access_type", "offline")
            .set_pkce_challenge(pkce_challenge.0.clone())
            .url();
        let csrf_token_key = csrf_token.secret().to_string();
        self.csrf_pkces
            .lock()
            .await
            .entry(csrf_token_key)
            .or_insert(pkce_challenge.1);
        url
    }
    pub async fn get_tokens(
        &self,
        code: AuthorizationCode,
        csrf_token: CsrfToken,
    ) -> Option<TokenInfo> {
        let mut hmap = self.csrf_pkces.clone().lock_owned().await;
        let mut res = self.client.exchange_code(code);

        if let Some(pkce_verifier) = hmap.remove(&csrf_token.secret().to_string()) {
            res = res.set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier.secret().to_string()))
        }

        match res.request_async(async_http_client).await {
            Ok(res) => {
                return Some(TokenInfo {
                    refresh_token: res.refresh_token().unwrap().secret().to_string(),
                    access_token: res.access_token().secret().to_string(),
                });
            }
            Err(err) => error!("got error in callback: {}", err),
        }
        None
    }
    pub async fn get_user_info(&self, token: String) -> Userinfo {
        let user_info_url = "https://www.googleapis.com/oauth2/v3/userinfo";
        let res = Client::new()
            .get(user_info_url)
            .bearer_auth(token)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .send()
            .await
            .unwrap();
        let text = res.text().await.unwrap();
        from_str(&text).unwrap()
    }
}
