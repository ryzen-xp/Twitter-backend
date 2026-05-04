use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;

use crate::AppState;
use crate::errors::AppError;
use crate::models::user::{CreateUserDto, User, UserResponse};

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

    // Now check that the  Username is  Already Exist to not .

    if is_username_exist(&state, &user.username).await.unwrap() {
        return Err(AppError::Validation(
            "Username is already exist".to_string(),
        ));
    }

    let created_user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, handle, email, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING
            id,
            username,
            handle,
            email,
            password_hash,
            bio,
            avatar_url,
            banner_url,
            is_verified,
            is_private,
            follower_count,
            following_count,
            tweet_count,
            created_at,
            updated_at
        "#,
    )
    .bind(user.username.trim())
    .bind(user.handle.trim())
    .bind(user.email.trim())
    .bind(user.password.trim())
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::Database(format!("failed to insert user: {e}")))?;

    Ok((StatusCode::CREATED, Json(created_user.into())))
}

//  Fetch User from DB .

pub async fn get_user(
    State(_state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    Err(AppError::NotFound("user not found".to_string()))
}

async fn is_username_exist(state: &Arc<AppState>, username: &String) -> Result<bool, AppError> {
    let username_exists: bool = sqlx::query_scalar(
        r#"
    SELECT EXISTS (
        SELECT 1
        FROM users
        WHERE username = $1
    )
    "#,
    )
    .bind(username.trim())
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::Database(format!("failed to check username: {e}")))?;

    Ok(username_exists)
}
