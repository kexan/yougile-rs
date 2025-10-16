use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Board {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Название доски
    #[serde(rename = "title")]
    pub title: String,
    /// ID проекта, в котором находится доска
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// Стикеры доски
    #[serde(rename = "stickers", skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Box<models::StickersDto>>,
}

impl Board {
    pub fn new(id: String, title: String, project_id: String) -> Board {
        Board {
            deleted: None,
            id,
            title,
            project_id,
            stickers: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoardList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<models::PagingMetadata>,
    /// Список досок
    #[serde(rename = "content")]
    pub content: Vec<models::BoardListBase>,
}

impl BoardList {
    pub fn new(paging: models::PagingMetadata, content: Vec<models::BoardListBase>) -> BoardList {
        BoardList {
            paging: Box::new(paging),
            content,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoardListBase {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Название доски
    #[serde(rename = "title")]
    pub title: String,
    /// ID проекта, в котором находится доска
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// Стикеры доски
    #[serde(rename = "stickers", skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Box<models::StickersDto>>,
}

impl BoardListBase {
    pub fn new(id: String, title: String, project_id: String) -> BoardListBase {
        BoardListBase {
            deleted: None,
            id,
            title,
            project_id,
            stickers: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateBoard {
    /// Название доски
    #[serde(rename = "title")]
    pub title: String,
    /// ID проекта, в котором находится доска
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// Стикеры доски
    #[serde(rename = "stickers", skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Box<models::StickersDto>>,
}

impl CreateBoard {
    pub fn new(title: String, project_id: String) -> CreateBoard {
        CreateBoard {
            title,
            project_id,
            stickers: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateBoard {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Название доски
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// ID проекта, в котором находится доска
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Стикеры доски
    #[serde(rename = "stickers", skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Box<models::StickersDto>>,
}

impl UpdateBoard {
    pub fn new() -> UpdateBoard {
        UpdateBoard {
            deleted: None,
            title: None,
            project_id: None,
            stickers: None,
        }
    }
}
