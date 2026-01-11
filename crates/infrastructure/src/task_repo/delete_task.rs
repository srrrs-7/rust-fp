use domain::error::AppError;
use domain::task::inputs::DeleteTaskInput;
use uuid::Uuid;

use super::repository::TaskRepositoryImpl;

pub async fn delete_task(
    repo: &TaskRepositoryImpl,
    input: DeleteTaskInput,
) -> Result<i64, AppError> {
    let task_id = Uuid::parse_str(&input.task_id)
        .map_err(|_| AppError::validation("invalid_task_id", "Invalid task id", None))?;

    let result = sqlx::query(
        r#"
            DELETE FROM tasks
            USING tasks_users tu
            WHERE tasks.task_id = tu.task_id
              AND tu.user_id = $1
              AND tasks.task_id = $2
            "#,
    )
    .bind(&input.user_id)
    .bind(task_id)
    .execute(&repo.pool)
    .await
    .map_err(|error| AppError::database(error.to_string()))?;

    Ok(result.rows_affected() as i64)
}
