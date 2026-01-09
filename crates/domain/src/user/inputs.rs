use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserInput {
    pub user_id: String,
    pub client_id: String,
    pub username: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUserInput {
    pub user_id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub name: Option<Option<String>>,
    pub picture: Option<Option<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUserInput {
    pub user_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetUserInput {
    pub user_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetUserByEmailInput {
    pub email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetUserByUsernameInput {
    pub username: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListUsersInput {
    pub client_id: String,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}
