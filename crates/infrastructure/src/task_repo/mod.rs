use application::task_service::TaskRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::error::AppError;
use domain::task::{
    CreateTaskInput, DeleteTaskInput, GetTaskInput, ListTasksInput, Task, UpdateTaskInput,
};
use sqlx::PgPool;
use uuid::Uuid;

mod create_task;
mod delete_task;
mod get_task;
mod list_tasks;
mod update_task;

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
        create_task::create_task(self, input).await
    }

    async fn update_task(&self, input: UpdateTaskInput) -> Result<i64, AppError> {
        update_task::update_task(self, input).await
    }

    async fn delete_task(&self, input: DeleteTaskInput) -> Result<i64, AppError> {
        delete_task::delete_task(self, input).await
    }

    async fn get_task(&self, input: GetTaskInput) -> Result<Option<Task>, AppError> {
        get_task::get_task(self, input).await
    }

    async fn list_tasks(&self, input: ListTasksInput) -> Result<Vec<Task>, AppError> {
        list_tasks::list_tasks(self, input).await
    }
}
