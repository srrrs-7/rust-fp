use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub user_id: String,
    pub task_id: String,
    pub content: String,
    pub completed_at: Option<DateTime<Utc>>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

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
