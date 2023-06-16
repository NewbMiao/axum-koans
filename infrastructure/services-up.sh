#!/bin/bash
set -e
workspace=$(cd "$(dirname "$0")" && pwd -P)
mode=${1:-up}
up_args=""
{
    cd "$workspace"
    if [ "$mode" = "down" ]; then
        docker-compose down
        exit 0
    elif [ "$mode" = "down" ]; then
        up_args="--build"
    fi
    docker-compose up -d $up_args
    # Check if sqlx-cli is installed
    if ! command -v sqlx &>/dev/null; then
        echo "sqlx-cli not found. Installing..."
        cargo install sqlx-cli --no-default-features --features native-tls,postgres --locked
    else
        echo "sqlx-cli already installed"
    fi
    cd ../
    sqlx migrate run --database-url "postgresql://axum-koans:axum-koans-sec@localhost:15432/axum-koans"
    cargo sqlx prepare
    echo "postgres migration done"
}
