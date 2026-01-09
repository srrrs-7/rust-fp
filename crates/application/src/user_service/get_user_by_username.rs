use domain::error::AppError;
use domain::user::{GetUserByUsernameInput, User};

use super::UserRepository;

pub async fn get_user_by_username<R: UserRepository + ?Sized>(
    repo: &R,
    input: GetUserByUsernameInput,
) -> Result<Option<User>, AppError> {
    repo.get_user_by_username(input).await
}
