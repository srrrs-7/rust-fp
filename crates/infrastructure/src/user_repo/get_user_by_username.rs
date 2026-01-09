use domain::error::AppError;
use domain::user::{GetUserByUsernameInput, User};

use super::{map_db_error, UserRepositoryImpl, UserRow};

pub async fn get_user_by_username(
    repo: &UserRepositoryImpl,
    input: GetUserByUsernameInput,
) -> Result<Option<User>, AppError> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
            SELECT user_id, client_id, username, email, name, picture, created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
    )
    .bind(&input.username)
    .fetch_optional(&repo.pool)
    .await
    .map_err(map_db_error)?;

    Ok(row.map(|row| row.into_user()))
}
