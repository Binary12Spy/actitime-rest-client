use serde::{Deserialize, Serialize};

use crate::payload::{Customer, Project, Task, TypeOfWork, WorkflowStatus};

/// Task list model representing a list of tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskList {
    /// List of tasks
    pub items: Vec<Task>,
    /// Batch offset index
    pub offset: i32,
    /// Maximum number of items in the response
    pub limit: i32,
    /// Related customers
    pub customers: Option<Vec<Customer>>,
    /// Related projects
    pub projects: Option<Vec<Project>>,
    /// Related types of work
    pub types_of_work: Option<Vec<TypeOfWork>>,
    /// Workflow statuses related to the tasks
    pub workflow_statuses: Option<Vec<WorkflowStatus>>,
}

impl TaskList {
    /// Create a TaskList from a JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
