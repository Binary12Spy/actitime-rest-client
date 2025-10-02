use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::date_formatter::date_format;

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
