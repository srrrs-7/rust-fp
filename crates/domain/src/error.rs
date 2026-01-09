use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {resource}")]
    NotFound { resource: String, message: String },
    #[error("Forbidden: {resource}")]
    Forbidden { resource: String, message: String },
    #[error("Unauthorized: {resource}")]
    Unauthorized { resource: String, message: String },
    #[error("Conflict: {resource}")]
    Conflict { resource: String, message: String },
    #[error("Domain error: {domain}")]
    Domain { domain: String, message: String },
    #[error("Validation error: {code}")]
    Validation {
        code: String,
        message: String,
        field: Option<String>,
    },
    #[error("Database error")]
    Database { message: String },
    #[error("API error")]
    Api { message: String },
}

impl AppError {
    pub fn not_found(resource: impl Into<String>, message: impl Into<String>) -> Self {
        Self::NotFound {
            resource: resource.into(),
            message: message.into(),
        }
    }

    pub fn conflict(resource: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Conflict {
            resource: resource.into(),
            message: message.into(),
        }
    }

    pub fn database(message: impl Into<String>) -> Self {
        Self::Database {
            message: message.into(),
        }
    }

    pub fn validation(
        code: impl Into<String>,
        message: impl Into<String>,
        field: Option<String>,
    ) -> Self {
        Self::Validation {
            code: code.into(),
            message: message.into(),
            field,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    NotFound,
    Forbidden,
    Unauthorized,
    Conflict,
    Domain,
    Validation,
    Database,
    Api,
}

impl From<&AppError> for ErrorKind {
    fn from(value: &AppError) -> Self {
        match value {
            AppError::NotFound { .. } => ErrorKind::NotFound,
            AppError::Forbidden { .. } => ErrorKind::Forbidden,
            AppError::Unauthorized { .. } => ErrorKind::Unauthorized,
            AppError::Conflict { .. } => ErrorKind::Conflict,
            AppError::Domain { .. } => ErrorKind::Domain,
            AppError::Validation { .. } => ErrorKind::Validation,
            AppError::Database { .. } => ErrorKind::Database,
            AppError::Api { .. } => ErrorKind::Api,
        }
    }
}
