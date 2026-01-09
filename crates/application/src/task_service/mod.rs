use async_trait::async_trait;
use domain::error::AppError;
use domain::task::{
    CreateTaskInput, DeleteTaskInput, GetTaskInput, ListTasksInput, Task, UpdateTaskInput,
};

mod create_task;
mod delete_task;
mod get_task;
mod list_tasks;
mod update_task;

pub use create_task::create_task;
pub use delete_task::delete_task;
pub use get_task::get_task;
pub use list_tasks::list_tasks;
pub use update_task::update_task;

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create_task(&self, input: CreateTaskInput) -> Result<Task, AppError>;
    async fn update_task(&self, input: UpdateTaskInput) -> Result<i64, AppError>;
    async fn delete_task(&self, input: DeleteTaskInput) -> Result<i64, AppError>;
    async fn get_task(&self, input: GetTaskInput) -> Result<Option<Task>, AppError>;
    async fn list_tasks(&self, input: ListTasksInput) -> Result<Vec<Task>, AppError>;
}
