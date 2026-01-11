use async_trait::async_trait;
use domain::error::AppError;
use domain::user::entity::User;
use domain::user::inputs::{
    CreateUserInput, DeleteUserInput, GetUserByEmailInput, GetUserByUsernameInput, GetUserInput,
    ListUsersInput, UpdateUserInput,
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
