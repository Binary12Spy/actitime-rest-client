use serde::{Deserialize, Serialize};

/// Allowed actions model representing the permissions for certain actions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowedActions {
    /// Permission to create/modify
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_modify: Option<bool>,
    /// Permission to delete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
}
