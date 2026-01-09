use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;

use application::task_service;
use domain::task::CreateTaskInput;

use crate::middleware::cognito_auth::AuthUser;
use crate::response::{from_app_error, validation_error, ErrorResponse};
use crate::routes::tasks::{parse_status, CreateTaskRequest, TaskResponse};
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<AuthUser>,
    Json(body): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if body.content.trim().is_empty() || body.content.len() > 1000 {
        return Err(validation_error(
            "invalid_content",
            "Content must be 1-1000 characters",
        ));
    }

    if let Some(status) = &body.status {
        if parse_status(status).is_err() {
            return Err(validation_error(
                "invalid_status",
                "Status must be PENDING, IN_PROGRESS, or COMPLETED",
            ));
        }
    }

    let task = task_service::create_task(
        &state.task_repo,
        CreateTaskInput {
            user_id: user.user_id,
            content: body.content,
        },
    )
    .await
    .map_err(from_app_error)?;

    Ok(Json(TaskResponse {
        task_id: task.task_id,
        user_id: task.user_id,
        content: task.content,
        completed_at: task.completed_at.map(|dt| dt.to_rfc3339()),
        version: task.version,
    }))
}
