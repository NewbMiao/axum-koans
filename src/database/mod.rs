pub mod profile;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn get_db_pool(database_url: String) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
