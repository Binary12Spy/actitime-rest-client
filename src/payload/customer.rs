use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{date_formatter::date_format, payload::PayloadTrait};

/// Workflow status model representing a status in a workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    /// Unique user identifier (read-only)
    pub id: i32,
    /// Name of the customer
    pub name: String,
    /// Archived status of the type of work. If 'true', type of work is archived. If 'false', type of work is active.
    pub archived: bool,
    /// Date the project was created
    #[serde(with = "date_format")]
    pub created: NaiveDate,
    /// Url of the project
    pub url: String,
    /// Description of the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PayloadTrait for Customer {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_json(json_str: &str) -> Self {
        serde_json::from_str(json_str).unwrap()
    }
}
