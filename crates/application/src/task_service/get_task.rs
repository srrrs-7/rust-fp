use domain::error::AppError;
use domain::task::{GetTaskInput, Task};

use super::TaskRepository;

pub async fn get_task<R: TaskRepository + ?Sized>(
    repo: &R,
    input: GetTaskInput,
) -> Result<Option<Task>, AppError> {
    repo.get_task(input).await
}
