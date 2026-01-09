use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

use application::user_service;
use domain::user::UpdateUserInput;

use crate::response::{from_app_error, validation_error, ErrorResponse};
use crate::routes::users::{UpdateUserRequest, UserResponse};
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(body): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if body.username.is_none()
        && body.email.is_none()
        && body.name.is_none()
        && body.picture.is_none()
    {
        return Err(validation_error(
            "invalid_body",
            "At least one field must be provided",
        ));
    }

    let user = user_service::update_user(
        state.user_repo.as_ref(),
        UpdateUserInput {
            user_id,
            username: body.username,
            email: body.email,
            name: body.name,
            picture: body.picture,
        },
    )
    .await
    .map_err(from_app_error)?;

    Ok(Json(UserResponse {
        user_id: user.user_id,
        client_id: user.client_id,
        username: user.username,
        email: user.email,
    }))
}

#[cfg(test)]
mod tests {
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use axum::Json;
    use domain::error::AppError;

    use super::handler;
    use crate::routes::test_support::{app_state, assert_status, MockTaskRepo, MockUserRepo};
    use crate::routes::users::UpdateUserRequest;

    #[tokio::test]
    async fn returns_bad_request_for_empty_body() {
        let state = app_state(MockTaskRepo::default(), MockUserRepo::default());
        let body = UpdateUserRequest {
            username: None,
            email: None,
            name: None,
            picture: None,
        };

        let result = handler(State(state), Path("user-1".to_string()), Json(body)).await;

        assert_status(result, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn returns_ok_on_success() {
        let state = app_state(MockTaskRepo::default(), MockUserRepo::default());
        let body = UpdateUserRequest {
            username: Some("user2".to_string()),
            email: None,
            name: None,
            picture: None,
        };

        let result = handler(State(state), Path("user-1".to_string()), Json(body)).await;

        assert_status(result, StatusCode::OK);
    }

    #[tokio::test]
    async fn returns_internal_server_error_on_repo_failure() {
        let state = app_state(
            MockTaskRepo::default(),
            MockUserRepo::with_update_result(Err(AppError::database("db error"))),
        );
        let body = UpdateUserRequest {
            username: Some("user2".to_string()),
            email: None,
            name: None,
            picture: None,
        };

        let result = handler(State(state), Path("user-1".to_string()), Json(body)).await;

        assert_status(result, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
