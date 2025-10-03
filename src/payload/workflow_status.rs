use serde::{Deserialize, Serialize};

use crate::payload::PayloadTrait;

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

impl PayloadTrait for WorkflowStatus {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_json(json_str: &str) -> Self {
        serde_json::from_str(json_str).unwrap()
    }
}
