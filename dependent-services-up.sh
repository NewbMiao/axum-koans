#!/bin/bash
docker-compose up -d
sqlx migrate run --database-url "postgresql://axum-koans:axum-koans-sec@localhost:15432/axum-koans"
