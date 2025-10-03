use crate::payload::{
    LeaveType, PaginationListing,
    enums::{LeaveTypeBalance, Method},
};
use crate::routes::Route;

pub enum GetLeaveTypesSortOrder {
    NameAsc,
    NameDesc,
}

impl GetLeaveTypesSortOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            GetLeaveTypesSortOrder::NameAsc => "+name",
            GetLeaveTypesSortOrder::NameDesc => "-name",
        }
    }
}

pub struct GetLeaveTypesParameters {
    /// Index offset of the first item to return (for pagination)
    pub offset: Option<u32>,
    /// Maximum number of items to return (for pagination)
    pub limit: Option<u32>,
    /// Task IDs to filter by
    pub type_ids: Option<Vec<u32>>,
    /// Task name filter (partial match)
    pub name_filter: Option<String>,
    /// Contains words filter (partial match)
    pub contains_words: Option<String>,
    /// Balance type to filter by
    pub balance: Option<LeaveTypeBalance>,
    /// Archived status to filter by
    pub archived: Option<bool>,
    /// Sort order for the returned tasks
    pub sort_order: Option<GetLeaveTypesSortOrder>,
}

impl Into<String> for GetLeaveTypesParameters {
    fn into(self) -> String {
        let mut query_params: Vec<(&'static str, String)> = vec![];

        if let Some(offset) = self.offset {
            query_params.push(("offset", offset.to_string()));
        } else {
            query_params.push(("offset", "0".to_string()));
        }
        if let Some(limit) = self.limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(type_ids) = self.type_ids {
            let ids = type_ids
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            query_params.push(("typeIds", ids));
        }
        if let Some(name_filter) = self.name_filter {
            query_params.push(("name", name_filter));
        }
        if let Some(contains_words) = self.contains_words {
            query_params.push(("words", contains_words));
        }
        if let Some(balance) = self.balance {
            query_params.push(("balance", format!("{:?}", balance)));
        }
        if let Some(archived) = self.archived {
            query_params.push(("archived", archived.to_string()));
        }
        if let Some(sort_order) = self.sort_order {
            query_params.push(("sort", sort_order.as_str().to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<_>>()
            .join("&");
        query_string
    }
}

pub fn get_leave_types(
    parameters: Option<GetLeaveTypesParameters>,
) -> Route<(), PaginationListing<LeaveType>> {
    let mut url: String = "/leaveTypes?".into();

    if let Some(params) = parameters {
        let query_string: String = params.into();
        url.push_str(&query_string);
    } else {
        url.push_str("offset=0&limit=100");
    }

    Route::new(Method::GET, url.as_str())
}

pub fn get_leave_type_by_id(id: i32) -> Route<(), LeaveType> {
    let url = format!("/leaveTypes/{}", id);
    Route::new(Method::GET, &url)
}
