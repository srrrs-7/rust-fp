use domain::error::AppError;
use domain::task::entity::Task;
use domain::task::inputs::GetTaskInput;

use super::repository::TaskRepository;

pub async fn get_task<R: TaskRepository + ?Sized>(
    repo: &R,
    input: GetTaskInput,
) -> Result<Option<Task>, AppError> {
    repo.get_task(input).await
}
