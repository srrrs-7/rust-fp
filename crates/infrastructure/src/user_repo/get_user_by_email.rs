use domain::error::AppError;
use domain::user::entity::User;
use domain::user::inputs::GetUserByEmailInput;

use super::repository::{map_db_error, UserRepositoryImpl, UserRow};

pub async fn get_user_by_email(
    repo: &UserRepositoryImpl,
    input: GetUserByEmailInput,
) -> Result<Option<User>, AppError> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
            SELECT user_id, client_id, username, email, name, picture, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
    )
    .bind(&input.email)
    .fetch_optional(&repo.pool)
    .await
    .map_err(map_db_error)?;

    Ok(row.map(|row| row.into_user()))
}
