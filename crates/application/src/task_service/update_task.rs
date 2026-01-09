use domain::error::AppError;
use domain::task::UpdateTaskInput;

use super::TaskRepository;

pub async fn update_task<R: TaskRepository + ?Sized>(
    repo: &R,
    input: UpdateTaskInput,
) -> Result<i64, AppError> {
    repo.update_task(input).await
}
