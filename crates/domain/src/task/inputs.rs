use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTaskInput {
    pub user_id: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTaskInput {
    pub user_id: String,
    pub task_id: String,
    pub content: Option<String>,
    pub completed_at: Option<Option<DateTime<Utc>>>,
    pub version: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTaskInput {
    pub user_id: String,
    pub task_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetTaskInput {
    pub user_id: String,
    pub task_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListTasksInput {
    pub user_id: String,
    pub page: i64,
    pub limit: i64,
}
