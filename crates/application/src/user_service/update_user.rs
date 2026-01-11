use domain::error::AppError;
use domain::user::entity::User;
use domain::user::inputs::UpdateUserInput;

use super::repository::UserRepository;

pub async fn update_user<R: UserRepository + ?Sized>(
    repo: &R,
    input: UpdateUserInput,
) -> Result<User, AppError> {
    repo.update_user(input).await
}
