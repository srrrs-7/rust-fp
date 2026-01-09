use domain::error::AppError;
use domain::task::{ListTasksInput, Task};

use super::{TaskRepositoryImpl, TaskRow};

pub async fn list_tasks(
    repo: &TaskRepositoryImpl,
    input: ListTasksInput,
) -> Result<Vec<Task>, AppError> {
    let offset = (input.page - 1).max(0) * input.limit;

    let rows = sqlx::query_as::<_, TaskRow>(
        r#"
            SELECT t.task_id, t.content, t.completed_at, t.version, t.created_at, t.updated_at
            FROM tasks t
            JOIN tasks_users tu ON t.task_id = tu.task_id
            WHERE tu.user_id = $1
            ORDER BY t.created_at DESC
            OFFSET $2
            LIMIT $3
            "#,
    )
    .bind(&input.user_id)
    .bind(offset)
    .bind(input.limit)
    .fetch_all(&repo.pool)
    .await
    .map_err(|error| AppError::database(error.to_string()))?;

    Ok(rows
        .into_iter()
        .map(|row| row.into_task(input.user_id.clone()))
        .collect())
}
