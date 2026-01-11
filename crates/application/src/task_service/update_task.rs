use domain::error::AppError;
use domain::task::inputs::UpdateTaskInput;

use super::repository::TaskRepository;

pub async fn update_task<R: TaskRepository + ?Sized>(
    repo: &R,
    input: UpdateTaskInput,
) -> Result<i64, AppError> {
    repo.update_task(input).await
}
