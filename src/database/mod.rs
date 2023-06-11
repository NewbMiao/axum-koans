pub mod profile;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{config::DatabaseConfig, errors::ServerError};

pub async fn get_db_pool(db_config: DatabaseConfig) -> Result<Pool<Postgres>, ServerError> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_config.url)
        .await?;
    Ok(pool)
}
