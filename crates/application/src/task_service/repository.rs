use async_trait::async_trait;
use domain::error::AppError;
use domain::task::entity::Task;
use domain::task::inputs::{
    CreateTaskInput, DeleteTaskInput, GetTaskInput, ListTasksInput, UpdateTaskInput,
};

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create_task(&self, input: CreateTaskInput) -> Result<Task, AppError>;
    async fn update_task(&self, input: UpdateTaskInput) -> Result<i64, AppError>;
    async fn delete_task(&self, input: DeleteTaskInput) -> Result<i64, AppError>;
    async fn get_task(&self, input: GetTaskInput) -> Result<Option<Task>, AppError>;
    async fn list_tasks(&self, input: ListTasksInput) -> Result<Vec<Task>, AppError>;
}
