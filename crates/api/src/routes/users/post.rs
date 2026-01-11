use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use application::user_service;
use domain::user::inputs::CreateUserInput;

use crate::response::{from_app_error, validation_error, ErrorResponse};
use crate::routes::users::types::{CreateUserRequest, UserResponse};
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if body.user_id.trim().is_empty() || body.user_id.len() > 128 {
        return Err(validation_error(
            "invalid_user_id",
            "User ID must be 1-128 characters",
        ));
    }
    if body.client_id.trim().is_empty() {
        return Err(validation_error(
            "invalid_client_id",
            "Client ID is required",
        ));
    }
    if body.username.trim().is_empty() {
        return Err(validation_error("invalid_username", "Username is required"));
    }
    if body.email.trim().is_empty() {
        return Err(validation_error("invalid_email", "Email is required"));
    }

    let user = user_service::create_user::create_user(
        state.user_repo.as_ref(),
        CreateUserInput {
            user_id: body.user_id,
            client_id: body.client_id,
            username: body.username,
            email: body.email,
            name: body.name,
            picture: body.picture,
        },
    )
    .await
    .map_err(from_app_error)?;

    Ok((
        StatusCode::CREATED,
        Json(UserResponse::from(user)),
    ))
}

#[cfg(test)]
mod tests {
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::Json;
    use domain::error::AppError;

    use super::handler;
    use crate::routes::test_support::{app_state, assert_status, MockTaskRepo, MockUserRepo};
    use crate::routes::users::types::CreateUserRequest;

    #[tokio::test]
    async fn returns_bad_request_for_missing_fields() {
        let state = app_state(MockTaskRepo::default(), MockUserRepo::default());
        let body = CreateUserRequest {
            user_id: "".to_string(),
            client_id: "".to_string(),
            username: "".to_string(),
            email: "".to_string(),
            name: None,
            picture: None,
        };

        let result = handler(State(state), Json(body)).await;

        assert_status(result, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn returns_created_on_success() {
        let state = app_state(MockTaskRepo::default(), MockUserRepo::default());
        let body = CreateUserRequest {
            user_id: "user-1".to_string(),
            client_id: "client-1".to_string(),
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
            name: None,
            picture: None,
        };

        let result = handler(State(state), Json(body)).await;

        assert_status(result, StatusCode::CREATED);
    }

    #[tokio::test]
    async fn returns_internal_server_error_on_repo_failure() {
        let state = app_state(
            MockTaskRepo::default(),
            MockUserRepo::with_create_result(Err(AppError::database("db error"))),
        );
        let body = CreateUserRequest {
            user_id: "user-1".to_string(),
            client_id: "client-1".to_string(),
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
            name: None,
            picture: None,
        };

        let result = handler(State(state), Json(body)).await;

        assert_status(result, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
