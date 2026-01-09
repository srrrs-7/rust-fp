use domain::error::AppError;
use domain::task::{CreateTaskInput, Task};

use super::TaskRepository;

pub async fn create_task<R: TaskRepository + ?Sized>(
    repo: &R,
    input: CreateTaskInput,
) -> Result<Task, AppError> {
    repo.create_task(input).await
}
