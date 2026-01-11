use domain::error::AppError;
use domain::user::entity::User;
use domain::user::inputs::ListUsersInput;

use super::repository::UserRepository;

pub async fn list_users<R: UserRepository + ?Sized>(
    repo: &R,
    input: ListUsersInput,
) -> Result<Vec<User>, AppError> {
    repo.list_users(input).await
}
