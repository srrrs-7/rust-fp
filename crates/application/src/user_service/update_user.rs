use domain::error::AppError;
use domain::user::{UpdateUserInput, User};

use super::UserRepository;

pub async fn update_user<R: UserRepository + ?Sized>(
    repo: &R,
    input: UpdateUserInput,
) -> Result<User, AppError> {
    repo.update_user(input).await
}
