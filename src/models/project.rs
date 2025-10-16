use crate::models::{self, board::BoardPermissions};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Название проекта
    #[serde(rename = "title")]
    pub title: String,
    /// Время создания проекта
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    /// Сотрудники на проекте и их роль. Возможные значения: <br/><div>1) системные роли: worker, admin, observer</div><div>2) ID пользовательской роли</div><div>3) \"-\" для удаления существующего пользователя из проекта</div>
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<serde_json::Value>,
}

impl Project {
    pub fn new(id: String, title: String, timestamp: f64) -> Project {
        Project {
            deleted: None,
            id,
            title,
            timestamp,
            users: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProject {
    /// Название проекта
    #[serde(rename = "title")]
    pub title: String,
    /// Сотрудники на проекте и их роль. Возможные значения: <br/><div>1) системные роли: worker, admin, observer</div><div>2) ID пользовательской роли</div><div>3) \"-\" для удаления существующего пользователя из проекта</div>
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<serde_json::Value>,
}

impl CreateProject {
    pub fn new(title: String) -> CreateProject {
        CreateProject { title, users: None }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateProject {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Название проекта
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Сотрудники на проекте и их роль. Возможные значения: <br/><div>1) системные роли: worker, admin, observer</div><div>2) ID пользовательской роли</div><div>3) \"-\" для удаления существующего пользователя из проекта</div>
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<serde_json::Value>,
}

impl UpdateProject {
    pub fn new() -> UpdateProject {
        UpdateProject {
            deleted: None,
            title: None,
            users: None,
        }
    }
}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<models::PagingMetadata>,
    /// Список проектов
    #[serde(rename = "content")]
    pub content: Vec<ProjectListBase>,
}

impl ProjectList {
    pub fn new(paging: models::PagingMetadata, content: Vec<ProjectListBase>) -> ProjectList {
        ProjectList {
            paging: Box::new(paging),
            content,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectListBase {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Название проекта
    #[serde(rename = "title")]
    pub title: String,
    /// Время создания проекта
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    /// Сотрудники на проекте и их роль. Возможные значения: <br/><div>1) системные роли: worker, admin, observer</div><div>2) ID пользовательской роли</div><div>3) \"-\" для удаления существующего пользователя из проекта</div>
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<serde_json::Value>,
}

impl ProjectListBase {
    pub fn new(id: String, title: String, timestamp: f64) -> ProjectListBase {
        ProjectListBase {
            deleted: None,
            id,
            title,
            timestamp,
            users: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectPermissions {
    #[serde(rename = "editTitle")]
    pub edit_title: bool,
    #[serde(rename = "delete")]
    pub delete: bool,
    #[serde(rename = "addBoard")]
    pub add_board: bool,
    #[serde(rename = "boards")]
    pub boards: Box<BoardPermissions>,
    #[serde(rename = "children")]
    pub children: serde_json::Value,
}

impl ProjectPermissions {
    pub fn new(
        edit_title: bool,
        delete: bool,
        add_board: bool,
        boards: BoardPermissions,
        children: serde_json::Value,
    ) -> ProjectPermissions {
        ProjectPermissions {
            edit_title,
            delete,
            add_board,
            boards: Box::new(boards),
            children,
        }
    }
}

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
    pub content: Vec<ProjectRoleListBase>,
}

impl ProjectRoleList {
    pub fn new(
        paging: models::PagingMetadata,
        content: Vec<ProjectRoleListBase>,
    ) -> ProjectRoleList {
        ProjectRoleList {
            paging: Box::new(paging),
            content,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectRoleListBase {
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

impl ProjectRoleListBase {
    pub fn new(id: String, name: String, permissions: ProjectPermissions) -> ProjectRoleListBase {
        ProjectRoleListBase {
            id,
            name,
            description: None,
            permissions: Box::new(permissions),
        }
    }
}
