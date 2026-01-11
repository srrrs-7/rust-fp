use domain::error::AppError;
use domain::task::entity::Task;
use domain::task::inputs::CreateTaskInput;

use super::repository::TaskRepository;

pub async fn create_task<R: TaskRepository + ?Sized>(
    repo: &R,
    input: CreateTaskInput,
) -> Result<Task, AppError> {
    repo.create_task(input).await
}
