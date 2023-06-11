use std::env;

use dotenvy::dotenv;

use crate::errors::ServerError;

#[derive(Debug, Clone)]
pub struct OauthClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub issuer_url: String,
}
pub struct DatabaseConfig {
    pub url: String,
}
pub struct Config {
    pub google: OauthClientConfig,
    pub keycloak: OauthClientConfig,
    pub database: DatabaseConfig,
}
impl Config {
    fn get_env(key: &str) -> Result<String, ServerError> {
        env::var(key)
            .map_err(|err| ServerError::ConfigError(format!("Failed to retrieve {key}: {err}",)))
    }
    pub fn from_env() -> Result<Self, ServerError> {
        dotenv().ok();
        Ok(Config {
            google: OauthClientConfig {
                client_id: Self::get_env("GOOGLE_CLIENT_ID")?,
                client_secret: Self::get_env("GOOGLE_CLIENT_SECRET")?,
                redirect_url: Self::get_env("GOOGLE_REDIRECT_URL")?,
                issuer_url: "".into(),
            },
            keycloak: OauthClientConfig {
                client_id: Self::get_env("KEYCLOAK_CLIENT_ID")?,
                client_secret: Self::get_env("KEYCLOAK_CLIENT_SECRET")?,
                redirect_url: Self::get_env("KEYCLOAK_REDIRECT_URL")?,
                issuer_url: Self::get_env("KEYCLOAK_ISSUER")?,
            },
            database: DatabaseConfig {
                url: Self::get_env("DATABASE_URL")?,
            },
        })
    }
}
