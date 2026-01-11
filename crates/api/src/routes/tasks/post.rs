use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;

use application::task_service;
use domain::task::inputs::CreateTaskInput;

use crate::middleware::cognito_auth::AuthUser;
use crate::response::{from_app_error, validation_error, ErrorResponse};
use crate::routes::tasks::types::{parse_status, CreateTaskRequest, TaskResponse};
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

    let task = task_service::create_task::create_task(
        state.task_repo.as_ref(),
        CreateTaskInput {
            user_id: user.user_id,
            content: body.content,
        },
    )
    .await
    .map_err(from_app_error)?;

    Ok(Json(TaskResponse::from(task)))
}

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, State};
    use axum::http::StatusCode;
    use axum::Json;
    use domain::error::AppError;

    use super::handler;
    use crate::routes::tasks::types::CreateTaskRequest;
    use crate::routes::test_support::{
        app_state, assert_status, auth_user, MockTaskRepo, MockUserRepo,
    };

    #[tokio::test]
    async fn returns_bad_request_for_invalid_content() {
        let state = app_state(MockTaskRepo::default(), MockUserRepo::default());
        let body = CreateTaskRequest {
            content: " ".to_string(),
            status: None,
        };

        let result = handler(State(state), Extension(auth_user()), Json(body)).await;

        assert_status(result, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn returns_bad_request_for_invalid_status() {
        let state = app_state(MockTaskRepo::default(), MockUserRepo::default());
        let body = CreateTaskRequest {
            content: "task".to_string(),
            status: Some("INVALID".to_string()),
        };

        let result = handler(State(state), Extension(auth_user()), Json(body)).await;

        assert_status(result, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn returns_ok_on_success() {
        let state = app_state(MockTaskRepo::default(), MockUserRepo::default());
        let body = CreateTaskRequest {
            content: "task".to_string(),
            status: None,
        };

        let result = handler(State(state), Extension(auth_user()), Json(body)).await;

        assert_status(result, StatusCode::OK);
    }

    #[tokio::test]
    async fn returns_internal_server_error_on_repo_failure() {
        let state = app_state(
            MockTaskRepo::with_create_result(Err(AppError::database("db error"))),
            MockUserRepo::default(),
        );
        let body = CreateTaskRequest {
            content: "task".to_string(),
            status: None,
        };

        let result = handler(State(state), Extension(auth_user()), Json(body)).await;

        assert_status(result, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
