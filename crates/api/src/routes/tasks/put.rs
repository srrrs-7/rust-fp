use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use application::task_service;
use domain::task::UpdateTaskInput;

use crate::middleware::cognito_auth::AuthUser;
use crate::response::{from_app_error, validation_error, ErrorResponse};
use crate::routes::tasks::{parse_status, CountResponse, TaskStatus, UpdateTaskRequest};
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<AuthUser>,
    Path(task_id): Path<String>,
    Json(body): Json<UpdateTaskRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if body.content.is_none() && body.status.is_none() {
        return Err(validation_error(
            "invalid_body",
            "At least one field (content or status) must be provided",
        ));
    }

    if let Some(content) = &body.content {
        if content.trim().is_empty() || content.len() > 1000 {
            return Err(validation_error(
                "invalid_content",
                "Content must be 1-1000 characters",
            ));
        }
    }

    let completed_at = match body.status.as_deref() {
        Some(status) => match parse_status(status) {
            Ok(TaskStatus::Completed) => Some(Some(chrono::Utc::now())),
            Ok(_) => Some(None),
            Err(_) => {
                return Err(validation_error(
                    "invalid_status",
                    "Status must be PENDING, IN_PROGRESS, or COMPLETED",
                ));
            }
        },
        None => None,
    };

    let count = task_service::update_task(
        state.task_repo.as_ref(),
        UpdateTaskInput {
            user_id: user.user_id,
            task_id,
            content: body.content,
            completed_at,
            version: body.version,
        },
    )
    .await
    .map_err(from_app_error)?;

    Ok((StatusCode::OK, Json(CountResponse { count })))
}

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Path, State};
    use axum::http::StatusCode;
    use axum::Json;
    use domain::error::AppError;

    use super::handler;
    use crate::routes::tasks::UpdateTaskRequest;
    use crate::routes::test_support::{
        app_state, assert_status, auth_user, MockTaskRepo, MockUserRepo,
    };

    #[tokio::test]
    async fn returns_bad_request_for_empty_body() {
        let state = app_state(MockTaskRepo::default(), MockUserRepo::default());
        let body = UpdateTaskRequest {
            content: None,
            status: None,
            version: 0,
        };

        let result = handler(
            State(state),
            Extension(auth_user()),
            Path("task-1".to_string()),
            Json(body),
        )
        .await;

        assert_status(result, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn returns_ok_on_success() {
        let state = app_state(MockTaskRepo::default(), MockUserRepo::default());
        let body = UpdateTaskRequest {
            content: Some("updated".to_string()),
            status: None,
            version: 1,
        };

        let result = handler(
            State(state),
            Extension(auth_user()),
            Path("task-1".to_string()),
            Json(body),
        )
        .await;

        assert_status(result, StatusCode::OK);
    }

    #[tokio::test]
    async fn returns_internal_server_error_on_repo_failure() {
        let state = app_state(
            MockTaskRepo::with_update_result(Err(AppError::database("db error"))),
            MockUserRepo::default(),
        );
        let body = UpdateTaskRequest {
            content: Some("updated".to_string()),
            status: None,
            version: 1,
        };

        let result = handler(
            State(state),
            Extension(auth_user()),
            Path("task-1".to_string()),
            Json(body),
        )
        .await;

        assert_status(result, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
