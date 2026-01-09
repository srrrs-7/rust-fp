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
    let count = user_service::delete_user(state.user_repo.as_ref(), DeleteUserInput { user_id })
        .await
        .map_err(from_app_error)?;

    Ok(Json(CountResponse { count }))
}

#[cfg(test)]
mod tests {
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use domain::error::AppError;

    use super::handler;
    use crate::routes::test_support::{app_state, assert_status, MockTaskRepo, MockUserRepo};

    #[tokio::test]
    async fn returns_ok_on_success() {
        let state = app_state(MockTaskRepo::default(), MockUserRepo::default());

        let result = handler(State(state), Path("user-1".to_string())).await;

        assert_status(result, StatusCode::OK);
    }

    #[tokio::test]
    async fn returns_internal_server_error_on_repo_failure() {
        let state = app_state(
            MockTaskRepo::default(),
            MockUserRepo::with_delete_result(Err(AppError::database("db error"))),
        );

        let result = handler(State(state), Path("user-1".to_string())).await;

        assert_status(result, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
