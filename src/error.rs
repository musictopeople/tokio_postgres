use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use std::fmt::Display;

#[derive(Debug)]
pub enum ApiError {
    Database(tokio_postgres::Error),
    Internal,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Database(e) => write!(f, "Database error: {}", e),
            ApiError::Internal => write!(f, "Internal server error"),
        }
    }
}

impl From<tokio_postgres::Error> for ApiError {
    fn from(err: tokio_postgres::Error) -> Self {
        ApiError::Database(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            ApiError::Database(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{{\"error\": \"Database error: {}\"}}", e),
            ),
            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("{\"error\": \"Internal server error\"}"),
            ),
        };

        (status, Json(body)).into_response()
    }
}
