use std::time::Duration;

use jwtk::jwk::RemoteJwksVerifier;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PayloadExtraFields {
    pub email_verified: bool,
    pub name: String,
    pub preferred_username: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}
#[tokio::main]
async fn main() -> jwtk::Result<()> {
    let token ="eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJCOElTY0Z6b19FVkFPeGcweTZjeXNtOUFfdTVYa2RsaUZlYzJsRFQxWDgwIn0.eyJleHAiOjE2ODc3ODgyNjEsImlhdCI6MTY4Nzc1MjI4NSwiYXV0aF90aW1lIjoxNjg3NzUyMjYxLCJqdGkiOiI4ZTA4NjRmYy1iNWEyLTQyMGQtOTBmMy1iMmRjZTYzMjkyZWEiLCJpc3MiOiJodHRwOi8vbG9jYWxob3N0OjgwODAvcmVhbG1zL2F4dW0ta29hbnMiLCJhdWQiOlsiYnJva2VyIiwiYWNjb3VudCJdLCJzdWIiOiI1ZjIzZmUxNS03NzhhLTQ5OWItYjdhOS0yMTBmMWVhNjk1ZDgiLCJ0eXAiOiJCZWFyZXIiLCJhenAiOiJheHVtLWtvYW5zIiwic2Vzc2lvbl9zdGF0ZSI6ImEyN2M4MzQ2LTNjODQtNDQ4Zi1iYWIzLThhNGVhNjE1NmEwZiIsImFjciI6IjAiLCJhbGxvd2VkLW9yaWdpbnMiOlsiKiJdLCJyZWFsbV9hY2Nlc3MiOnsicm9sZXMiOlsib2ZmbGluZV9hY2Nlc3MiLCJ1bWFfYXV0aG9yaXphdGlvbiIsImRlZmF1bHQtcm9sZXMtYXh1bS1rb2FucyJdfSwicmVzb3VyY2VfYWNjZXNzIjp7ImJyb2tlciI6eyJyb2xlcyI6WyJyZWFkLXRva2VuIl19LCJhY2NvdW50Ijp7InJvbGVzIjpbIm1hbmFnZS1hY2NvdW50IiwibWFuYWdlLWFjY291bnQtbGlua3MiLCJ2aWV3LXByb2ZpbGUiXX19LCJzY29wZSI6Im9wZW5pZCBwcm9maWxlIGVtYWlsIiwic2lkIjoiYTI3YzgzNDYtM2M4NC00NDhmLWJhYjMtOGE0ZWE2MTU2YTBmIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsIm5hbWUiOiJOZXdiIE1pYW8iLCJwcmVmZXJyZWRfdXNlcm5hbWUiOiJjaGF0Ym90QG5ld2JtaWFvLmNvbSIsImdpdmVuX25hbWUiOiJOZXdiIiwiZmFtaWx5X25hbWUiOiJNaWFvIiwiZW1haWwiOiJjaGF0Ym90QG5ld2JtaWFvLmNvbSJ9.ofvBDsv9vrWlwu_fbx_aB5JPibC3ZSwWDIGai6oTBXMtjliXhu4BE-hgZm3VheE_5XXIA9WlM2O8o_NDgB7mKfkEPmLT5m2VbVe-R-IKqVkTcRX83_dMdUyJOeDv6F1e_6yjovLQ-R941PqCR6NCvEaMVQUSqcZ5NRtgMVdzO_L-KIjpP7YWv1lUBfE_EII3t--HuoTsBSOYWVofYrQvNUC8_sL-8GQknvNl974cKJeyvmVL7GYTDosxkbh7e0L10AgLlmrnJA71E4xDXyoLSQ2BXRd_Mqj8gbPMzEtJBzRfJWm4bRQbsJQFJFBcTYs1BHay2vhZtQsYqNAKGMvxGQ"; //gitleaks:allow

    // invalid: Error: NoKey
    // let token="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"; //gitleaks:allow
    let jwks = RemoteJwksVerifier::new(
        "http://localhost:8080/realms/axum-koans/protocol/openid-connect/certs".into(),
        None,
        Duration::from_secs(300),
    );
    let c = jwks.verify::<PayloadExtraFields>(token).await?;

    println!("headers:\n{}", serde_json::to_string(c.header())?);
    println!("claims:\n{}", serde_json::to_string(c.claims())?);

    Ok(())
}
