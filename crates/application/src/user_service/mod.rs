use async_trait::async_trait;
use domain::error::AppError;
use domain::user::{
    CreateUserInput, DeleteUserInput, GetUserByEmailInput, GetUserByUsernameInput, GetUserInput,
    ListUsersInput, UpdateUserInput, User,
};

mod create_user;
mod delete_user;
mod get_user;
mod get_user_by_email;
mod get_user_by_username;
mod list_users;
mod update_user;

pub use create_user::create_user;
pub use delete_user::delete_user;
pub use get_user::get_user;
pub use get_user_by_email::get_user_by_email;
pub use get_user_by_username::get_user_by_username;
pub use list_users::list_users;
pub use update_user::update_user;

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
