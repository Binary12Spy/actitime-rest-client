use serde::{Deserialize, Serialize};

/// API Error model representing an error response from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    /// Key of the error
    pub key: String,
    /// Message describing the error
    pub message: String,
    /// DStack trace of the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<String>,
    /// Field associated with the error, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

impl ApiError {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
