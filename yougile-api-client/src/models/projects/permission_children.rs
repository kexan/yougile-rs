use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type for children in ProjectPermissions - represents nested structures in permissions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectPermissionChildValue {
    PermissionMap(HashMap<String, serde_json::Value>), // For permissions-like structures
    Array(Vec<serde_json::Value>), // For arrays
    RawValue(serde_json::Value), // For generic values
}

pub type ProjectPermissionChildren = HashMap<String, ProjectPermissionChildValue>;