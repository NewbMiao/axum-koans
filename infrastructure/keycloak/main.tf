terraform {
  required_providers {
    keycloak = {
      source  = "mrparkers/keycloak"
      version = ">= 4.0.0"
    }
  }
}

provider "keycloak" {
  client_id = "admin-cli"
  url       = "http://localhost:8080"
  username  = "admin"
  password  = "admin-sec"
}

resource "keycloak_realm" "realm_axum_koans" {
  realm   = "axum-koans"
  enabled = true
}
# TF_VAR_google_client_id
variable "google_client_id" {
  description = "google client id"
  type        = string
}
# TF_VAR_google_client_id
variable "google_client_secret" {
  description = "google client secret"
  type        = string
}
resource "keycloak_oidc_google_identity_provider" "google" {
  realm         = keycloak_realm.realm_axum_koans.id
  client_id     = var.google_client_id
  client_secret = var.google_client_secret
  trust_email   = true
  hosted_domain = "*"
  sync_mode     = "IMPORT"
  provider_id   = "google"

  default_scopes = "openid profile email"
  # for token exchange to get google access token
  request_refresh_token = true
  # for retrieve idp token (with refresh token)
  store_token                   = true
  add_read_token_role_on_create = true
}

resource "keycloak_openid_client" "client_axum_koans" {
  realm_id = keycloak_realm.realm_axum_koans.id
  name     = "axum-koans"
  enabled  = true


  client_id             = "axum-koans"
  client_secret         = "tF2yB4ELq5zqT6cSpSNJMA1fuq9DVXnc"
  standard_flow_enabled = true

  access_type = "CONFIDENTIAL"
  valid_redirect_uris = [
    "http://localhost:8000/keycloak/login-callback"
  ]
  web_origins        = ["*"]
  use_refresh_tokens = true
}

resource "keycloak_identity_provider_token_exchange_scope_permission" "oidc_idp_permission" {
  realm_id       = keycloak_realm.realm_axum_koans.id
  provider_alias = keycloak_oidc_google_identity_provider.google.alias
  policy_type    = "client"
  clients = [
    keycloak_openid_client.client_axum_koans.id
  ]
}
