# axum-koans

## setup

1. add [google oauth api credentials](https://console.cloud.google.com/apis/credentials)

> refer to steps of **Create a Google Application** in [How to setup Sign in with Google using Keycloak](https://keycloakthemes.com/blog/how-to-setup-sign-in-with-google-using-keycloak)

- Authorized JavaScript origins
  - http://localhost:8000
  - http://localhost:8080
- Authorized redirect URIs
  - http://localhost:8000/google/auth-callback
  - http://localhost:8080/realms/axum-koans/broker/google/endpoint

2. add google api credentials (clientId and clientSecret) in below `.env` file

3. init application

```shell
# [Optional], for devtool initialization
sh ./devtool-init.sh

# configure initialization, and put your google client configuration in
cp .env.dev .env

# init keycloak and postgres
# Details in [infrastructure](/infrastructure/readme.md)
sh ./infrastructure/keycloak/init.sh

# start keycloak and postgres
sh ./infrastructure/services-up.sh

# start app
cargo run
```

## features

### Oauth

- google-oauth ([Authorization Code flow (with PKCE)](https://blog.postman.com/pkce-oauth-how-to/))

  http://localhost:8000/google/auth

- keycloak-google-oauth ([Token Exchange](https://www.keycloak.org/docs/latest/securing_apps/#_token-exchange) && [Retrieving external IDP tokens](https://www.keycloak.org/docs/latest/server_admin/#retrieving-external-idp-tokens))

  http://localhost:8000/keycloak/login (use google login method)
