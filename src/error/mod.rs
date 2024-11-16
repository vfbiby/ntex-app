use ntex::web::{HttpResponse, WebResponseError, HttpRequest};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl WebResponseError for AppError {
    fn error_response(&self, _: &HttpRequest) -> HttpResponse {
        match self {
            AppError::Database(e) => {
                tracing::error!("Database error: {}", e);
                let error = json!({ "error": "Internal server error" });
                HttpResponse::InternalServerError()
                    .json(&error)
            }
            AppError::Validation(msg) => {
                let error = json!({ "error": msg });
                HttpResponse::BadRequest()
                    .json(&error)
            }
            AppError::NotFound(msg) => {
                let error = json!({ "error": msg });
                HttpResponse::NotFound()
                    .json(&error)
            }
            AppError::BadRequest(msg) => {
                let error = json!({ "error": msg });
                HttpResponse::BadRequest()
                    .json(&error)
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                let error = json!({ "error": "Internal server error" });
                HttpResponse::InternalServerError()
                    .json(&error)
            }
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
