use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;

use application::user_service;
use domain::user::ListUsersInput;

use crate::response::{from_app_error, validation_error, ErrorResponse};
use crate::routes::users::{ListUsersQuery, UserListResponse, UserResponse};
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

    let users = user_service::list_users(
        &state.user_repo,
        ListUsersInput {
            client_id: query.client_id,
            page: Some(page),
            limit: Some(limit),
        },
    )
    .await
    .map_err(from_app_error)?;

    let response = UserListResponse {
        users: users
            .into_iter()
            .map(|user| UserResponse {
                user_id: user.user_id,
                client_id: user.client_id,
                username: user.username,
                email: user.email,
            })
            .collect(),
        page,
        limit,
    };

    Ok(Json(response))
}
