use anyhow::anyhow;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

use crate::errors::AppError;

pub type DbPool = Pool<Postgres>;

pub async fn connect_db() -> Result<DbPool, AppError> {
    let db_url = env::var("DB_URL")
        .map_err(|e| AppError::Internal(anyhow!("[ENV]: failed to fetch DB URL: {}", e)))?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .map_err(|e| AppError::Database(format!("failed to connect to Postgres: {e}")))?;

    println!("🛢️ Database Connected!");

    Ok(pool)
}
