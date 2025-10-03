use serde::{Deserialize, Serialize};

use crate::payload::PayloadTrait;

/// A generic struct for paginated listings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationListing<ItemType> {
    /// The items in the current page
    pub items: Vec<ItemType>,
    /// Offset of the first item in the current page
    pub offset: u32,
    /// Maximum number of items per page
    pub limit: u32,
}

impl<ItemType> PayloadTrait for PaginationListing<ItemType>
where
    ItemType: Serialize + for<'de> Deserialize<'de>,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_json(json_str: &str) -> Self {
        serde_json::from_str(json_str).unwrap()
    }
}
