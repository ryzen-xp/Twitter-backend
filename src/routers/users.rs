use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;
use crate::handlers::users::{create_user, get_user};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/user", post(create_user))
        .route("/user/{username}", get(get_user))
}
