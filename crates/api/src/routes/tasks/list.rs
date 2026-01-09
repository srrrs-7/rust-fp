use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;

use application::task_service;
use domain::task::ListTasksInput;

use crate::middleware::cognito_auth::AuthUser;
use crate::response::{from_app_error, ErrorResponse};
use crate::routes::tasks::{Pagination, TaskListResponse, TaskResponse};
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<AuthUser>,
    Query(params): Query<Pagination>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).max(1).min(100);

    let tasks = task_service::list_tasks(
        &state.task_repo,
        ListTasksInput {
            user_id: user.user_id,
            page,
            limit,
        },
    )
    .await
    .map_err(from_app_error)?;

    let response = TaskListResponse {
        tasks: tasks
            .into_iter()
            .map(|task| TaskResponse {
                task_id: task.task_id,
                user_id: task.user_id,
                content: task.content,
                completed_at: task.completed_at.map(|dt| dt.to_rfc3339()),
                version: task.version,
            })
            .collect(),
        page,
        limit,
    };

    Ok(Json(response))
}
