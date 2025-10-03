use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{date_formatter::date_format, payload::PayloadTrait};

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

impl PayloadTrait for Schedule {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_json(json_str: &str) -> Self {
        serde_json::from_str(json_str).unwrap()
    }
}
