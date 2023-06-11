use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

use crate::errors::ServerError;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Profile {
    id: i32,
    account_id: String,
    username: String,
    email: String,
    picture: Option<String>,
    #[serde(skip)]
    refresh_token: String,
    created_date: DateTime<Utc>,
    updated_date: Option<DateTime<Utc>>,
}
impl Profile {
    pub fn new(
        account_id: String,
        username: String,
        email: String,
        picture: Option<String>,
        refresh_token: String,
    ) -> Self {
        Self {
            id: 0,
            account_id,
            username,
            email,
            picture,
            refresh_token,
            created_date: Default::default(),
            updated_date: Default::default(),
        }
    }
    pub async fn find_one(&self, db_pool: Pool<Postgres>) -> Result<Profile, ServerError> {
        sqlx::query_as!(
            Profile,
            "SELECT id, account_id, username, email, picture, refresh_token, created_date, updated_date FROM profiles WHERE account_id = $1",
            self.account_id
        )
        .fetch_one(&db_pool)
        .await
        .map_err(ServerError::PGError)
    }
    pub async fn save(&self, db_pool: Pool<Postgres>) -> Result<Profile, ServerError> {
        sqlx::query!(
            r#"
            INSERT INTO profiles (account_id, username, email, picture, refresh_token)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (account_id) DO UPDATE SET
                account_id = EXCLUDED.account_id,
                username = EXCLUDED.username,
                email = EXCLUDED.email,
                picture = EXCLUDED.picture,
                refresh_token = EXCLUDED.refresh_token
            RETURNING *
            "#,
            self.account_id,
            self.username,
            self.email,
            self.picture,
            self.refresh_token
        )
        .fetch_one(&db_pool)
        .await
        .map_err(ServerError::PGError)
        .map(|row| Profile {
            id: row.id,
            account_id: row.account_id.clone(),
            username: row.username.clone(),
            email: row.email.clone(),
            picture: row.picture,
            refresh_token: row.refresh_token.clone(),
            created_date: row.created_date,
            updated_date: row.updated_date,
        })
    }
}
