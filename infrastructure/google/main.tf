provider "google" {
  project = "axum-koans"
}


resource "google_project" "goauth" {
  project_id = "axum-koans-oauth"
  name       = "axum-koans-oauth"
  # required an organization setup first: https://workspace.google.com/
  org_id = "584666702915"
}


resource "google_project_service" "goauth_service" {
  project = google_project.goauth.project_id
  service = "iap.googleapis.com"
}

resource "google_iap_brand" "goauth_brand" {
  support_email     = "chatbot@newbmiao.com"
  application_title = "axum-koans-goauth"
  project           = google_project_service.goauth_service.project
}

resource "google_iap_client" "goauth_client" {
  display_name = "axum koans google client"
  brand        = google_iap_brand.goauth_brand.name
  # Note: below config not working with google api yet. need to configure them manually by creating a new web application typed client credential
  #   redirect_uris = ["http://localhost:8000/google/auth-callback", "http://localhost:8080/realms/axum-koans/broker/google/endpoint"]

  #   authorized_origins = ["http://localhost:8000", "http://localhost:8080"]
}
