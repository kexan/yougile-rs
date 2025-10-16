use crate::models::{self, PagingMetadata};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringStickerState {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// ID состояния стикера
    #[serde(rename = "id")]
    pub id: String,
    /// Имя состояния стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Цвет состояния стикера
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl StringStickerState {
    pub fn new(id: String, name: String) -> StringStickerState {
        StringStickerState {
            deleted: None,
            id,
            name,
            color: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateStringSticker {
    /// Имя стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Иконка стикера
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<super::Icon>,
    /// Состояния стикера.
    #[serde(rename = "states", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<StringStickerStateNoId>>,
}

impl CreateStringSticker {
    pub fn new(name: String) -> CreateStringSticker {
        CreateStringSticker {
            name,
            icon: None,
            states: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateStringStickerState {
    /// Имя состояния стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Цвет состояния стикера
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl CreateStringStickerState {
    pub fn new(name: String) -> CreateStringStickerState {
        CreateStringStickerState { name, color: None }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringStickerWithStates {
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Иконка стикера
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<super::Icon>,
    /// Состояния стикера.
    #[serde(rename = "states", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<StringStickerState>>,
    /// Количество элементов, которые хочется получить. Максимум 1000.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    /// Индекс первого элемента страницы
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
}

impl StringStickerWithStates {
    pub fn new(id: String, name: String) -> StringStickerWithStates {
        StringStickerWithStates {
            id,
            deleted: None,
            name,
            icon: None,
            states: None,
            limit: None,
            offset: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringStickerWithStatesList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<PagingMetadata>,
    /// Список текстовых стикеров компании
    #[serde(rename = "content")]
    pub content: Vec<StringStickerWithStatesListBase>,
}

impl StringStickerWithStatesList {
    pub fn new(
        paging: PagingMetadata,
        content: Vec<StringStickerWithStatesListBase>,
    ) -> StringStickerWithStatesList {
        StringStickerWithStatesList {
            paging: Box::new(paging),
            content,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateStringSticker {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя стикера
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Иконка стикера
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<super::Icon>,
}

impl UpdateStringSticker {
    pub fn new() -> UpdateStringSticker {
        UpdateStringSticker {
            deleted: None,
            name: None,
            icon: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateStringStickerState {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя состояния стикера
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Цвет состояния стикера
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl UpdateStringStickerState {
    pub fn new() -> UpdateStringStickerState {
        UpdateStringStickerState {
            deleted: None,
            name: None,
            color: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringStickerStateNoId {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя состояния стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Цвет состояния стикера
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl StringStickerStateNoId {
    pub fn new(name: String) -> StringStickerStateNoId {
        StringStickerStateNoId {
            deleted: None,
            name,
            color: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringStickerWithStatesListBase {
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Иконка стикера
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<super::Icon>,
    /// Состояния стикера.
    #[serde(rename = "states", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<StringStickerState>>,
    /// Количество элементов, которые хочется получить. Максимум 1000.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    /// Индекс первого элемента страницы
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
}

impl StringStickerWithStatesListBase {
    pub fn new(id: String, name: String) -> StringStickerWithStatesListBase {
        StringStickerWithStatesListBase {
            id,
            deleted: None,
            name,
            icon: None,
            states: None,
            limit: None,
            offset: None,
        }
    }
}

