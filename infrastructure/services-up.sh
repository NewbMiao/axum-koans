#!/bin/bash
set -e
workspace=$(cd "$(dirname "$0")" && pwd -P)
{
    cd "$workspace"
    docker-compose up -d
    # Check if sqlx-cli is installed
    if ! command -v sqlx &>/dev/null; then
        echo "sqlx-cli not found. Installing..."
        cargo install sqlx-cli --no-default-features --features native-tls,postgres --locked
    else
        echo "sqlx-cli already installed"
    fi
    cd ../
    sqlx migrate run --database-url "postgresql://axum-koans:axum-koans-sec@localhost:15432/axum-koans"
    echo "postgres migration done"
}
