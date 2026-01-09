use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub content: String,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub content: Option<String>,
    pub status: Option<String>,
    pub version: i32,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub task_id: String,
    pub user_id: String,
    pub content: String,
    pub completed_at: Option<String>,
    pub version: i32,
}

#[derive(Debug, Serialize)]
pub struct TaskListResponse {
    pub tasks: Vec<TaskResponse>,
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Serialize)]
pub struct CountResponse {
    pub count: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

pub fn parse_status(value: &str) -> Result<TaskStatus, ()> {
    match value {
        "PENDING" => Ok(TaskStatus::Pending),
        "IN_PROGRESS" => Ok(TaskStatus::InProgress),
        "COMPLETED" => Ok(TaskStatus::Completed),
        _ => Err(()),
    }
}

pub mod delete;
pub mod get;
pub mod list;
pub mod post;
pub mod put;
