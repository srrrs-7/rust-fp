use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::Utc;

use application::task_service::repository::TaskRepository;
use application::user_service::repository::UserRepository;
use domain::error::AppError;
use domain::task::entity::Task;
use domain::task::inputs::{
    CreateTaskInput, DeleteTaskInput, GetTaskInput, ListTasksInput, UpdateTaskInput,
};
use domain::user::entity::User;
use domain::user::inputs::{
    CreateUserInput, DeleteUserInput, GetUserByEmailInput, GetUserByUsernameInput, GetUserInput,
    ListUsersInput, UpdateUserInput,
};

use crate::middleware::cognito_auth::AuthUser;
use crate::response::ErrorResponse;
use crate::AppState;

pub fn app_state(task_repo: MockTaskRepo, user_repo: MockUserRepo) -> AppState {
    AppState {
        task_repo: Arc::new(task_repo),
        user_repo: Arc::new(user_repo),
    }
}

pub fn auth_user() -> AuthUser {
    AuthUser {
        user_id: "user-123".to_string(),
        email: Some("user@example.com".to_string()),
        username: Some("user123".to_string()),
        groups: vec!["users".to_string()],
    }
}

pub fn assert_status<T: IntoResponse>(result: Result<T, ErrorResponse>, expected: StatusCode) {
    let status = match result {
        Ok(response) => response.into_response().status(),
        Err(error) => error.into_response().status(),
    };
    assert_eq!(status, expected);
}

fn sample_task(user_id: &str, task_id: &str) -> Task {
    Task {
        user_id: user_id.to_string(),
        task_id: task_id.to_string(),
        content: "sample task".to_string(),
        completed_at: None,
        version: 0,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn sample_user(user_id: &str, client_id: &str) -> User {
    User {
        user_id: user_id.to_string(),
        client_id: client_id.to_string(),
        username: "user123".to_string(),
        email: "user@example.com".to_string(),
        name: None,
        picture: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn take_or_default<T>(
    cell: &Mutex<Option<Result<T, AppError>>>,
    default: impl FnOnce() -> Result<T, AppError>,
) -> Result<T, AppError> {
    cell.lock().expect("mutex").take().unwrap_or_else(default)
}

pub struct MockTaskRepo {
    pub create_result: Mutex<Option<Result<Task, AppError>>>,
    pub update_result: Mutex<Option<Result<i64, AppError>>>,
    pub delete_result: Mutex<Option<Result<i64, AppError>>>,
    pub get_result: Mutex<Option<Result<Option<Task>, AppError>>>,
    pub list_result: Mutex<Option<Result<Vec<Task>, AppError>>>,
}

impl Default for MockTaskRepo {
    fn default() -> Self {
        Self {
            create_result: Mutex::new(None),
            update_result: Mutex::new(None),
            delete_result: Mutex::new(None),
            get_result: Mutex::new(None),
            list_result: Mutex::new(None),
        }
    }
}

impl MockTaskRepo {
    pub fn with_create_result(result: Result<Task, AppError>) -> Self {
        Self {
            create_result: Mutex::new(Some(result)),
            ..Default::default()
        }
    }

    pub fn with_update_result(result: Result<i64, AppError>) -> Self {
        Self {
            update_result: Mutex::new(Some(result)),
            ..Default::default()
        }
    }

    pub fn with_delete_result(result: Result<i64, AppError>) -> Self {
        Self {
            delete_result: Mutex::new(Some(result)),
            ..Default::default()
        }
    }

    pub fn with_get_result(result: Result<Option<Task>, AppError>) -> Self {
        Self {
            get_result: Mutex::new(Some(result)),
            ..Default::default()
        }
    }

    pub fn with_list_result(result: Result<Vec<Task>, AppError>) -> Self {
        Self {
            list_result: Mutex::new(Some(result)),
            ..Default::default()
        }
    }
}

#[async_trait]
impl TaskRepository for MockTaskRepo {
    async fn create_task(&self, input: CreateTaskInput) -> Result<Task, AppError> {
        take_or_default(&self.create_result, || {
            Ok(sample_task(&input.user_id, "task-1"))
        })
    }

    async fn update_task(&self, _input: UpdateTaskInput) -> Result<i64, AppError> {
        take_or_default(&self.update_result, || Ok(1))
    }

    async fn delete_task(&self, _input: DeleteTaskInput) -> Result<i64, AppError> {
        take_or_default(&self.delete_result, || Ok(1))
    }

    async fn get_task(&self, input: GetTaskInput) -> Result<Option<Task>, AppError> {
        take_or_default(&self.get_result, || {
            Ok(Some(sample_task(&input.user_id, &input.task_id)))
        })
    }

    async fn list_tasks(&self, input: ListTasksInput) -> Result<Vec<Task>, AppError> {
        take_or_default(&self.list_result, || {
            Ok(vec![sample_task(&input.user_id, "task-1")])
        })
    }
}

pub struct MockUserRepo {
    pub create_result: Mutex<Option<Result<User, AppError>>>,
    pub update_result: Mutex<Option<Result<User, AppError>>>,
    pub delete_result: Mutex<Option<Result<i64, AppError>>>,
    pub get_result: Mutex<Option<Result<Option<User>, AppError>>>,
    pub get_by_email_result: Mutex<Option<Result<Option<User>, AppError>>>,
    pub get_by_username_result: Mutex<Option<Result<Option<User>, AppError>>>,
    pub list_result: Mutex<Option<Result<Vec<User>, AppError>>>,
}

impl Default for MockUserRepo {
    fn default() -> Self {
        Self {
            create_result: Mutex::new(None),
            update_result: Mutex::new(None),
            delete_result: Mutex::new(None),
            get_result: Mutex::new(None),
            get_by_email_result: Mutex::new(None),
            get_by_username_result: Mutex::new(None),
            list_result: Mutex::new(None),
        }
    }
}

impl MockUserRepo {
    pub fn with_create_result(result: Result<User, AppError>) -> Self {
        Self {
            create_result: Mutex::new(Some(result)),
            ..Default::default()
        }
    }

    pub fn with_update_result(result: Result<User, AppError>) -> Self {
        Self {
            update_result: Mutex::new(Some(result)),
            ..Default::default()
        }
    }

    pub fn with_delete_result(result: Result<i64, AppError>) -> Self {
        Self {
            delete_result: Mutex::new(Some(result)),
            ..Default::default()
        }
    }

    pub fn with_get_result(result: Result<Option<User>, AppError>) -> Self {
        Self {
            get_result: Mutex::new(Some(result)),
            ..Default::default()
        }
    }

    pub fn with_get_by_email_result(result: Result<Option<User>, AppError>) -> Self {
        Self {
            get_by_email_result: Mutex::new(Some(result)),
            ..Default::default()
        }
    }

    pub fn with_get_by_username_result(result: Result<Option<User>, AppError>) -> Self {
        Self {
            get_by_username_result: Mutex::new(Some(result)),
            ..Default::default()
        }
    }

    pub fn with_list_result(result: Result<Vec<User>, AppError>) -> Self {
        Self {
            list_result: Mutex::new(Some(result)),
            ..Default::default()
        }
    }
}

#[async_trait]
impl UserRepository for MockUserRepo {
    async fn create_user(&self, input: CreateUserInput) -> Result<User, AppError> {
        take_or_default(&self.create_result, || {
            Ok(User {
                user_id: input.user_id,
                client_id: input.client_id,
                username: input.username,
                email: input.email,
                name: input.name,
                picture: input.picture,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        })
    }

    async fn update_user(&self, input: UpdateUserInput) -> Result<User, AppError> {
        take_or_default(&self.update_result, || {
            Ok(sample_user(&input.user_id, "client-1"))
        })
    }

    async fn delete_user(&self, _input: DeleteUserInput) -> Result<i64, AppError> {
        take_or_default(&self.delete_result, || Ok(1))
    }

    async fn get_user(&self, input: GetUserInput) -> Result<Option<User>, AppError> {
        take_or_default(&self.get_result, || {
            Ok(Some(sample_user(&input.user_id, "client-1")))
        })
    }

    async fn get_user_by_email(
        &self,
        input: GetUserByEmailInput,
    ) -> Result<Option<User>, AppError> {
        take_or_default(&self.get_by_email_result, || {
            let mut user = sample_user("user-1", "client-1");
            user.email = input.email;
            Ok(Some(user))
        })
    }

    async fn get_user_by_username(
        &self,
        input: GetUserByUsernameInput,
    ) -> Result<Option<User>, AppError> {
        take_or_default(&self.get_by_username_result, || {
            let mut user = sample_user("user-1", "client-1");
            user.username = input.username;
            Ok(Some(user))
        })
    }

    async fn list_users(&self, input: ListUsersInput) -> Result<Vec<User>, AppError> {
        take_or_default(&self.list_result, || {
            Ok(vec![sample_user("user-1", &input.client_id)])
        })
    }
}
