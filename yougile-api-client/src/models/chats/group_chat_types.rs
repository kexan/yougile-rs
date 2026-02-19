use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Type for the users field in GroupChat - likely a list of user IDs
pub type UsersList = Vec<String>;

// Type for user_role_map - mapping user IDs to role IDs
pub type UserRoleMap = HashMap<String, String>;

// Type for role_config_map - mapping role IDs to role configurations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleConfig {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<HashMap<String, serde_json::Value>>, // Keep as Value for now to avoid nested complexity
}

pub type RoleConfigMap = HashMap<String, RoleConfig>;

impl RoleConfig {
    pub fn new(title: String) -> Self {
        Self {
            title,
            permissions: None,
        }
    }
}

