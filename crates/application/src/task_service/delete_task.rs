use domain::error::AppError;
use domain::task::DeleteTaskInput;

use super::TaskRepository;

pub async fn delete_task<R: TaskRepository + ?Sized>(
    repo: &R,
    input: DeleteTaskInput,
) -> Result<i64, AppError> {
    repo.delete_task(input).await
}
