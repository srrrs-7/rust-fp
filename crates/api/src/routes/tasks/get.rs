use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

use application::task_service;
use domain::error::AppError;
use domain::task::GetTaskInput;

use crate::middleware::cognito_auth::AuthUser;
use crate::response::{from_app_error, ErrorResponse};
use crate::routes::tasks::TaskResponse;
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<AuthUser>,
    Path(task_id): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let task = task_service::get_task(
        &state.task_repo,
        GetTaskInput {
            user_id: user.user_id,
            task_id,
        },
    )
    .await
    .map_err(from_app_error)?
    .ok_or_else(|| from_app_error(AppError::not_found("Task", "Task not found")))?;

    Ok(Json(TaskResponse {
        task_id: task.task_id,
        user_id: task.user_id,
        content: task.content,
        completed_at: task.completed_at.map(|dt| dt.to_rfc3339()),
        version: task.version,
    }))
}
