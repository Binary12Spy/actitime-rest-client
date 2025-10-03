use crate::payload::{
    TaskList,
    enums::{Method, TaskStatus},
};
use crate::routes::Route;

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

impl GetTasksParameters {
    pub fn new() -> Self {
        Self {
            offset: None,
            limit: None,
            task_ids: None,
            customer_ids: None,
            project_ids: None,
            type_of_work_ids: None,
            workflow_status_ids: None,
            sort_order: None,
            name_filter: None,
            contains_words: None,
            status: None,
            include_referenced: None,
        }
    }

    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_task_ids(mut self, task_ids: Vec<u32>) -> Self {
        self.task_ids = Some(task_ids);
        self
    }

    pub fn with_customer_ids(mut self, customer_ids: Vec<u32>) -> Self {
        self.customer_ids = Some(customer_ids);
        self
    }

    pub fn with_project_ids(mut self, project_ids: Vec<u32>) -> Self {
        self.project_ids = Some(project_ids);
        self
    }

    pub fn with_type_of_work_ids(mut self, type_of_work_ids: Vec<u32>) -> Self {
        self.type_of_work_ids = Some(type_of_work_ids);
        self
    }

    pub fn with_workflow_status_ids(mut self, workflow_status_ids: Vec<u32>) -> Self {
        self.workflow_status_ids = Some(workflow_status_ids);
        self
    }

    pub fn with_sort_order(mut self, sort_order: GetTasksSortOrder) -> Self {
        self.sort_order = Some(sort_order);
        self
    }

    pub fn with_name_filter(mut self, name_filter: String) -> Self {
        self.name_filter = Some(name_filter);
        self
    }

    pub fn with_contains_words(mut self, contains_words: String) -> Self {
        self.contains_words = Some(contains_words);
        self
    }

    pub fn with_status(mut self, status: TaskStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_include_referenced(
        mut self,
        include_referenced: Vec<GetTasksIncludeReferenced>,
    ) -> Self {
        self.include_referenced = Some(include_referenced);
        self
    }
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

pub fn get_tasks(parameters: Option<GetTasksParameters>) -> Route<(), TaskList> {
    let mut url: String = "/tasks?".into();

    if let Some(params) = parameters {
        let query_string: String = params.into();
        url.push_str(&query_string);
    } else {
        url.push_str("offset=0&limit=100");
    }

    Route::new(Method::GET, url.as_str())
}
