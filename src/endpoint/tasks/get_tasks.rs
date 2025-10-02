use crate::client_context::ClientContext;
use crate::errors::ClientError;
use crate::payload::{ApiError, TaskList, enums::TaskStatus};

pub enum GetTasksSortOrder {
    CreatedAsc,
    CreatedDesc,
    NameAsc,
    NameDesc,
    StatusAsc,
    StatusDesc,
}

impl GetTasksSortOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            GetTasksSortOrder::CreatedAsc => "+created",
            GetTasksSortOrder::CreatedDesc => "-created",
            GetTasksSortOrder::NameAsc => "+name",
            GetTasksSortOrder::NameDesc => "-name",
            GetTasksSortOrder::StatusAsc => "+status",
            GetTasksSortOrder::StatusDesc => "-status",
        }
    }
}

pub enum GetTasksIncludeReferenced {
    Customers,
    Projects,
    TypeOfWork,
    WorkflowStatuses,
}

pub struct GetTasksParameters {
    /// Index offset of the first item to return (for pagination)
    pub offset: Option<u32>,
    /// Maximum number of items to return (for pagination)
    pub limit: Option<u32>,
    /// Task IDs to filter by
    pub task_ids: Option<Vec<u32>>,
    /// Customer IDs to filter by
    pub customer_ids: Option<Vec<u32>>,
    /// Project IDs to filter by
    pub project_ids: Option<Vec<u32>>,
    /// Type of Work IDs to filter by
    pub type_of_work_ids: Option<Vec<u32>>,
    /// Workflow Status IDs to filter by
    pub workflow_status_ids: Option<Vec<u32>>,
    /// Sort order for the returned tasks
    pub sort_order: Option<GetTasksSortOrder>,
    /// Task name filter (partial match)
    pub name_filter: Option<String>,
    /// Contains words filter (partial match)
    pub contains_words: Option<String>,
    /// Status filter
    pub status: Option<TaskStatus>,
    /// Include referenced entities in the response
    pub include_referenced: Option<Vec<GetTasksIncludeReferenced>>,
}

impl Into<String> for GetTasksParameters {
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
        if let Some(task_ids) = self.task_ids {
            let ids = task_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            query_params.push(("taskIds", ids));
        }
        if let Some(customer_ids) = self.customer_ids {
            let ids = customer_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            query_params.push(("customerIds", ids));
        }
        if let Some(project_ids) = self.project_ids {
            let ids = project_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            query_params.push(("projectIds", ids));
        }
        if let Some(type_of_work_ids) = self.type_of_work_ids {
            let ids = type_of_work_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            query_params.push(("typeOfWorkIds", ids));
        }
        if let Some(workflow_status_ids) = self.workflow_status_ids {
            let ids = workflow_status_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            query_params.push(("workflowStatusIds", ids));
        }
        if let Some(sort_order) = self.sort_order {
            query_params.push(("sort", sort_order.as_str().to_string()));
        }
        if let Some(name_filter) = self.name_filter {
            query_params.push(("name", name_filter));
        }
        if let Some(contains_words) = self.contains_words {
            query_params.push(("words", contains_words));
        }
        if let Some(status) = self.status {
            let status_str = match status {
                TaskStatus::Open => "open",
                TaskStatus::Completed => "completed",
            };
            query_params.push(("status", status_str.to_string()));
        }
        if let Some(include_referenced) = self.include_referenced {
            let includes = include_referenced
                .iter()
                .map(|inc| match inc {
                    GetTasksIncludeReferenced::Customers => "customers",
                    GetTasksIncludeReferenced::Projects => "projects",
                    GetTasksIncludeReferenced::TypeOfWork => "typeOfWork",
                    GetTasksIncludeReferenced::WorkflowStatuses => "workflowStatuses",
                })
                .collect::<Vec<_>>()
                .join(",");
            query_params.push(("includeReferenced", includes));
        }

        let query_string = query_params
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<_>>()
            .join("&");
        query_string
    }
}

pub fn get_tasks(
    client_context: &ClientContext,
    parameters: Option<GetTasksParameters>,
) -> Result<TaskList, ClientError> {
    let mut url = format!("{}/tasks?", client_context.base_url);

    if let Some(params) = parameters {
        let query_string: String = params.into();
        url.push_str(&query_string);
    } else {
        url.push_str("offset=0&limit=100");
    }

    if client_context.basic_auth_token.is_none() {
        return Err(ClientError::Unauthorized);
    }

    let response = client_context
        .http_client
        .get(&url)
        .send()
        .map_err(ClientError::Reqwest)?;

    let status_code = response.status().as_u16();
    let text = response.text().map_err(ClientError::Reqwest)?;
    match status_code {
        200 => {
            let user = TaskList::from_json(&text).map_err(ClientError::SerdeJson)?;
            Ok(user)
        }
        _ => {
            let api_error = ApiError::from_json(&text).map_err(ClientError::SerdeJson)?;
            Err(ClientError::ApiError(status_code, api_error))
        }
    }
}
