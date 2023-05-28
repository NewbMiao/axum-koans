use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

#[derive(Deserialize, Serialize, FromRow)]
pub struct Profile {
    id: i32,
    account_id: String,
    username: String,
    email: String,
    refresh_token: String,
}

impl Profile {
    pub fn new(account_id: String, username: String, email: String, refresh_token: String) -> Self {
        Self {
            id: 0,
            account_id,
            username,
            email,
            refresh_token,
        }
    }
    pub async fn find_one(&self, db_pool: Pool<Postgres>) -> Result<Profile, sqlx::Error> {
        sqlx::query_as!(
            Profile,
            "SELECT id, account_id, username, email, refresh_token FROM profiles WHERE id = $1",
            self.id
        )
        .fetch_one(&db_pool)
        .await
    }
    pub async fn save(&self, db_pool: Pool<Postgres>) -> Result<i32, sqlx::Error> {
        let row = sqlx::query!(
            "INSERT INTO profiles (account_id, username, email, refresh_token) VALUES ($1, $2, $3, $4) RETURNING id",
            self.account_id,
            self.username,
            self.email,
            self.refresh_token
        )
        .fetch_one(&db_pool).await?;
        Ok(row.id)
    }
}
