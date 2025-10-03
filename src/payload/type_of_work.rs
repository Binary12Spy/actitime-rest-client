use serde::{Deserialize, Serialize};

use crate::payload::PayloadTrait;

/// Workflow status model representing a status in a workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeOfWork {
    /// Unique user identifier (read-only)
    pub id: i32,
    /// Name of the workflow status
    pub name: String,
    /// Work unit cost
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    /// Archived status of the type of work. If 'true', type of work is archived. If 'false', type of work is active.
    pub archived: bool,
    /// Billable status of the type of work. If 'true', type of work is billable. If 'false', type of work is non-billable.
    pub billable: bool,
    /// Default status of the type of work. If 'true', type of work is default. If 'false', type of work is non-default.
    pub default: bool,
}

impl PayloadTrait for TypeOfWork {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_json(json_str: &str) -> Self {
        serde_json::from_str(json_str).unwrap()
    }
}
