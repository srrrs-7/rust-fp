use domain::error::AppError;
use domain::task::{ListTasksInput, Task};

use super::TaskRepository;

pub async fn list_tasks<R: TaskRepository + ?Sized>(
    repo: &R,
    input: ListTasksInput,
) -> Result<Vec<Task>, AppError> {
    repo.list_tasks(input).await
}
