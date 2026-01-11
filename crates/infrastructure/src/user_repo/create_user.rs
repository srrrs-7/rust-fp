use domain::error::AppError;
use domain::user::entity::User;
use domain::user::inputs::CreateUserInput;

use super::repository::{map_db_error, UserRepositoryImpl, UserRow};

pub async fn create_user(
    repo: &UserRepositoryImpl,
    input: CreateUserInput,
) -> Result<User, AppError> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
            INSERT INTO users (user_id, client_id, username, email, name, picture, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
            RETURNING user_id, client_id, username, email, name, picture, created_at, updated_at
            "#,
    )
    .bind(&input.user_id)
    .bind(&input.client_id)
    .bind(&input.username)
    .bind(&input.email)
    .bind(&input.name)
    .bind(&input.picture)
    .fetch_one(&repo.pool)
    .await
    .map_err(map_db_error)?;

    Ok(row.into_user())
}
