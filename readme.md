# axum-koans

## setup

1. add [google oauth configuration](https://console.cloud.google.com/apis/credentials)

- Authorized JavaScript origins
  - http://localhost:8000
  - http://localhost:8080
- Authorized redirect URIs
  - http://localhost:8000/google/auth-callback
  - http://localhost:8080/realms/axum-koans/broker/google/endpoint

2. add google client configuration in keycloak realm.json

```json
// replace `clientSecret` and `clientId` with your own google oauth client
"identityProviders": [
    {
      "alias": "google",
      "internalId": "955ac868-8fbf-44ad-80e5-41c5e9d44953",
      "providerId": "google",
      "enabled": true,
      "updateProfileFirstLoginMode": "on",
      "trustEmail": false,
      "storeToken": false,
      "addReadTokenRoleOnCreate": false,
      "authenticateByDefault": false,
      "linkOnly": false,
      "firstBrokerLoginFlowAlias": "first broker login",
      "config": {
        "offlineAccess": "true",
        "userIp": "false",
        "clientSecret": "**********",
        "clientId": "**********.apps.googleusercontent.com"
      }
    }
  ]
```

3. init application

```shell
# keycloak and postgres
docker-compose up
# database migration
# install database migrations tool sqx
cargo install sqlx-cli --no-default-features --features native-tls,postgres --locked
sqlx migrate run
```

## features

### Oauth

- google-oauth

http://localhost:8000/google/auth

- keycloak-google-oauth (token-exchange)

http://localhost:8000/keycloak/login (use google login method)
