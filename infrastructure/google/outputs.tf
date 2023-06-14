output "goauth_client_id" {
  value = google_iap_client.goauth_client.client_id
}
output "goauth_client_secret" {
  value     = google_iap_client.goauth_client.secret
  sensitive = true
}
