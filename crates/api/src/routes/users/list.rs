use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;

use application::user_service;
use domain::user::inputs::ListUsersInput;

use crate::response::{from_app_error, validation_error, ErrorResponse};
use crate::routes::users::types::{ListUsersQuery, UserListResponse, UserResponse};
use crate::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Query(query): Query<ListUsersQuery>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if query.client_id.trim().is_empty() {
        return Err(validation_error(
            "invalid_client_id",
            "Client ID is required",
        ));
    }

    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).clamp(1, 100);

    let users = user_service::list_users::list_users(
        state.user_repo.as_ref(),
        ListUsersInput {
            client_id: query.client_id,
            page: Some(page),
            limit: Some(limit),
        },
    )
    .await
    .map_err(from_app_error)?;

    let response = UserListResponse {
        users: users.into_iter().map(UserResponse::from).collect(),
        page,
        limit,
    };

    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use axum::extract::{Query, State};
    use axum::http::StatusCode;
    use domain::error::AppError;

    use super::handler;
    use crate::routes::test_support::{app_state, assert_status, MockTaskRepo, MockUserRepo};
    use crate::routes::users::types::ListUsersQuery;

    #[tokio::test]
    async fn returns_bad_request_for_missing_client_id() {
        let state = app_state(MockTaskRepo::default(), MockUserRepo::default());
        let query = ListUsersQuery {
            client_id: " ".to_string(),
            page: None,
            limit: None,
        };

        let result = handler(State(state), Query(query)).await;

        assert_status(result, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn returns_ok_on_success() {
        let state = app_state(MockTaskRepo::default(), MockUserRepo::default());
        let query = ListUsersQuery {
            client_id: "client-1".to_string(),
            page: Some(1),
            limit: Some(10),
        };

        let result = handler(State(state), Query(query)).await;

        assert_status(result, StatusCode::OK);
    }

    #[tokio::test]
    async fn returns_internal_server_error_on_repo_failure() {
        let state = app_state(
            MockTaskRepo::default(),
            MockUserRepo::with_list_result(Err(AppError::database("db error"))),
        );
        let query = ListUsersQuery {
            client_id: "client-1".to_string(),
            page: None,
            limit: None,
        };

        let result = handler(State(state), Query(query)).await;

        assert_status(result, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
