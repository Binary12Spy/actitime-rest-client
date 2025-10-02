use serde::{Deserialize, Serialize};

/// Workflow status model representing a status in a workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStatus {
    /// Unique user identifier (read-only)
    pub id: i32,
    /// Name of the workflow status
    pub name: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub status: Option<String>,
    /// Allowed actions for this workflow status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<Vec<String>>,
}
