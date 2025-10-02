use serde::{Deserialize, Serialize};

/// Allowed actions for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowedActions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_modify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
}
