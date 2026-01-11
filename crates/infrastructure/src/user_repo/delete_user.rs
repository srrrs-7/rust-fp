use domain::error::AppError;
use domain::user::inputs::DeleteUserInput;

use super::repository::{map_db_error, UserRepositoryImpl};

pub async fn delete_user(
    repo: &UserRepositoryImpl,
    input: DeleteUserInput,
) -> Result<i64, AppError> {
    let result = sqlx::query("DELETE FROM users WHERE user_id = $1")
        .bind(&input.user_id)
        .execute(&repo.pool)
        .await
        .map_err(map_db_error)?;

    if result.rows_affected() == 0 {
        return Err(AppError::not_found("User", "User not found"));
    }

    Ok(result.rows_affected() as i64)
}
