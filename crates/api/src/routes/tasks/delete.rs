use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use application::task_service;
use domain::task::inputs::DeleteTaskInput;

use crate::middleware::cognito_auth::AuthUser;
use crate::response::{from_app_error, ErrorResponse};
use crate::routes::tasks::types::CountResponse;
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<AuthUser>,
    Path(task_id): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let count = task_service::delete_task::delete_task(
        state.task_repo.as_ref(),
        DeleteTaskInput {
            user_id: user.user_id,
            task_id,
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
    async fn returns_internal_server_error_on_repo_failure() {
        let state = app_state(
            MockTaskRepo::with_delete_result(Err(AppError::database("db error"))),
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
