use domain::error::AppError;
use domain::user::entity::User;
use domain::user::inputs::CreateUserInput;

use super::repository::UserRepository;

pub async fn create_user<R: UserRepository + ?Sized>(
    repo: &R,
    input: CreateUserInput,
) -> Result<User, AppError> {
    repo.create_user(input).await
}
