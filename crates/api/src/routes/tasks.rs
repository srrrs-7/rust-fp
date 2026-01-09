use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use application::task_service;
use domain::error::AppError;
use domain::task::{
    CreateTaskInput, DeleteTaskInput, GetTaskInput, ListTasksInput, UpdateTaskInput,
};

use crate::middleware::cognito_auth::AuthUser;
use crate::response::{from_app_error, validation_error, ErrorResponse};
use crate::AppState;

#[derive(Debug, Deserialize)]
struct Pagination {
    page: Option<i64>,
    limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct CreateTaskRequest {
    content: String,
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateTaskRequest {
    content: Option<String>,
    status: Option<String>,
    version: i32,
}

#[derive(Debug, Serialize)]
struct TaskResponse {
    task_id: String,
    user_id: String,
    content: String,
    completed_at: Option<String>,
    version: i32,
}

#[derive(Debug, Serialize)]
struct TaskListResponse {
    tasks: Vec<TaskResponse>,
    page: i64,
    limit: i64,
}

#[derive(Debug, Serialize)]
struct CountResponse {
    count: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/task", axum::routing::post(create_task))
        .route("/task/:id", axum::routing::get(get_task))
        .route("/task/:id", axum::routing::put(update_task))
        .route("/task/:id", axum::routing::delete(delete_task))
        .route("/tasks", axum::routing::get(list_tasks))
}

async fn create_task(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<AuthUser>,
    Json(body): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if body.content.trim().is_empty() || body.content.len() > 1000 {
        return Err(validation_error("invalid_content", "Content must be 1-1000 characters"));
    }

    if let Some(status) = &body.status {
        if parse_status(status).is_err() {
            return Err(validation_error("invalid_status", "Status must be PENDING, IN_PROGRESS, or COMPLETED"));
        }
    }

    let task = task_service::create_task(
        &state.task_repo,
        CreateTaskInput {
            user_id: user.user_id,
            content: body.content,
        },
    )
    .await
    .map_err(from_app_error)?;

    Ok(Json(TaskResponse {
        task_id: task.task_id,
        user_id: task.user_id,
        content: task.content,
        completed_at: task.completed_at.map(|dt| dt.to_rfc3339()),
        version: task.version,
    }))
}

async fn get_task(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<AuthUser>,
    Path(task_id): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let task = task_service::get_task(
        &state.task_repo,
        GetTaskInput {
            user_id: user.user_id,
            task_id,
        },
    )
    .await
    .map_err(from_app_error)?
    .ok_or_else(|| from_app_error(AppError::not_found("Task", "Task not found")))?;

    Ok(Json(TaskResponse {
        task_id: task.task_id,
        user_id: task.user_id,
        content: task.content,
        completed_at: task.completed_at.map(|dt| dt.to_rfc3339()),
        version: task.version,
    }))
}

async fn list_tasks(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<AuthUser>,
    Query(params): Query<Pagination>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).max(1).min(100);

    let tasks = task_service::list_tasks(
        &state.task_repo,
        ListTasksInput {
            user_id: user.user_id,
            page,
            limit,
        },
    )
    .await
    .map_err(from_app_error)?;

    let response = TaskListResponse {
        tasks: tasks
            .into_iter()
            .map(|task| TaskResponse {
                task_id: task.task_id,
                user_id: task.user_id,
                content: task.content,
                completed_at: task.completed_at.map(|dt| dt.to_rfc3339()),
                version: task.version,
            })
            .collect(),
        page,
        limit,
    };

    Ok(Json(response))
}

async fn update_task(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<AuthUser>,
    Path(task_id): Path<String>,
    Json(body): Json<UpdateTaskRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if body.content.is_none() && body.status.is_none() {
        return Err(validation_error(
            "invalid_body",
            "At least one field (content or status) must be provided",
        ));
    }

    if let Some(content) = &body.content {
        if content.trim().is_empty() || content.len() > 1000 {
            return Err(validation_error("invalid_content", "Content must be 1-1000 characters"));
        }
    }

    let completed_at = match body.status.as_deref() {
        Some(status) => match parse_status(status) {
            Ok(TaskStatus::Completed) => Some(Some(chrono::Utc::now())),
            Ok(_) => Some(None),
            Err(_) => {
                return Err(validation_error(
                    "invalid_status",
                    "Status must be PENDING, IN_PROGRESS, or COMPLETED",
                ));
            }
        },
        None => None,
    };

    let count = task_service::update_task(
        &state.task_repo,
        UpdateTaskInput {
            user_id: user.user_id,
            task_id,
            content: body.content,
            completed_at,
            version: body.version,
        },
    )
    .await
    .map_err(from_app_error)?;

    Ok((StatusCode::OK, Json(CountResponse { count })))
}

async fn delete_task(
    State(state): State<AppState>,
    axum::extract::Extension(user): axum::extract::Extension<AuthUser>,
    Path(task_id): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let count = task_service::delete_task(
        &state.task_repo,
        DeleteTaskInput {
            user_id: user.user_id,
            task_id,
        },
    )
    .await
    .map_err(from_app_error)?;

    Ok((StatusCode::OK, Json(CountResponse { count })))
}

fn parse_status(value: &str) -> Result<TaskStatus, ()> {
    match value {
        "PENDING" => Ok(TaskStatus::Pending),
        "IN_PROGRESS" => Ok(TaskStatus::InProgress),
        "COMPLETED" => Ok(TaskStatus::Completed),
        _ => Err(()),
    }
}
