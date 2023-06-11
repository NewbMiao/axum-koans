pub mod google_auth;
pub mod keycloak_auth;

#[derive(Debug)]
pub enum KeyCloakIdp {
    Google,
}

impl Default for KeyCloakIdp {
    fn default() -> Self {
        Self::Google
    }
}

impl KeyCloakIdp {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Google => "google",
        }
    }
}
