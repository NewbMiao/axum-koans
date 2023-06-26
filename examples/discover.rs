use base64::Engine;
use serde::{Deserialize, Serialize};

use base64::engine::general_purpose;
use openidconnect::core::{
    CoreAuthDisplay, CoreClaimName, CoreClaimType, CoreClientAuthMethod, CoreGrantType,
    CoreJsonWebKey, CoreJsonWebKeyType, CoreJsonWebKeyUse, CoreJweContentEncryptionAlgorithm,
    CoreJweKeyManagementAlgorithm, CoreJwsSigningAlgorithm, CoreResponseMode, CoreResponseType,
    CoreSubjectIdentifierType,
};
use openidconnect::reqwest::http_client;
use openidconnect::{AdditionalProviderMetadata, IssuerUrl, JsonWebKey, ProviderMetadata};

// Teach openidconnect-rs about a Google custom extension to the OpenID Discovery response that we can use as the RFC
// 7009 OAuth 2.0 Token Revocation endpoint. For more information about the Google specific Discovery response see the
// Google OpenID Connect service documentation at: https://developers.google.com/identity/protocols/oauth2/openid-connect#discovery
#[derive(Clone, Debug, Deserialize, Serialize)]
struct RevocationEndpointProviderMetadata {
    revocation_endpoint: String,
}
impl AdditionalProviderMetadata for RevocationEndpointProviderMetadata {}
type GoogleProviderMetadata = ProviderMetadata<
    RevocationEndpointProviderMetadata,
    CoreAuthDisplay,
    CoreClientAuthMethod,
    CoreClaimName,
    CoreClaimType,
    CoreGrantType,
    CoreJweContentEncryptionAlgorithm,
    CoreJweKeyManagementAlgorithm,
    CoreJwsSigningAlgorithm,
    CoreJsonWebKeyType,
    CoreJsonWebKeyUse,
    CoreJsonWebKey,
    CoreResponseMode,
    CoreResponseType,
    CoreSubjectIdentifierType,
>;

fn main() {
    let issuer_url = IssuerUrl::new("http://localhost:8080/realms/axum-koans".to_string())
        .expect("Invalid issuer URL");

    let provider_metadata = GoogleProviderMetadata::discover(&issuer_url, http_client).unwrap();
    let jwk: &CoreJsonWebKey = provider_metadata
        .jwks()
        .keys()
        .iter()
        .find(|v| v.key_id().unwrap().to_string() == "B8IScFzo_EVAOxg0y6cysm9A_u5XkdliFec2lDT1X80")
        .unwrap();

    // valid signature
    let token ="eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJCOElTY0Z6b19FVkFPeGcweTZjeXNtOUFfdTVYa2RsaUZlYzJsRFQxWDgwIn0.eyJleHAiOjE2ODc3ODgyNjEsImlhdCI6MTY4Nzc1MjI4NSwiYXV0aF90aW1lIjoxNjg3NzUyMjYxLCJqdGkiOiI4ZTA4NjRmYy1iNWEyLTQyMGQtOTBmMy1iMmRjZTYzMjkyZWEiLCJpc3MiOiJodHRwOi8vbG9jYWxob3N0OjgwODAvcmVhbG1zL2F4dW0ta29hbnMiLCJhdWQiOlsiYnJva2VyIiwiYWNjb3VudCJdLCJzdWIiOiI1ZjIzZmUxNS03NzhhLTQ5OWItYjdhOS0yMTBmMWVhNjk1ZDgiLCJ0eXAiOiJCZWFyZXIiLCJhenAiOiJheHVtLWtvYW5zIiwic2Vzc2lvbl9zdGF0ZSI6ImEyN2M4MzQ2LTNjODQtNDQ4Zi1iYWIzLThhNGVhNjE1NmEwZiIsImFjciI6IjAiLCJhbGxvd2VkLW9yaWdpbnMiOlsiKiJdLCJyZWFsbV9hY2Nlc3MiOnsicm9sZXMiOlsib2ZmbGluZV9hY2Nlc3MiLCJ1bWFfYXV0aG9yaXphdGlvbiIsImRlZmF1bHQtcm9sZXMtYXh1bS1rb2FucyJdfSwicmVzb3VyY2VfYWNjZXNzIjp7ImJyb2tlciI6eyJyb2xlcyI6WyJyZWFkLXRva2VuIl19LCJhY2NvdW50Ijp7InJvbGVzIjpbIm1hbmFnZS1hY2NvdW50IiwibWFuYWdlLWFjY291bnQtbGlua3MiLCJ2aWV3LXByb2ZpbGUiXX19LCJzY29wZSI6Im9wZW5pZCBwcm9maWxlIGVtYWlsIiwic2lkIjoiYTI3YzgzNDYtM2M4NC00NDhmLWJhYjMtOGE0ZWE2MTU2YTBmIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsIm5hbWUiOiJOZXdiIE1pYW8iLCJwcmVmZXJyZWRfdXNlcm5hbWUiOiJjaGF0Ym90QG5ld2JtaWFvLmNvbSIsImdpdmVuX25hbWUiOiJOZXdiIiwiZmFtaWx5X25hbWUiOiJNaWFvIiwiZW1haWwiOiJjaGF0Ym90QG5ld2JtaWFvLmNvbSJ9.ofvBDsv9vrWlwu_fbx_aB5JPibC3ZSwWDIGai6oTBXMtjliXhu4BE-hgZm3VheE_5XXIA9WlM2O8o_NDgB7mKfkEPmLT5m2VbVe-R-IKqVkTcRX83_dMdUyJOeDv6F1e_6yjovLQ-R941PqCR6NCvEaMVQUSqcZ5NRtgMVdzO_L-KIjpP7YWv1lUBfE_EII3t--HuoTsBSOYWVofYrQvNUC8_sL-8GQknvNl974cKJeyvmVL7GYTDosxkbh7e0L10AgLlmrnJA71E4xDXyoLSQ2BXRd_Mqj8gbPMzEtJBzRfJWm4bRQbsJQFJFBcTYs1BHay2vhZtQsYqNAKGMvxGQ"; //gitleaks:allow

    // invalid: Err(CryptoError("bad signature"))
    // let token="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"; //gitleaks:allow

    let test = token.split('.').collect::<Vec<&str>>();
    let headers = test[0];
    let payload = test[1];
    let signature = test[2];

    let verify = jwk.verify_signature(
        &CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha256,
        format!("{}.{}", headers, payload).as_bytes(),
        &decode_segment(signature).unwrap(),
    );
    println!("Verified with jwk: {:?}", verify);
    let revocation_endpoint = provider_metadata
        .additional_metadata()
        .revocation_endpoint
        .clone();
    println!(
        "Discovered Google revocation endpoint: {}",
        revocation_endpoint
    );
}

fn decode_segment(segment: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::URL_SAFE_NO_PAD.decode(segment)
}
