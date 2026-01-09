use application::user_service::UserRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::error::AppError;
use domain::user::{
    CreateUserInput, DeleteUserInput, GetUserByEmailInput, GetUserByUsernameInput, GetUserInput,
    ListUsersInput, UpdateUserInput, User,
};
use sqlx::PgPool;

mod create_user;
mod delete_user;
mod get_user;
mod get_user_by_email;
mod get_user_by_username;
mod list_users;
mod update_user;

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct UserRow {
    user_id: String,
    client_id: String,
    username: String,
    email: String,
    name: Option<String>,
    picture: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UserRow {
    fn into_user(self) -> User {
        User {
            user_id: self.user_id,
            client_id: self.client_id,
            username: self.username,
            email: self.email,
            name: self.name,
            picture: self.picture,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

fn map_db_error(error: sqlx::Error) -> AppError {
    match &error {
        sqlx::Error::Database(db_error) => match db_error.code() {
            Some(code) if code == "23505" => AppError::conflict("User", db_error.message()),
            Some(code) if code == "23503" => AppError::conflict("User", db_error.message()),
            _ => AppError::database(db_error.message().to_string()),
        },
        _ => AppError::database(error.to_string()),
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(&self, input: CreateUserInput) -> Result<User, AppError> {
        create_user::create_user(self, input).await
    }

    async fn update_user(&self, input: UpdateUserInput) -> Result<User, AppError> {
        update_user::update_user(self, input).await
    }

    async fn delete_user(&self, input: DeleteUserInput) -> Result<i64, AppError> {
        delete_user::delete_user(self, input).await
    }

    async fn get_user(&self, input: GetUserInput) -> Result<Option<User>, AppError> {
        get_user::get_user(self, input).await
    }

    async fn get_user_by_email(
        &self,
        input: GetUserByEmailInput,
    ) -> Result<Option<User>, AppError> {
        get_user_by_email::get_user_by_email(self, input).await
    }

    async fn get_user_by_username(
        &self,
        input: GetUserByUsernameInput,
    ) -> Result<Option<User>, AppError> {
        get_user_by_username::get_user_by_username(self, input).await
    }

    async fn list_users(&self, input: ListUsersInput) -> Result<Vec<User>, AppError> {
        list_users::list_users(self, input).await
    }
}
