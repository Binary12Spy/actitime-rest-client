use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::enums::TaskStatus;
use crate::{
    date_formatter::date_format_option,
    payload::{AllowedActions, PayloadTrait},
};

/// User model representing an ActiTime user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskWithAllowedActions {
    /// Unique user identifier (read-only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Name of the task
    pub name: String,
    /// Description of the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Created date of the task
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "date_format_option", default)]
    pub created: Option<NaiveDate>,
    /// Status of the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
    /// Workflow status ID of the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_status_id: Option<i32>,
    /// Type of work ID associated with the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_of_work_id: Option<i32>,
    /// URL of the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Project name associated with the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    /// Customer name associated with the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    /// Workflow status name associated with the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_status_name: Option<String>,
    /// Type of work name associated with the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_of_work_name: Option<String>,
    /// Allowed actions for this task
    pub allowed_actions: AllowedActions,
    /// Deadline of the task
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "date_format_option", default)]
    pub deadline: Option<NaiveDate>,
    /// Estimated time for the task in hours
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time: Option<i32>,
    /// Customer ID associated with the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<i32>,
    /// Project ID associated with the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i32>,
}

impl PayloadTrait for TaskWithAllowedActions {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_json(json_str: &str) -> Self {
        serde_json::from_str(json_str).unwrap()
    }
}

impl TaskWithAllowedActions {
    /// Creates a new `TaskWithAllowedActions` with the specified name and allowed actions.
    ///
    /// # Arguments
    /// * `name` - The name of the task.
    /// * `allowed_actions` - The allowed actions for this task.
    ///
    /// # Returns
    /// A new instance of `TaskWithAllowedActions`.
    ///
    /// # Example
    /// ``` ignore
    /// let task = TaskWithAllowedActions::new("New Task".to_string(), AllowedActions { can_edit: true, can_delete: false });
    /// ```
    pub fn new(name: String, allowed_actions: AllowedActions) -> Self {
        TaskWithAllowedActions {
            id: None,
            name,
            description: None,
            created: None,
            status: None,
            workflow_status_id: None,
            type_of_work_id: None,
            url: None,
            project_name: None,
            customer_name: None,
            workflow_status_name: None,
            allowed_actions,
            type_of_work_name: None,
            deadline: None,
            estimated_time: None,
            customer_id: None,
            project_id: None,
        }
    }

    /// Sets the `id` field and returns the modified `TaskWithAllowedActions`.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the task.
    ///
    /// # Returns
    /// The modified `TaskWithAllowedActions` with the updated `id`.
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the `name` field and returns the modified `TaskWithAllowedActions`.
    ///
    /// # Arguments
    /// * `name` - The name of the task.
    ///
    /// # Returns
    /// The modified `TaskWithAllowedActions` with the updated `name`.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the `created` field and returns the modified `TaskWithAllowedActions`.
    ///
    /// # Arguments
    /// * `created` - The creation date of the task.
    ///
    /// # Returns
    /// The modified `TaskWithAllowedActions` with the updated `created` date.
    pub fn with_created(mut self, created: NaiveDate) -> Self {
        self.created = Some(created);
        self
    }

    /// Sets the `status` field and returns the modified `TaskWithAllowedActions`.
    ///
    /// # Arguments
    /// * `status` - The status of the task.
    ///
    /// # Returns
    /// The modified `TaskWithAllowedActions` with the updated `status`.
    pub fn with_status(mut self, status: TaskStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the `workflow_status_id` field and returns the modified `TaskWithAllowedActions`.
    ///
    /// # Arguments
    /// * `workflow_status_id` - The workflow status ID of the task.
    ///
    /// # Returns
    /// The modified `TaskWithAllowedActions` with the updated `workflow_status_id`.
    pub fn with_workflow_status_id(mut self, workflow_status_id: i32) -> Self {
        self.workflow_status_id = Some(workflow_status_id);
        self
    }

    /// Sets the `type_of_work_id` field and returns the modified `TaskWithAllowedActions`.
    ///
    /// # Arguments
    /// * `type_of_work_id` - The type of work ID associated with the task
    ///
    /// # Returns
    /// The modified `TaskWithAllowedActions` with the updated `type_of_work_id`.
    pub fn with_type_of_work_id(mut self, type_of_work_id: i32) -> Self {
        self.type_of_work_id = Some(type_of_work_id);
        self
    }

    /// Sets the `url` field and returns the modified `TaskWithAllowedActions`.
    ///
    /// # Arguments
    /// * `url` - The URL of the task.
    ///
    /// # Returns
    /// The modified `TaskWithAllowedActions` with the updated `url`.
    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    /// Sets the `project_name` field and returns the modified `TaskWithAllowedActions`.
    ///
    /// # Arguments
    /// * `project_name` - The project name associated with the task.
    ///
    /// # Returns
    /// The modified `TaskWithAllowedActions` with the updated `project_name`.
    pub fn with_project_name(mut self, project_name: String) -> Self {
        self.project_name = Some(project_name);
        self
    }

    /// Sets the `customer_name` field and returns the modified `TaskWithAllowedActions`.
    ///
    /// # Arguments
    /// * `customer_name` - The customer name associated with the task.
    ///
    /// # Returns
    /// The modified `TaskWithAllowedActions` with the updated `customer_name`.
    pub fn with_customer_name(mut self, customer_name: String) -> Self {
        self.customer_name = Some(customer_name);
        self
    }

    /// Sets the `workflow_status_name` field and returns the modified `TaskWithAllowedActions`.
    ///
    /// # Arguments
    /// * `workflow_status_name` - The workflow status name associated with the task.
    ///
    /// # Returns
    /// The modified `TaskWithAllowedActions` with the updated `workflow_status_name`.
    pub fn with_workflow_status_name(mut self, workflow_status_name: String) -> Self {
        self.workflow_status_name = Some(workflow_status_name);
        self
    }

    /// Sets the `type_of_work_name` field and returns the modified `TaskWithAllowedActions`.
    ///
    /// # Arguments
    /// * `type_of_work_name` - The type of work name associated with the task
    ///
    /// # Returns
    /// The modified `TaskWithAllowedActions` with the updated `type_of_work_name`.
    pub fn with_type_of_work_name(mut self, type_of_work_name: String) -> Self {
        self.type_of_work_name = Some(type_of_work_name);
        self
    }

    /// Sets the `deadline` field and returns the modified `TaskWithAllowedActions`.
    ///
    /// # Arguments
    /// * `deadline` - The deadline of the task.
    ///
    /// # Returns
    /// The modified `TaskWithAllowedActions` with the updated `deadline`.
    pub fn with_deadline(mut self, deadline: NaiveDate) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Sets the `estimated_time` field and returns the modified `TaskWithAllowedActions`.
    ///
    /// # Arguments
    /// * `estimated_time` - The estimated time for the task in hours.
    ///
    /// # Returns
    /// The modified `TaskWithAllowedActions` with the updated `estimated_time`.
    pub fn with_estimated_time(mut self, estimated_time: i32) -> Self {
        self.estimated_time = Some(estimated_time);
        self
    }

    /// Sets the `customer_id` field and returns the modified `TaskWithAllowedActions`.
    ///
    /// # Arguments
    /// * `customer_id` - The customer ID associated with the task.
    ///
    /// # Returns
    /// The modified `TaskWithAllowedActions` with the updated `customer_id`.
    pub fn with_customer_id(mut self, customer_id: i32) -> Self {
        self.customer_id = Some(customer_id);
        self
    }

    /// Sets the `project_id` field and returns the modified `TaskWithAllowedActions`.
    ///
    /// # Arguments
    /// * `project_id` - The project ID associated with the task.
    ///
    /// # Returns
    /// The modified `TaskWithAllowedActions` with the updated `project_id`.
    pub fn with_project_id(mut self, project_id: i32) -> Self {
        self.project_id = Some(project_id);
        self
    }
}
