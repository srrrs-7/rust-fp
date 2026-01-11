use domain::error::AppError;
use domain::user::entity::User;
use domain::user::inputs::GetUserInput;

use super::repository::{map_db_error, UserRepositoryImpl, UserRow};

pub async fn get_user(
    repo: &UserRepositoryImpl,
    input: GetUserInput,
) -> Result<Option<User>, AppError> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
            SELECT user_id, client_id, username, email, name, picture, created_at, updated_at
            FROM users
            WHERE user_id = $1
            "#,
    )
    .bind(&input.user_id)
    .fetch_optional(&repo.pool)
    .await
    .map_err(map_db_error)?;

    Ok(row.map(|row| row.into_user()))
}
