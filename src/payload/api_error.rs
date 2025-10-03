use serde::{Deserialize, Serialize};

use crate::payload::PayloadTrait;

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

impl PayloadTrait for ApiError {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_json(json_str: &str) -> Self {
        serde_json::from_str(json_str).unwrap()
    }
}
