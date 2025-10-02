use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::date_formatter::date_format;

/// Schedule model representing a user's work schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    /// Start date of the schedule
    /// Example: "2020-01-01"
    #[serde(with = "date_format", default)]
    pub date_from: NaiveDate,
    /// End date of the schedule
    /// Example: "2020-07-01"
    #[serde(with = "date_format", default)]
    pub date_to: NaiveDate,
    /// Workday durations in minutes for each day in the specified date range
    pub entries: Vec<u32>,
}

impl Schedule {
    pub fn from_json(json_data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_data)
    }
}
