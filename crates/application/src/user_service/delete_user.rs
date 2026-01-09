use domain::error::AppError;
use domain::user::DeleteUserInput;

use super::UserRepository;

pub async fn delete_user<R: UserRepository + ?Sized>(
    repo: &R,
    input: DeleteUserInput,
) -> Result<i64, AppError> {
    repo.delete_user(input).await
}
