use std::{result::Result, sync::PoisonError};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("IO error: {0}")]
    IOError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Concurrency error: {0}")]
    ConcurrencyError(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::DatabaseError(format!("{value}"))
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(format!("{value}"))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::ParseError(format!("{value}"))
    }
}

impl From<leon::ParseError> for AppError {
    fn from(value: leon::ParseError) -> Self {
        Self::ParseError(format!("{value}"))
    }
}

impl From<leon::RenderError> for AppError {
    fn from(value: leon::RenderError) -> Self {
        Self::ParseError(format!("{value}"))
    }
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(value: PoisonError<T>) -> Self {
        Self::ConcurrencyError(format!("{value}"))
    } 
}