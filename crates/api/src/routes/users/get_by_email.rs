use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

use application::user_service;
use domain::error::AppError;
use domain::user::GetUserByEmailInput;

use crate::response::{from_app_error, ErrorResponse};
use crate::routes::users::UserResponse;
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let user = user_service::get_user_by_email(&state.user_repo, GetUserByEmailInput { email })
        .await
        .map_err(from_app_error)?
        .ok_or_else(|| from_app_error(AppError::not_found("User", "User not found")))?;

    Ok(Json(UserResponse {
        user_id: user.user_id,
        client_id: user.client_id,
        username: user.username,
        email: user.email,
    }))
}
