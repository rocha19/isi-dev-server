// src/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use deadpool::managed::PoolError;
use serde_json::json;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    Validation(ValidationErrors),
    Database(String),
    NotFound,
    Conflict(String),
    UnprocessableEntity(String),
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Validation(err) => (
                StatusCode::BAD_REQUEST,
                format!("Validation error: {:?}", err),
            ),
            AppError::Database(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", err),
            ),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::UnprocessableEntity(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = json!({
            "error": error_message,
            "status": status.as_u16()
        });

        (status, axum::Json(body)).into_response()
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        AppError::Validation(err)
    }
}

impl From<PoolError<tokio_postgres::Error>> for AppError {
    fn from(err: PoolError<tokio_postgres::Error>) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<deadpool_postgres::PoolError> for AppError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        AppError::Database(err.to_string())
    }
}
