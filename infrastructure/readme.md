# Infrastructure

## Setup

### Keycloak configuration

> https://registry.terraform.io/providers/mrparkers/keycloak/latest/docs

```sh
sh ./keycloak/init.sh
```

### Google configuration (optional)

> Better configuration it manually with steps of **Create a Google Application** in [How to setup Sign in with Google using Keycloak](https://keycloakthemes.com/blog/how-to-setup-sign-in-with-google-using-keycloak)

https://registry.terraform.io/providers/DrFaust92/google/latest/docs

Requires:

- [google organization](https://workspace.google.com/) setup first
- [api credentials](https://console.cloud.google.com/apis/credentials) need to be setup with `redirect_uris` and `authorization_origins` later manually

```sh
sh ./google/init.sh
```
