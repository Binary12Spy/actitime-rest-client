use serde::{Deserialize, Serialize};

/// Task status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Task is active
    Open,
    /// Task is completed
    Completed,
}

/// Leave type balance enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LeaveTypeBalance {
    /// No balance tracking
    None,
    /// Sick leave balance tracking
    Sick,
    /// Paid time off balance tracking
    PTO,
}

/// HTTP method enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Method {
    /// HTTP GET method
    GET,
    /// HTTP POST method
    POST,
    /// HTTP PUT method
    PUT,
    /// HTTP DELETE method
    DELETE,
    /// HTTP PATCH method
    PATCH,
}
