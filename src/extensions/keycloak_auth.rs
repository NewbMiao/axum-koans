use std::{collections::HashMap, sync::Arc};

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

pub struct KeycloakAuth {
    client_secret: String,
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
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        let client_secret_string = client_secret.to_string();
        let client_id = ClientId::new(client_id.to_string());
        let client_secret = ClientSecret::new(client_secret.to_string());

        let token_url = TokenUrl::new(
            "http://localhost:8080/realms/axum-demo/protocol/openid-connect/token".to_string(),
        )
        .unwrap();
        let auth_url = AuthUrl::new(
            "http://localhost:8080/realms/axum-demo/protocol/openid-connect/auth".to_string(),
        )
        .unwrap();
        let redirect_url =
            RedirectUrl::new("http://localhost:8000/keycloak/login-callback".to_string()).unwrap();
        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(redirect_url);

        Self {
            client_secret: client_secret_string,
            client,
            csrf_pkces: Arc::new(Mutex::new(HashMap::default())),
        }
    }
    pub async fn auth_url(&self) -> Url {
        let pkce_challenge = PkceCodeChallenge::new_random_sha256();
        let (url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".to_string()))
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
        // println!("{:?}", hmap);
        let pkce_verifier = hmap.remove(&csrf_token.secret().to_string()).unwrap();
        let res = self
            .client
            .exchange_code(code)
            .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier.secret().to_string()))
            .request_async(async_http_client)
            .await;
        match res {
            Ok(res) => {
                return Some(TokenInfo {
                    refresh_token: res.refresh_token().unwrap().secret().to_string(),
                    access_token: res.access_token().secret().to_string(),
                });
            }
            Err(err) => eprintln!("got error in callback: {}", err),
        }
        None
    }
    pub async fn token_exchange(
        &self,
        access_token: String,
        requested_issuer: String,
    ) -> TokenExchangeResponse {
        // 从 Keycloak 的令牌响应中获取访问令牌
        let token_url = "http://localhost:8080/realms/axum-demo/protocol/openid-connect/token";
        let userinfo_response = Client::new()
            .post(token_url)
            .form(&[
                (
                    "grant_type",
                    "urn:ietf:params:oauth:grant-type:token-exchange",
                ),
                ("subject_token", &access_token),
                ("client_id", self.client.client_id()),
                ("client_secret", &self.client_secret),
                (
                    "requested_token_type",
                    "urn:ietf:params:oauth:token-type:access_token",
                ),
                ("requested_issuer", &requested_issuer),
            ])
            .send()
            .await
            .unwrap();
        let response_text = userinfo_response.text().await.unwrap();
        from_str(&response_text).unwrap()
    }
    pub async fn get_user_info(&self, token: String) -> UserInfo {
        let user_info_url =
            "http://localhost:8080/realms/axum-demo/protocol/openid-connect/userinfo";
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
