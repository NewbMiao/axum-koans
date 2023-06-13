#!/bin/bash
set -e
workspace=$(cd "$(dirname "$0")" && pwd -P)
{
    cd "$workspace"
    docker-compose up -d
    cd -
    sqlx migrate run --database-url "postgresql://axum-koans:axum-koans-sec@localhost:15432/axum-koans"
}
