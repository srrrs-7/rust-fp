use domain::error::AppError;
use domain::task::{GetTaskInput, Task};
use uuid::Uuid;

use super::{TaskRepositoryImpl, TaskRow};

pub async fn get_task(
    repo: &TaskRepositoryImpl,
    input: GetTaskInput,
) -> Result<Option<Task>, AppError> {
    let task_id = Uuid::parse_str(&input.task_id)
        .map_err(|_| AppError::validation("invalid_task_id", "Invalid task id", None))?;

    let row = sqlx::query_as::<_, TaskRow>(
        r#"
            SELECT t.task_id, t.content, t.completed_at, t.version, t.created_at, t.updated_at
            FROM tasks t
            JOIN tasks_users tu ON t.task_id = tu.task_id
            WHERE tu.user_id = $1 AND t.task_id = $2
            "#,
    )
    .bind(&input.user_id)
    .bind(task_id)
    .fetch_optional(&repo.pool)
    .await
    .map_err(|error| AppError::database(error.to_string()))?;

    Ok(row.map(|row| row.into_task(input.user_id)))
}
