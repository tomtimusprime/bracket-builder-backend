use anyhow::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Default, Serialize, Deserialize, FromRow, Clone)]
pub struct Bracket {
    pub id: i32,
    pub topic: String,
    pub size: i32,
    pub created_at: NaiveDateTime,
}

pub async fn init_db() -> Result<MySqlPool> {
    let database_url = std::env::var("DATABASE_URL")?;
    let connection_pool = MySqlPool::connect(&database_url).await?;
    Ok(connection_pool)
}

pub async fn all_brackets(connection_pool: &MySqlPool) -> Result<Vec<Bracket>> {
    Ok(sqlx::query_as::<_, Bracket>("SELECT * FROM Brackets")
        .fetch_all(connection_pool)
        .await?)
}
