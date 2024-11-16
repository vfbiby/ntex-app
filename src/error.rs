use ntex::http::StatusCode;
use ntex::web::{HttpRequest, HttpResponse, WebResponseError};
use sea_orm::DbErr;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] DbErr),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl WebResponseError for AppError {
    fn error_response(&self, _req: &HttpRequest) -> HttpResponse {
        let status = match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = self.to_string();
        let response = ErrorResponse {
            code: status.as_u16(),
            message,
        };

        HttpResponse::build(status)
            .json(&response)
    }
}
