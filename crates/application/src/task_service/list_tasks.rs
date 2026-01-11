use domain::error::AppError;
use domain::task::entity::Task;
use domain::task::inputs::ListTasksInput;

use super::repository::TaskRepository;

pub async fn list_tasks<R: TaskRepository + ?Sized>(
    repo: &R,
    input: ListTasksInput,
) -> Result<Vec<Task>, AppError> {
    repo.list_tasks(input).await
}
