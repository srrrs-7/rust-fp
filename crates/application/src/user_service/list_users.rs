use domain::error::AppError;
use domain::user::{ListUsersInput, User};

use super::UserRepository;

pub async fn list_users<R: UserRepository + ?Sized>(
    repo: &R,
    input: ListUsersInput,
) -> Result<Vec<User>, AppError> {
    repo.list_users(input).await
}
