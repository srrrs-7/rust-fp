use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

use application::task_service;
use domain::error::AppError;
use domain::task::inputs::GetTaskInput;

use crate::middleware::cognito_auth::AuthUser;
use crate::response::{from_app_error, ErrorResponse};
use crate::routes::tasks::types::TaskResponse;
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<AuthUser>,
    Path(task_id): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let task = task_service::get_task::get_task(
        state.task_repo.as_ref(),
        GetTaskInput {
            user_id: user.user_id,
            task_id,
        },
    )
    .await
    .map_err(from_app_error)?
    .ok_or_else(|| from_app_error(AppError::not_found("Task", "Task not found")))?;

    Ok(Json(TaskResponse::from(task)))
}

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path, State};
    use axum::http::StatusCode;
    use domain::error::AppError;

    use super::handler;
    use crate::routes::test_support::{
        app_state, assert_status, auth_user, MockTaskRepo, MockUserRepo,
    };

    #[tokio::test]
    async fn returns_ok_on_success() {
        let state = app_state(MockTaskRepo::default(), MockUserRepo::default());

        let result = handler(
            State(state),
            Extension(auth_user()),
            Path("task-1".to_string()),
        )
        .await;

        assert_status(result, StatusCode::OK);
    }

    #[tokio::test]
    async fn returns_not_found_when_missing() {
        let state = app_state(
            MockTaskRepo::with_get_result(Ok(None)),
            MockUserRepo::default(),
        );

        let result = handler(
            State(state),
            Extension(auth_user()),
            Path("task-1".to_string()),
        )
        .await;

        assert_status(result, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn returns_internal_server_error_on_repo_failure() {
        let state = app_state(
            MockTaskRepo::with_get_result(Err(AppError::database("db error"))),
            MockUserRepo::default(),
        );

        let result = handler(
            State(state),
            Extension(auth_user()),
            Path("task-1".to_string()),
        )
        .await;

        assert_status(result, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
