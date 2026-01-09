use application::task_service::TaskRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::error::AppError;
use domain::task::{
    CreateTaskInput, DeleteTaskInput, GetTaskInput, ListTasksInput, Task,
    UpdateTaskInput,
};
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

#[derive(Clone)]
pub struct TaskRepositoryImpl {
    pool: PgPool,
}

impl TaskRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct TaskRow {
    task_id: Uuid,
    content: String,
    completed_at: Option<DateTime<Utc>>,
    version: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TaskRow {
    fn into_task(self, user_id: String) -> Task {
        Task {
            user_id,
            task_id: self.task_id.to_string(),
            content: self.content,
            completed_at: self.completed_at,
            version: self.version,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[async_trait]
impl TaskRepository for TaskRepositoryImpl {
    async fn create_task(&self, input: CreateTaskInput) -> Result<Task, AppError> {
        let mut tx = self
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

        sqlx::query(
            r#"INSERT INTO "_TasksToUser" ("A", "B") VALUES ($1, $2)"#,
        )
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

    async fn update_task(&self, input: UpdateTaskInput) -> Result<i64, AppError> {
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

        builder.push(
            " FROM \"_TasksToUser\" tu WHERE tasks.task_id = tu.\"A\" AND tu.\"B\" = ",
        );
        builder.push_bind(&input.user_id);
        builder.push(" AND tasks.task_id = ");
        builder.push_bind(task_id);
        builder.push(" AND tasks.version = ");
        builder.push_bind(input.version);

        let result = builder
            .build()
            .execute(&self.pool)
            .await
            .map_err(|error| AppError::database(error.to_string()))?;

        Ok(result.rows_affected() as i64)
    }

    async fn delete_task(&self, input: DeleteTaskInput) -> Result<i64, AppError> {
        let task_id = Uuid::parse_str(&input.task_id)
            .map_err(|_| AppError::validation("invalid_task_id", "Invalid task id", None))?;

        let result = sqlx::query(
            r#"
            DELETE FROM tasks
            USING "_TasksToUser" tu
            WHERE tasks.task_id = tu."A"
              AND tu."B" = $1
              AND tasks.task_id = $2
            "#,
        )
        .bind(&input.user_id)
        .bind(task_id)
        .execute(&self.pool)
        .await
        .map_err(|error| AppError::database(error.to_string()))?;

        Ok(result.rows_affected() as i64)
    }

    async fn get_task(&self, input: GetTaskInput) -> Result<Option<Task>, AppError> {
        let task_id = Uuid::parse_str(&input.task_id)
            .map_err(|_| AppError::validation("invalid_task_id", "Invalid task id", None))?;

        let row = sqlx::query_as::<_, TaskRow>(
            r#"
            SELECT t.task_id, t.content, t.completed_at, t.version, t.created_at, t.updated_at
            FROM tasks t
            JOIN "_TasksToUser" tu ON t.task_id = tu."A"
            WHERE tu."B" = $1 AND t.task_id = $2
            "#,
        )
        .bind(&input.user_id)
        .bind(task_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| AppError::database(error.to_string()))?;

        Ok(row.map(|row| row.into_task(input.user_id)))
    }

    async fn list_tasks(&self, input: ListTasksInput) -> Result<Vec<Task>, AppError> {
        let offset = (input.page - 1).max(0) * input.limit;

        let rows = sqlx::query_as::<_, TaskRow>(
            r#"
            SELECT t.task_id, t.content, t.completed_at, t.version, t.created_at, t.updated_at
            FROM tasks t
            JOIN "_TasksToUser" tu ON t.task_id = tu."A"
            WHERE tu."B" = $1
            ORDER BY t.created_at DESC
            OFFSET $2
            LIMIT $3
            "#,
        )
        .bind(&input.user_id)
        .bind(offset)
        .bind(input.limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|error| AppError::database(error.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| row.into_task(input.user_id.clone()))
            .collect())
    }
}
