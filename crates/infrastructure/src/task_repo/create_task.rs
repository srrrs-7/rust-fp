use domain::error::AppError;
use domain::task::entity::Task;
use domain::task::inputs::CreateTaskInput;

use super::repository::{TaskRepositoryImpl, TaskRow};

pub async fn create_task(
    repo: &TaskRepositoryImpl,
    input: CreateTaskInput,
) -> Result<Task, AppError> {
    let mut tx = repo
        .pool
        .begin()
        .await
        .map_err(|error| AppError::database(error.to_string()))?;

    let row = sqlx::query_as::<_, TaskRow>(
        r#"
            INSERT INTO tasks (content, status, completed_at, version, created_at, updated_at)
            VALUES ($1, 'PENDING', NULL, 0, NOW(), NOW())
            RETURNING task_id, content, completed_at, version, created_at, updated_at
            "#,
    )
    .bind(&input.content)
    .fetch_one(&mut *tx)
    .await
    .map_err(|error| AppError::database(error.to_string()))?;

    sqlx::query(r#"INSERT INTO tasks_users (task_id, user_id) VALUES ($1, $2)"#)
        .bind(row.task_id)
        .bind(&input.user_id)
        .execute(&mut *tx)
        .await
        .map_err(|error| AppError::database(error.to_string()))?;

    tx.commit()
        .await
        .map_err(|error| AppError::database(error.to_string()))?;

    Ok(row.into_task(input.user_id))
}
