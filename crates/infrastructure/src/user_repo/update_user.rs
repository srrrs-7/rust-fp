use domain::error::AppError;
use domain::user::entity::User;
use domain::user::inputs::UpdateUserInput;
use sqlx::{Postgres, QueryBuilder};

use super::repository::{map_db_error, UserRepositoryImpl, UserRow};

pub async fn update_user(
    repo: &UserRepositoryImpl,
    input: UpdateUserInput,
) -> Result<User, AppError> {
    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE users SET ");
    let mut separated = builder.separated(", ");

    if let Some(username) = &input.username {
        separated.push("username = ").push_bind(username);
    }
    if let Some(email) = &input.email {
        separated.push("email = ").push_bind(email);
    }
    if let Some(name) = &input.name {
        separated.push("name = ").push_bind(name);
    }
    if let Some(picture) = &input.picture {
        separated.push("picture = ").push_bind(picture);
    }

    separated.push("updated_at = NOW()");

    builder.push(" WHERE user_id = ");
    builder.push_bind(&input.user_id);
    builder.push(
        " RETURNING user_id, client_id, username, email, name, picture, created_at, updated_at",
    );

    let row = builder
        .build_query_as::<UserRow>()
        .fetch_optional(&repo.pool)
        .await
        .map_err(map_db_error)?;

    row.map(|row| row.into_user())
        .ok_or_else(|| AppError::not_found("User", "User not found"))
}
