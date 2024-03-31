use std::{num::ParseFloatError, string::FromUtf8Error, sync::PoisonError, time::SystemTimeError};
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

    #[error("Http error: {0}")]
    HttpError(String),

    #[error("Request input error: {0}")]
    RequestInputError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("System error: {0}")]
    SystemError(String),

    #[error("Crypto error: {0}")]
    CryptoError(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl From<mongodb::error::Error> for AppError {
    fn from(value: mongodb::error::Error) -> Self {
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

impl From<axum::http::Error> for AppError {
    fn from(value: axum::http::Error) -> Self {
        Self::HttpError(format!("{value}"))
    } 
}

impl From<oauth2::url::ParseError> for AppError {
    fn from(value: oauth2::url::ParseError) -> Self {
        Self::ParseError(format!("{value}"))
    }
}

impl From<ParseFloatError> for AppError {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseError(format!("{value}"))
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::ParseError(format!("{value}"))
    }
}

impl From<FromUtf8Error> for AppError {
    fn from(value: FromUtf8Error) -> Self {
        Self::ParseError(format!("{value}"))
    }
}

impl From<base64::DecodeError> for AppError {
    fn from(value: base64::DecodeError) -> Self {
        Self::ParseError(format!("{value}"))
    }
}

impl From<SystemTimeError> for AppError {
    fn from(value: SystemTimeError) -> Self {
        Self::SystemError(format!("{value}"))
    }
}

impl From<rcgen::Error> for AppError {
    fn from(value: rcgen::Error) -> Self {
        Self::CryptoError(format!("{value}"))
    }
}