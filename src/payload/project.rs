use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::date_formatter::date_format;

/// Project model representing a project in ActiTime
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// Unique user identifier (read-only)
    pub id: i32,
    /// Customer ID associated with the project
    pub customer_id: i32,
    /// Name of the project
    pub name: String,
    /// Archived status of the type of work. If 'true', type of work is archived. If 'false', type of work is active.
    pub archived: bool,
    /// Date the project was created
    #[serde(with = "date_format")]
    pub created: NaiveDate,
    /// Url of the project
    pub url: String,
    /// Customer name associated with the project
    pub customer_name: String,
    /// Description of the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
