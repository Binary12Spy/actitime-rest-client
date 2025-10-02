use std::error::Error as StdError;
use std::fmt;

use crate::payload::ApiError;

/// General error type for the ActiTime client
#[derive(Debug)]
pub enum ClientError {
    NotFound,
    Unauthorized,
    BadRequest,
    Timeout,
    ApiError(u16, ApiError),
    Message(String),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    Other(Box<dyn StdError + Send + Sync>),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::NotFound => write!(f, "Resource not found"),
            ClientError::Unauthorized => write!(f, "Unauthorized access"),
            ClientError::BadRequest => write!(f, "Bad request"),
            ClientError::Timeout => write!(f, "Operation timed out"),
            ClientError::ApiError(status_code, api_error) => {
                write!(f, "API Error ({}): {:?}", status_code, api_error)
            }
            ClientError::Message(msg) => write!(f, "Error: {}", msg),
            ClientError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            ClientError::SerdeJson(e) => write!(f, "Serde JSON error: {}", e),
            ClientError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl StdError for ClientError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ClientError::Reqwest(e) => Some(e),
            ClientError::Other(e) => Some(&**e),
            _ => None,
        }
    }
}
