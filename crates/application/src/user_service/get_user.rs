use domain::error::AppError;
use domain::user::{GetUserInput, User};

use super::UserRepository;

pub async fn get_user<R: UserRepository + ?Sized>(
    repo: &R,
    input: GetUserInput,
) -> Result<Option<User>, AppError> {
    repo.get_user(input).await
}
