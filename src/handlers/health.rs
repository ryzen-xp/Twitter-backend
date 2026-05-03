use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use sqlx::Executor;

use crate::AppState;

pub async fn get_health_status(State(state): State<Arc<AppState>>) -> (StatusCode, String) {
    match state.db.execute("SELECT 1").await {
        Ok(_) => (StatusCode::OK, String::from("Database is Healthy !!")),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            String::from("Database blast, check it ASAP !!!"),
        ),
    }
}
