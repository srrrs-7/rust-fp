use domain::error::AppError;
use domain::user::entity::User;
use domain::user::inputs::ListUsersInput;

use super::repository::{map_db_error, UserRepositoryImpl, UserRow};

pub async fn list_users(
    repo: &UserRepositoryImpl,
    input: ListUsersInput,
) -> Result<Vec<User>, AppError> {
    let page = input.page.unwrap_or(1).max(1);
    let limit = input.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let rows = sqlx::query_as::<_, UserRow>(
        r#"
            SELECT user_id, client_id, username, email, name, picture, created_at, updated_at
            FROM users
            WHERE client_id = $1
            ORDER BY created_at DESC
            OFFSET $2
            LIMIT $3
            "#,
    )
    .bind(&input.client_id)
    .bind(offset)
    .bind(limit)
    .fetch_all(&repo.pool)
    .await
    .map_err(map_db_error)?;

    Ok(rows.into_iter().map(|row| row.into_user()).collect())
}
