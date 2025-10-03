use serde::{Deserialize, Serialize};

use crate::payload::{PayloadTrait, enums::LeaveTypeBalance};

/// Project model representing a project in ActiTime
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveType {
    /// Unique user identifier (read-only)
    pub id: i32,
    /// Name of the leave type
    pub name: String,
    /// Balance type of the leave type
    pub balance: LeaveTypeBalance,
    /// Archived status of the leave type
    pub archived: bool,
}

impl PayloadTrait for LeaveType {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_json(json_str: &str) -> Self {
        serde_json::from_str(json_str).unwrap()
    }
}
