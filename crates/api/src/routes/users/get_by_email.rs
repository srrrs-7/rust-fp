use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

use application::user_service;
use domain::error::AppError;
use domain::user::inputs::GetUserByEmailInput;

use crate::response::{from_app_error, ErrorResponse};
use crate::routes::users::types::UserResponse;
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let user = user_service::get_user_by_email::get_user_by_email(
        state.user_repo.as_ref(),
        GetUserByEmailInput { email },
    )
    .await
    .map_err(from_app_error)?
    .ok_or_else(|| from_app_error(AppError::not_found("User", "User not found")))?;

    Ok(Json(UserResponse::from(user)))
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

        let result = handler(State(state), Path("user@example.com".to_string())).await;

        assert_status(result, StatusCode::OK);
    }

    #[tokio::test]
    async fn returns_not_found_when_missing() {
        let state = app_state(
            MockTaskRepo::default(),
            MockUserRepo::with_get_by_email_result(Ok(None)),
        );

        let result = handler(State(state), Path("user@example.com".to_string())).await;

        assert_status(result, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn returns_internal_server_error_on_repo_failure() {
        let state = app_state(
            MockTaskRepo::default(),
            MockUserRepo::with_get_by_email_result(Err(AppError::database("db error"))),
        );

        let result = handler(State(state), Path("user@example.com".to_string())).await;

        assert_status(result, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
