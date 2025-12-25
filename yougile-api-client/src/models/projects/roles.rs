use crate::models::{self, common::Page};
use serde::{Deserialize, Serialize};

use super::permissions::ProjectPermissions;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectRole {
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Название роли
    #[serde(rename = "name")]
    pub name: String,
    /// Описание роли
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Права в проекте
    #[serde(rename = "permissions")]
    pub permissions: ProjectPermissions,
}

impl ProjectRole {
    pub fn new(id: String, name: String, permissions: ProjectPermissions) -> ProjectRole {
        ProjectRole {
            id,
            name,
            description: None,
            permissions,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProjectRole {
    /// Название роли
    #[serde(rename = "name")]
    pub name: String,
    /// Описание роли
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Права в проекте
    #[serde(rename = "permissions")]
    pub permissions: ProjectPermissions,
}

impl CreateProjectRole {
    pub fn new(name: String, permissions: ProjectPermissions) -> CreateProjectRole {
        CreateProjectRole {
            name,
            description: None,
            permissions,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateProjectRole {
    /// Название роли
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Описание роли
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Права в проекте
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ProjectPermissions>,
}

impl UpdateProjectRole {
    pub fn new() -> UpdateProjectRole {
        UpdateProjectRole {
            name: None,
            description: None,
            permissions: None,
        }
    }
}

pub type ProjectRoleList = Page<ProjectRole>;
