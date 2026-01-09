use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use application::task_service;
use domain::task::DeleteTaskInput;

use crate::middleware::cognito_auth::AuthUser;
use crate::response::{from_app_error, ErrorResponse};
use crate::routes::tasks::CountResponse;
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<AuthUser>,
    Path(task_id): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let count = task_service::delete_task(
        &state.task_repo,
        DeleteTaskInput {
            user_id: user.user_id,
            task_id,
        },
    )
    .await
    .map_err(from_app_error)?;

    Ok((StatusCode::OK, Json(CountResponse { count })))
}
