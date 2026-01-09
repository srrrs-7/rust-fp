use domain::error::AppError;
use domain::task::UpdateTaskInput;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use super::TaskRepositoryImpl;

pub async fn update_task(
    repo: &TaskRepositoryImpl,
    input: UpdateTaskInput,
) -> Result<i64, AppError> {
    let task_id = Uuid::parse_str(&input.task_id)
        .map_err(|_| AppError::validation("invalid_task_id", "Invalid task id", None))?;

    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE tasks SET ");
    let mut separated = builder.separated(", ");

    if let Some(content) = &input.content {
        separated.push("content = ").push_bind(content);
    }
    if let Some(completed_at) = &input.completed_at {
        separated.push("completed_at = ").push_bind(completed_at);
    }

    separated.push("version = version + 1");
    separated.push("updated_at = NOW()");

    builder.push(" FROM tasks_users tu WHERE tasks.task_id = tu.task_id AND tu.user_id = ");
    builder.push_bind(&input.user_id);
    builder.push(" AND tasks.task_id = ");
    builder.push_bind(task_id);
    builder.push(" AND tasks.version = ");
    builder.push_bind(input.version);

    let result = builder
        .build()
        .execute(&repo.pool)
        .await
        .map_err(|error| AppError::database(error.to_string()))?;

    Ok(result.rows_affected() as i64)
}
