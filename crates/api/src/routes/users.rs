use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use application::user_service;
use domain::error::AppError;
use domain::user::{
    CreateUserInput, DeleteUserInput, GetUserByEmailInput, GetUserByUsernameInput,
    GetUserInput, ListUsersInput, UpdateUserInput,
};

use crate::response::{from_app_error, validation_error, ErrorResponse};
use crate::AppState;

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    user_id: String,
    client_id: String,
    username: String,
    email: String,
    name: Option<String>,
    picture: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateUserRequest {
    username: Option<String>,
    email: Option<String>,
    name: Option<Option<String>>,
    picture: Option<Option<String>>,
}

#[derive(Debug, Deserialize)]
struct ListUsersQuery {
    client_id: String,
    page: Option<i64>,
    limit: Option<i64>,
}

#[derive(Debug, Serialize)]
struct UserResponse {
    user_id: String,
    client_id: String,
    username: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct UserListResponse {
    users: Vec<UserResponse>,
    page: i64,
    limit: i64,
}

#[derive(Debug, Serialize)]
struct CountResponse {
    count: i64,
}

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/user", axum::routing::post(create_user))
        .route("/user/:id", axum::routing::get(get_user))
        .route("/user/:id", axum::routing::put(update_user))
        .route("/user/:id", axum::routing::delete(delete_user))
        .route("/users", axum::routing::get(list_users))
        .route("/users/by-email/:email", axum::routing::get(get_user_by_email))
        .route("/users/by-username/:username", axum::routing::get(get_user_by_username))
}

async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if body.user_id.trim().is_empty() || body.user_id.len() > 128 {
        return Err(validation_error("invalid_user_id", "User ID must be 1-128 characters"));
    }
    if body.client_id.trim().is_empty() {
        return Err(validation_error("invalid_client_id", "Client ID is required"));
    }
    if body.username.trim().is_empty() {
        return Err(validation_error("invalid_username", "Username is required"));
    }
    if body.email.trim().is_empty() {
        return Err(validation_error("invalid_email", "Email is required"));
    }

    let user = user_service::create_user(
        &state.user_repo,
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

    Ok((StatusCode::CREATED, Json(UserResponse {
        user_id: user.user_id,
        client_id: user.client_id,
        username: user.username,
        email: user.email,
    })))
}

async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(body): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if body.username.is_none() && body.email.is_none() && body.name.is_none() && body.picture.is_none()
    {
        return Err(validation_error(
            "invalid_body",
            "At least one field must be provided",
        ));
    }

    let user = user_service::update_user(
        &state.user_repo,
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

async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let count = user_service::delete_user(&state.user_repo, DeleteUserInput { user_id })
        .await
        .map_err(from_app_error)?;

    Ok(Json(CountResponse { count }))
}

async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let user = user_service::get_user(&state.user_repo, GetUserInput { user_id })
        .await
        .map_err(from_app_error)?
        .ok_or_else(|| from_app_error(AppError::not_found("User", "User not found")))?;

    Ok(Json(UserResponse {
        user_id: user.user_id,
        client_id: user.client_id,
        username: user.username,
        email: user.email,
    }))
}

async fn get_user_by_email(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let user = user_service::get_user_by_email(&state.user_repo, GetUserByEmailInput { email })
        .await
        .map_err(from_app_error)?
        .ok_or_else(|| from_app_error(AppError::not_found("User", "User not found")))?;

    Ok(Json(UserResponse {
        user_id: user.user_id,
        client_id: user.client_id,
        username: user.username,
        email: user.email,
    }))
}

async fn get_user_by_username(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let user = user_service::get_user_by_username(
        &state.user_repo,
        GetUserByUsernameInput { username },
    )
    .await
    .map_err(from_app_error)?
    .ok_or_else(|| from_app_error(AppError::not_found("User", "User not found")))?;

    Ok(Json(UserResponse {
        user_id: user.user_id,
        client_id: user.client_id,
        username: user.username,
        email: user.email,
    }))
}

async fn list_users(
    State(state): State<AppState>,
    Query(query): Query<ListUsersQuery>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if query.client_id.trim().is_empty() {
        return Err(validation_error("invalid_client_id", "Client ID is required"));
    }

    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).max(1).min(100);

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
