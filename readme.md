# axum-koans

## setup

1. add [google oauth configuration](https://console.cloud.google.com/apis/credentials)

- Authorized JavaScript origins
  - http://localhost:8000
  - http://localhost:8080
- Authorized redirect URIs
  - http://localhost:8000/google/auth-callback
  - http://localhost:8080/realms/axum-koans/broker/google/endpoint

2. add google client configuration in below `.env` file

3. init application

```shell
# Optional, for precommit initialization
sh ./init.sh

# configure initialization, and put your google client configuration in
cp .env.dev .env

# init keycloak and postgres
# Details in [infrastructure](/infrastructure/readme.md)
sh ./infrastructure/keycloak/init.sh

# Optional, database migration, already migrated in above steps
# install database migrations tool sqx
# cargo install sqlx-cli --no-default-features --features native-tls,postgres --locked
sqlx migrate run
```

## features

### Oauth

- google-oauth

  http://localhost:8000/google/auth

- keycloak-google-oauth (token-exchange)

  http://localhost:8000/keycloak/login (use google login method)
