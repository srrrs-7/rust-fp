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
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    let tasks = task_service::list_tasks(
        state.task_repo.as_ref(),
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

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Query, State};
    use axum::http::StatusCode;
    use domain::error::AppError;

    use super::handler;
    use crate::routes::tasks::Pagination;
    use crate::routes::test_support::{
        app_state, assert_status, auth_user, MockTaskRepo, MockUserRepo,
    };

    #[tokio::test]
    async fn returns_ok_on_success() {
        let state = app_state(MockTaskRepo::default(), MockUserRepo::default());
        let params = Pagination {
            page: Some(1),
            limit: Some(10),
        };

        let result = handler(State(state), Extension(auth_user()), Query(params)).await;

        assert_status(result, StatusCode::OK);
    }

    #[tokio::test]
    async fn returns_internal_server_error_on_repo_failure() {
        let state = app_state(
            MockTaskRepo::with_list_result(Err(AppError::database("db error"))),
            MockUserRepo::default(),
        );
        let params = Pagination {
            page: None,
            limit: None,
        };

        let result = handler(State(state), Extension(auth_user()), Query(params)).await;

        assert_status(result, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
