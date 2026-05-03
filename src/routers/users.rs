use std::sync::Arc;

use axum::{Router, routing::post};

use crate::handlers::users::create_user;
use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/users", post(create_user))
}

