use std::{collections::HashMap, sync::Arc};

use crate::{config::OauthClientConfig, errors::ServerError};
use hyper::{header::CONTENT_TYPE, http::HeaderValue};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, url::Url, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tokio::sync::Mutex;
use tracing::warn;

use super::KeyCloakIdp;

pub struct KeycloakAuth {
    config: OauthClientConfig,
    client: BasicClient,

    csrf_pkces: Arc<Mutex<HashMap<String, PkceCodeVerifier>>>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TokenInfo {
    pub refresh_token: String,
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenExchangeResponse {
    pub access_token: String,
    pub expires_in: u64,
    // account_link_url: String,
    // issued_token_type: String,
    // not_before_policy: u64,
    // refresh_expires_in: u64,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct UserInfo {
    sub: String,
    email_verified: bool,
    name: String,
    preferred_username: String,
    given_name: String,
    family_name: String,
    email: String,
}

impl KeycloakAuth {
    pub fn new(config: OauthClientConfig) -> Self {
        let config_clone = config.clone();
        let client_id = ClientId::new(config.client_id);
        let client_secret = ClientSecret::new(config.client_secret);

        let token_url = TokenUrl::new(get_url_with_issuer(
            &config.issuer_url,
            "/protocol/openid-connect/token",
        ))
        .unwrap();
        let auth_url = AuthUrl::new(get_url_with_issuer(
            &config.issuer_url,
            "/protocol/openid-connect/auth",
        ))
        .unwrap();
        let redirect_url = RedirectUrl::new(config.redirect_url).unwrap();
        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(redirect_url);

        Self {
            config: config_clone,
            client,
            csrf_pkces: Arc::new(Mutex::new(HashMap::default())),
        }
    }
    pub async fn auth_url(&self, kc_idp_hint: KeyCloakIdp) -> Url {
        let pkce_challenge = PkceCodeChallenge::new_random_sha256();
        let (url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".to_string()))
            .add_extra_param("kc_idp_hint", kc_idp_hint.as_str()) // use google directly
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
    ) -> Result<TokenInfo, ServerError> {
        let mut hmap = self.csrf_pkces.clone().lock_owned().await;
        let mut res = self.client.exchange_code(code);

        if let Some(pkce_verifier) = hmap.remove(&csrf_token.secret().to_string()) {
            res = res.set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier.secret().to_string()))
        }

        let res = res.request_async(async_http_client).await?;
        Ok(TokenInfo {
            refresh_token: res.refresh_token().unwrap().secret().to_string(),
            access_token: res.access_token().secret().to_string(),
        })
    }
    pub async fn get_idp_token(
        &self,
        access_token: String,
        requested_issuer: &str,
    ) -> Result<TokenInfo, ServerError> {
        let token_url = get_url_with_issuer(
            &self.config.issuer_url,
            &format!("/broker/{requested_issuer}/token"),
        );
        let response = Client::new()
            .get(token_url)
            .bearer_auth(access_token)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .send()
            .await?;
        let res = response.text().await?;
        warn!("get broker token response: {}", res);
        Ok(from_str(&res)?)
    }
    pub async fn token_exchange(
        &self,
        access_token: String,
        requested_issuer: &str,
    ) -> Result<TokenExchangeResponse, ServerError> {
        let token_url =
            get_url_with_issuer(&self.config.issuer_url, "/protocol/openid-connect/token");
        let response = Client::new()
            .post(token_url)
            .form(&[
                (
                    "grant_type",
                    "urn:ietf:params:oauth:grant-type:token-exchange",
                ),
                ("subject_token", &access_token),
                ("client_id", &self.config.client_id),
                ("client_secret", &self.config.client_secret),
                (
                    "requested_token_type",
                    "urn:ietf:params:oauth:token-type:access_token",
                ),
                ("requested_issuer", requested_issuer),
            ])
            .send()
            .await?;
        Ok(from_str(&response.text().await?)?)
    }
    pub async fn get_user_info(&self, token: String) -> Result<UserInfo, ServerError> {
        let user_info_url =
            get_url_with_issuer(&self.config.issuer_url, "/protocol/openid-connect/userinfo");
        let res = Client::new()
            .get(user_info_url)
            .bearer_auth(token)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .send()
            .await?;

        Ok(from_str(&res.text().await?)?)
    }
}

fn get_url_with_issuer(issue_url: &str, path: &str) -> String {
    format!("{issue_url}{path}")
}
