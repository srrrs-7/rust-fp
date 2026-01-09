use domain::error::AppError;
use domain::user::{CreateUserInput, User};

use super::UserRepository;

pub async fn create_user<R: UserRepository + ?Sized>(
    repo: &R,
    input: CreateUserInput,
) -> Result<User, AppError> {
    repo.create_user(input).await
}
