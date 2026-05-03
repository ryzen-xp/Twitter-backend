use anyhow::anyhow;
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use std::env;

use crate::errors::AppError;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn connect_db() -> Result<DbPool, AppError> {
    let db_url = env::var("DB_URL")
        .map_err(|e| AppError::Internal(anyhow!("[ENV]: failed to fetch DB URL: {}", e)))?;

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder()
        .max_size(5)
        .build(manager)
        .map_err(|e| AppError::Database(format!("failed to build Diesel pool: {e}")))?;

    println!("🛢️ Database Connected!");

    Ok(pool)
}
