use serde::Deserialize;

pub mod auth;
pub mod login;
#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}
