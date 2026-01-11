use domain::error::AppError;
use domain::user::entity::User;
use domain::user::inputs::GetUserByEmailInput;

use super::repository::UserRepository;

pub async fn get_user_by_email<R: UserRepository + ?Sized>(
    repo: &R,
    input: GetUserByEmailInput,
) -> Result<Option<User>, AppError> {
    repo.get_user_by_email(input).await
}
