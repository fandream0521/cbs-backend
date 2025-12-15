use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("not found")]
    NotFound,
    #[error("unauthorized")]
    #[allow(dead_code)]
    Unauthorized,
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("conflict: {0}")]
    #[allow(dead_code)]
    Conflict(String),
    #[error("internal error")]
    Internal(#[from] anyhow::Error),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorBody {
    code: i32,
    message: String,
    data: Option<serde_json::Value>,
}

impl IntoResponse for DomainError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            DomainError::NotFound => (StatusCode::NOT_FOUND, 404, self.to_string()),
            DomainError::Unauthorized => (StatusCode::UNAUTHORIZED, 401, self.to_string()),
            DomainError::InvalidInput(_) => (StatusCode::BAD_REQUEST, 400, self.to_string()),
            DomainError::Conflict(_) => (StatusCode::CONFLICT, 409, self.to_string()),
            DomainError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, 500, self.to_string()),
        };

        let body = ErrorBody {
            code,
            message,
            data: None,
        };

        (status, axum::Json(body)).into_response()
    }
}
