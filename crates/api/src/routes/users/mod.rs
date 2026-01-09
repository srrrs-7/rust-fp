use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub user_id: String,
    pub client_id: String,
    pub username: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub name: Option<Option<String>>,
    pub picture: Option<Option<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    pub client_id: String,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user_id: String,
    pub client_id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Serialize)]
pub struct CountResponse {
    pub count: i64,
}

pub mod delete;
pub mod get;
pub mod get_by_email;
pub mod get_by_username;
pub mod list;
pub mod post;
pub mod put;
