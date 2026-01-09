use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

use application::user_service;
use domain::user::DeleteUserInput;

use crate::response::{from_app_error, ErrorResponse};
use crate::routes::users::CountResponse;
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let count = user_service::delete_user(&state.user_repo, DeleteUserInput { user_id })
        .await
        .map_err(from_app_error)?;

    Ok(Json(CountResponse { count }))
}
