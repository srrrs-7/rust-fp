use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use application::user_service;
use domain::user::CreateUserInput;

use crate::response::{from_app_error, validation_error, ErrorResponse};
use crate::routes::users::{CreateUserRequest, UserResponse};
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if body.user_id.trim().is_empty() || body.user_id.len() > 128 {
        return Err(validation_error(
            "invalid_user_id",
            "User ID must be 1-128 characters",
        ));
    }
    if body.client_id.trim().is_empty() {
        return Err(validation_error(
            "invalid_client_id",
            "Client ID is required",
        ));
    }
    if body.username.trim().is_empty() {
        return Err(validation_error("invalid_username", "Username is required"));
    }
    if body.email.trim().is_empty() {
        return Err(validation_error("invalid_email", "Email is required"));
    }

    let user = user_service::create_user(
        &state.user_repo,
        CreateUserInput {
            user_id: body.user_id,
            client_id: body.client_id,
            username: body.username,
            email: body.email,
            name: body.name,
            picture: body.picture,
        },
    )
    .await
    .map_err(from_app_error)?;

    Ok((
        StatusCode::CREATED,
        Json(UserResponse {
            user_id: user.user_id,
            client_id: user.client_id,
            username: user.username,
            email: user.email,
        }),
    ))
}
