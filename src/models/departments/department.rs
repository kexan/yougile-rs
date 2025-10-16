use crate::models::{self, PagingMetadata};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Department {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Название отдела
    #[serde(rename = "title")]
    pub title: String,
    /// Id родительского отдела. Оставить пустым или \"-\", если это отдел верхнего уровня
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// Сотрудники на отделе и их роль. Возможные значения: <br/><div>1) manager или member</div><div>2) \"-\" или \"\" для удаления существующего пользователя из отдела</div>
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<serde_json::Value>,
}

impl Department {
    pub fn new(id: String, title: String) -> Department {
        Department {
            deleted: None,
            id,
            title,
            parent_id: None,
            users: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DepartmentList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<PagingMetadata>,
    /// Список отделов
    #[serde(rename = "content")]
    pub content: Vec<DepartmentListBase>,
}

impl DepartmentList {
    pub fn new(paging: PagingMetadata, content: Vec<DepartmentListBase>) -> DepartmentList {
        DepartmentList {
            paging: Box::new(paging),
            content,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DepartmentListBase {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Название отдела
    #[serde(rename = "title")]
    pub title: String,
    /// Id родительского отдела. Оставить пустым или \"-\", если это отдел верхнего уровня
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// Сотрудники на отделе и их роль. Возможные значения: <br/><div>1) manager или member</div><div>2) \"-\" или \"\" для удаления существующего пользователя из отдела</div>
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<serde_json::Value>,
}

impl DepartmentListBase {
    pub fn new(id: String, title: String) -> DepartmentListBase {
        DepartmentListBase {
            deleted: None,
            id,
            title,
            parent_id: None,
            users: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateDepartment {
    /// Название отдела
    #[serde(rename = "title")]
    pub title: String,
    /// Id родительского отдела. Оставить пустым или \"-\", если это отдел верхнего уровня
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// Сотрудники на отделе и их роль. Возможные значения: <br/><div>1) manager или member</div><div>2) \"-\" или \"\" для удаления существующего пользователя из отдела</div>
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<serde_json::Value>,
}

impl CreateDepartment {
    pub fn new(title: String) -> CreateDepartment {
        CreateDepartment {
            title,
            parent_id: None,
            users: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDepartment {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Название отдела
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Id родительского отдела. Оставить пустым или \"-\", если это отдел верхнего уровня
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// Сотрудники на отделе и их роль. Возможные значения: <br/><div>1) manager или member</div><div>2) \"-\" или \"\" для удаления существующего пользователя из отдела</div>
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<serde_json::Value>,
}

impl UpdateDepartment {
    pub fn new() -> UpdateDepartment {
        UpdateDepartment {
            deleted: None,
            title: None,
            parent_id: None,
            users: None,
        }
    }
}
