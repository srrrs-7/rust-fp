use async_trait::async_trait;
use domain::error::AppError;
use domain::user::{
    CreateUserInput, DeleteUserInput, GetUserByEmailInput, GetUserByUsernameInput,
    GetUserInput, ListUsersInput, UpdateUserInput, User,
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, input: CreateUserInput) -> Result<User, AppError>;
    async fn update_user(&self, input: UpdateUserInput) -> Result<User, AppError>;
    async fn delete_user(&self, input: DeleteUserInput) -> Result<i64, AppError>;
    async fn get_user(&self, input: GetUserInput) -> Result<Option<User>, AppError>;
    async fn get_user_by_email(&self, input: GetUserByEmailInput)
        -> Result<Option<User>, AppError>;
    async fn get_user_by_username(
        &self,
        input: GetUserByUsernameInput,
    ) -> Result<Option<User>, AppError>;
    async fn list_users(&self, input: ListUsersInput) -> Result<Vec<User>, AppError>;
}

pub async fn create_user<R: UserRepository>(
    repo: &R,
    input: CreateUserInput,
) -> Result<User, AppError> {
    repo.create_user(input).await
}

pub async fn update_user<R: UserRepository>(
    repo: &R,
    input: UpdateUserInput,
) -> Result<User, AppError> {
    repo.update_user(input).await
}

pub async fn delete_user<R: UserRepository>(
    repo: &R,
    input: DeleteUserInput,
) -> Result<i64, AppError> {
    repo.delete_user(input).await
}

pub async fn get_user<R: UserRepository>(
    repo: &R,
    input: GetUserInput,
) -> Result<Option<User>, AppError> {
    repo.get_user(input).await
}

pub async fn get_user_by_email<R: UserRepository>(
    repo: &R,
    input: GetUserByEmailInput,
) -> Result<Option<User>, AppError> {
    repo.get_user_by_email(input).await
}

pub async fn get_user_by_username<R: UserRepository>(
    repo: &R,
    input: GetUserByUsernameInput,
) -> Result<Option<User>, AppError> {
    repo.get_user_by_username(input).await
}

pub async fn list_users<R: UserRepository>(
    repo: &R,
    input: ListUsersInput,
) -> Result<Vec<User>, AppError> {
    repo.list_users(input).await
}
