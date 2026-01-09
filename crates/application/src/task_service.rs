use async_trait::async_trait;
use domain::error::AppError;
use domain::task::{
    CreateTaskInput, DeleteTaskInput, GetTaskInput, ListTasksInput, Task,
    UpdateTaskInput,
};

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create_task(&self, input: CreateTaskInput) -> Result<Task, AppError>;
    async fn update_task(&self, input: UpdateTaskInput) -> Result<i64, AppError>;
    async fn delete_task(&self, input: DeleteTaskInput) -> Result<i64, AppError>;
    async fn get_task(&self, input: GetTaskInput) -> Result<Option<Task>, AppError>;
    async fn list_tasks(&self, input: ListTasksInput) -> Result<Vec<Task>, AppError>;
}

pub async fn create_task<R: TaskRepository>(
    repo: &R,
    input: CreateTaskInput,
) -> Result<Task, AppError> {
    repo.create_task(input).await
}

pub async fn update_task<R: TaskRepository>(
    repo: &R,
    input: UpdateTaskInput,
) -> Result<i64, AppError> {
    repo.update_task(input).await
}

pub async fn delete_task<R: TaskRepository>(
    repo: &R,
    input: DeleteTaskInput,
) -> Result<i64, AppError> {
    repo.delete_task(input).await
}

pub async fn get_task<R: TaskRepository>(
    repo: &R,
    input: GetTaskInput,
) -> Result<Option<Task>, AppError> {
    repo.get_task(input).await
}

pub async fn list_tasks<R: TaskRepository>(
    repo: &R,
    input: ListTasksInput,
) -> Result<Vec<Task>, AppError> {
    repo.list_tasks(input).await
}
