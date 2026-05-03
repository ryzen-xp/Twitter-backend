use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
};
use diesel::{RunQueryDsl, sql_query};

use crate::AppState;

pub async fn get_health_status(State(state): State<Arc<AppState>>) -> (StatusCode, String) {
    let db = state.db.clone();

    match tokio::task::spawn_blocking(move || {
        let mut connection = db.get().map_err(|_| ())?;
        sql_query("SELECT 1").execute(&mut connection).map_err(|_| ())
    })
    .await
    {
        Ok(Ok(_)) => (StatusCode::OK, String::from("Database is Healthy !!")),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            String::from("Database blast, check it ASAP !!!"),
        ),
        Ok(Err(_)) => (
            StatusCode::SERVICE_UNAVAILABLE,
            String::from("Database blast, check it ASAP !!!"),
        ),
    }
}
