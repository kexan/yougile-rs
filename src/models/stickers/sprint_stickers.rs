use crate::models::{self, PagingMetadata};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SprintStickerState {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// ID состояния стикера
    #[serde(rename = "id")]
    pub id: String,
    /// Имя состояния стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Дата начала спринта в секундах от 01.01.1970
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<f64>,
    /// Дата окончания спринта в секундах от 01.01.1970
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
}

impl SprintStickerState {
    pub fn new(id: String, name: String) -> SprintStickerState {
        SprintStickerState {
            deleted: None,
            id,
            name,
            begin: None,
            end: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SprintStickerWithStates {
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Состояния стикера.
    #[serde(rename = "states", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<SprintStickerState>>,
}

impl SprintStickerWithStates {
    pub fn new(id: String, name: String) -> SprintStickerWithStates {
        SprintStickerWithStates {
            id,
            deleted: None,
            name,
            states: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SprintStickerStateNoId {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя состояния стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Дата начала спринта в секундах от 01.01.1970
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<f64>,
    /// Дата окончания спринта в секундах от 01.01.1970
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
}

impl SprintStickerStateNoId {
    pub fn new(name: String) -> SprintStickerStateNoId {
        SprintStickerStateNoId {
            deleted: None,
            name,
            begin: None,
            end: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SprintStickerWithStatesList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<PagingMetadata>,
    /// Список спринтовых стикеров компании
    #[serde(rename = "content")]
    pub content: Vec<SprintStickerWithStates>,
}

impl SprintStickerWithStatesList {
    pub fn new(
        paging: PagingMetadata,
        content: Vec<SprintStickerWithStates>,
    ) -> SprintStickerWithStatesList {
        SprintStickerWithStatesList {
            paging: Box::new(paging),
            content,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSprintSticker {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя стикера
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateSprintSticker {
    pub fn new() -> UpdateSprintSticker {
        UpdateSprintSticker {
            deleted: None,
            name: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSprintStickerState {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя состояния стикера
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Дата начала спринта в секундах от 01.01.1970
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<f64>,
    /// Дата окончания спринта в секундах от 01.01.1970
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
}

impl UpdateSprintStickerState {
    pub fn new() -> UpdateSprintStickerState {
        UpdateSprintStickerState {
            deleted: None,
            name: None,
            begin: None,
            end: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSprintSticker {
    /// Имя стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Состояния стикера.
    #[serde(rename = "states", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<SprintStickerStateNoId>>,
}

impl CreateSprintSticker {
    pub fn new(name: String) -> CreateSprintSticker {
        CreateSprintSticker { name, states: None }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSprintStickerState {
    /// Имя состояния стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Дата начала спринта в секундах от 01.01.1970
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<f64>,
    /// Дата окончания спринта в секундах от 01.01.1970
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
}

impl CreateSprintStickerState {
    pub fn new(name: String) -> CreateSprintStickerState {
        CreateSprintStickerState {
            name,
            begin: None,
            end: None,
        }
    }
}
