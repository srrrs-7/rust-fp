use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use domain::error::{AppError, ErrorKind};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub success: bool,
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    pub field: Option<String>,
}

pub struct ErrorResponse {
    status: StatusCode,
    body: ErrorBody,
}

impl ErrorResponse {
    pub fn new(status: StatusCode, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status,
            body: ErrorBody {
                success: false,
                error: ErrorDetail {
                    code: code.into(),
                    message: message.into(),
                    field: None,
                },
            },
        }
    }

    pub fn with_field(
        status: StatusCode,
        code: impl Into<String>,
        message: impl Into<String>,
        field: Option<String>,
    ) -> Self {
        Self {
            status,
            body: ErrorBody {
                success: false,
                error: ErrorDetail {
                    code: code.into(),
                    message: message.into(),
                    field,
                },
            },
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}

pub fn from_app_error(error: AppError) -> ErrorResponse {
    let kind = ErrorKind::from(&error);
    match error {
        AppError::NotFound { resource, message } => {
            ErrorResponse::new(StatusCode::NOT_FOUND, resource, message)
        }
        AppError::Forbidden { resource, message } => {
            ErrorResponse::new(StatusCode::FORBIDDEN, resource, message)
        }
        AppError::Unauthorized { resource, message } => {
            ErrorResponse::new(StatusCode::UNAUTHORIZED, resource, message)
        }
        AppError::Conflict { resource, message } => {
            ErrorResponse::new(StatusCode::CONFLICT, resource, message)
        }
        AppError::Domain { domain, message } => {
            ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, domain, message)
        }
        AppError::Validation { code, message, field } => {
            ErrorResponse::with_field(StatusCode::BAD_REQUEST, code, message, field)
        }
        AppError::Database { message } => ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{:?}", kind),
            message,
        ),
        AppError::Api { message } => ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{:?}", kind),
            message,
        ),
    }
}

pub fn validation_error(code: impl Into<String>, message: impl Into<String>) -> ErrorResponse {
    ErrorResponse::with_field(StatusCode::BAD_REQUEST, code, message, None)
}
