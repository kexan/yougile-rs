use crate::models::{self};
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
    pub permissions: Box<ProjectPermissions>,
}

impl ProjectRole {
    pub fn new(id: String, name: String, permissions: ProjectPermissions) -> ProjectRole {
        ProjectRole {
            id,
            name,
            description: None,
            permissions: Box::new(permissions),
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
    pub permissions: Box<ProjectPermissions>,
}

impl CreateProjectRole {
    pub fn new(name: String, permissions: ProjectPermissions) -> CreateProjectRole {
        CreateProjectRole {
            name,
            description: None,
            permissions: Box::new(permissions),
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
    pub permissions: Option<Box<ProjectPermissions>>,
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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectRoleList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<models::PagingMetadata>,
    /// Список ролей в проекте
    #[serde(rename = "content")]
    pub content: Vec<ProjectRole>,
}

impl ProjectRoleList {
    pub fn new(paging: models::PagingMetadata, content: Vec<ProjectRole>) -> ProjectRoleList {
        ProjectRoleList {
            paging: Box::new(paging),
            content,
        }
    }
}

