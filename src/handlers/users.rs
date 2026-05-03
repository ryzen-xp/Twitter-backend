use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use diesel::prelude::*;

use crate::errors::AppError;
use crate::models::user::{CreateUserDto, NewUser, User, UserResponse};
use crate::schema::users::users;
use crate::AppState;

// [Post] create_user

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<CreateUserDto>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    if user.username.trim().is_empty() {
        return Err(AppError::Validation("username is required".to_string()));
    }

    if user.handle.trim().is_empty() {
        return Err(AppError::Validation("handle is required".to_string()));
    }

    if user.email.trim().is_empty() {
        return Err(AppError::Validation("email is required".to_string()));
    }

    if user.password.trim().is_empty() {
        return Err(AppError::Validation("password is required".to_string()));
    }

    let new_user = NewUser {
        username: user.username.trim().to_string(),
        handle: user.handle.trim().to_string(),
        email: user.email.trim().to_string(),
        password_hash: user.password.trim().to_string(),
    };

    let db = state.db.clone();
    let created_user = tokio::task::spawn_blocking(move || -> Result<User, AppError> {
        let mut connection = db
            .get()
            .map_err(|e| AppError::Database(format!("failed to get Diesel connection: {e}")))?;

        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut connection)
            .map_err(|e| AppError::Database(format!("failed to insert user: {e}")))
    })
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("user insert task failed: {e}")))??;

    Ok((StatusCode::CREATED, Json(created_user.into())))
}
