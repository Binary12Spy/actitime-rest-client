use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::date_formatter::date_format_option;
use crate::payload::{PayloadTrait, UserWithAllowedActions, UserWithPassword};

/// User model representing an ActiTime user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// Unique user identifier (read-only)
    pub id: i32,
    /// Unique identifier of user department
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<i32>,
    /// Unique identifier of user time zone group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone_group_id: Option<i32>,
    /// User's hire date (visible only to users with 'Manage Accounts & Permissions' access right)
    /// Example: "2020-01-01"
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "date_format_option", default)]
    pub hired: Option<NaiveDate>,
    /// User's release date (visible only to users with 'Manage Accounts & Permissions' access right)
    /// Example: "2020-07-01"
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "date_format_option", default)]
    pub release_date: Option<NaiveDate>,
    /// User email (visible only for users with 'Manage Accounts & Permissions' access right)
    pub email: String,
    /// Full name (first name + MI. + last name) - read-only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// Unique username (visible only for users with 'Manage Accounts & Permissions' access right)
    pub username: String,
    /// Shows status of user account. If 'true', account is active. If 'false', account is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// First name of user
    pub first_name: String,
    /// Middle initial (MI) of user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// Last name of user
    pub last_name: String,
}

impl PayloadTrait for User {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn from_json(json_str: &str) -> Self {
        serde_json::from_str(json_str).unwrap()
    }
}

impl User {
    /// Creates a new User with the minimum required fields
    pub fn new(
        id: i32,
        email: String,
        username: String,
        first_name: String,
        last_name: String,
    ) -> Self {
        Self {
            id,
            department_id: None,
            time_zone_group_id: None,
            hired: None,
            release_date: None,
            email,
            full_name: None,
            username,
            active: None,
            first_name,
            middle_name: None,
            last_name,
        }
    }

    /// Builder pattern methods for optional fields
    pub fn with_department_id(mut self, department_id: i32) -> Self {
        self.department_id = Some(department_id);
        self
    }

    pub fn with_time_zone_group_id(mut self, time_zone_group_id: i32) -> Self {
        self.time_zone_group_id = Some(time_zone_group_id);
        self
    }

    pub fn with_hired(mut self, hired: NaiveDate) -> Self {
        self.hired = Some(hired);
        self
    }

    pub fn with_release_date(mut self, release_date: NaiveDate) -> Self {
        self.release_date = Some(release_date);
        self
    }

    pub fn with_full_name(mut self, full_name: String) -> Self {
        self.full_name = Some(full_name);
        self
    }

    pub fn with_active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    pub fn with_middle_name(mut self, middle_name: String) -> Self {
        self.middle_name = Some(middle_name);
        self
    }
}

impl From<UserWithAllowedActions> for User {
    fn from(user_with_actions: UserWithAllowedActions) -> Self {
        User {
            id: user_with_actions.id,
            department_id: user_with_actions.department_id,
            time_zone_group_id: user_with_actions.time_zone_group_id,
            hired: user_with_actions.hired,
            release_date: user_with_actions.release_date,
            email: user_with_actions.email,
            full_name: user_with_actions.full_name,
            username: user_with_actions.username,
            active: user_with_actions.active,
            first_name: user_with_actions.first_name,
            middle_name: user_with_actions.middle_name,
            last_name: user_with_actions.last_name,
        }
    }
}

impl From<&UserWithAllowedActions> for User {
    fn from(user_with_actions: &UserWithAllowedActions) -> Self {
        User {
            id: user_with_actions.id,
            department_id: user_with_actions.department_id,
            time_zone_group_id: user_with_actions.time_zone_group_id,
            hired: user_with_actions.hired,
            release_date: user_with_actions.release_date,
            email: user_with_actions.email.clone(),
            full_name: user_with_actions.full_name.clone(),
            username: user_with_actions.username.clone(),
            active: user_with_actions.active,
            first_name: user_with_actions.first_name.clone(),
            middle_name: user_with_actions.middle_name.clone(),
            last_name: user_with_actions.last_name.clone(),
        }
    }
}

impl From<UserWithPassword> for User {
    fn from(user_with_password: UserWithPassword) -> Self {
        User {
            id: user_with_password.id,
            department_id: user_with_password.department_id,
            time_zone_group_id: user_with_password.time_zone_group_id,
            hired: user_with_password.hired,
            release_date: user_with_password.release_date,
            email: user_with_password.email,
            full_name: user_with_password.full_name,
            username: user_with_password.username,
            active: user_with_password.active,
            first_name: user_with_password.first_name,
            middle_name: user_with_password.middle_name,
            last_name: user_with_password.last_name,
        }
    }
}

impl From<&UserWithPassword> for User {
    fn from(user_with_password: &UserWithPassword) -> Self {
        User {
            id: user_with_password.id,
            department_id: user_with_password.department_id,
            time_zone_group_id: user_with_password.time_zone_group_id,
            hired: user_with_password.hired,
            release_date: user_with_password.release_date,
            email: user_with_password.email.clone(),
            full_name: user_with_password.full_name.clone(),
            username: user_with_password.username.clone(),
            active: user_with_password.active,
            first_name: user_with_password.first_name.clone(),
            middle_name: user_with_password.middle_name.clone(),
            last_name: user_with_password.last_name.clone(),
        }
    }
}
