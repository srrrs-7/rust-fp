use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

use application::user_service;
use domain::user::UpdateUserInput;

use crate::response::{from_app_error, validation_error, ErrorResponse};
use crate::routes::users::{UpdateUserRequest, UserResponse};
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(body): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if body.username.is_none()
        && body.email.is_none()
        && body.name.is_none()
        && body.picture.is_none()
    {
        return Err(validation_error(
            "invalid_body",
            "At least one field must be provided",
        ));
    }

    let user = user_service::update_user(
        &state.user_repo,
        UpdateUserInput {
            user_id,
            username: body.username,
            email: body.email,
            name: body.name,
            picture: body.picture,
        },
    )
    .await
    .map_err(from_app_error)?;

    Ok(Json(UserResponse {
        user_id: user.user_id,
        client_id: user.client_id,
        username: user.username,
        email: user.email,
    }))
}
