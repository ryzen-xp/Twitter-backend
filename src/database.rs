use anyhow::anyhow;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

use crate::errors::AppError;

pub type DbPool = Pool<Postgres>;
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./src/migrations");

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

pub async fn initialize_database(pool: &DbPool) -> Result<(), AppError> {
    MIGRATOR
        .run(pool)
        .await
        .map_err(|e| AppError::Database(format!("failed to run database migrations: {e}")))?;

    println!("📦 Database migrations are up to date!");

    Ok(())
}
