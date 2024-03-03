
use axum::http::StatusCode;
use axum::response::{Response, IntoResponse};
use crate::errors::AppError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::RequestInputError(msg) =>
                (StatusCode::BAD_REQUEST, format!("{}", msg)).into_response(),
            _ =>
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", self)).into_response(),
        }
    }
}
