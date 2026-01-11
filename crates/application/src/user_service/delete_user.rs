use domain::error::AppError;
use domain::user::inputs::DeleteUserInput;

use super::repository::UserRepository;

pub async fn delete_user<R: UserRepository + ?Sized>(
    repo: &R,
    input: DeleteUserInput,
) -> Result<i64, AppError> {
    repo.delete_user(input).await
}
