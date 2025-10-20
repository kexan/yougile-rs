use crate::models::{self};
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
    pub content: Vec<Project>,
}

impl ProjectList {
    pub fn new(paging: models::PagingMetadata, content: Vec<Project>) -> ProjectList {
        ProjectList {
            paging: Box::new(paging),
            content,
        }
    }
}


