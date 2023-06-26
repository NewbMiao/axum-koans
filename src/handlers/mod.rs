use serde::Deserialize;

pub mod auth;
pub mod login;
pub mod user;
#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}
