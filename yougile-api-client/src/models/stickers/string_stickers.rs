use crate::{
    SprintSticker,
    models::{self, PagingMetadata, common::Page},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringStateData {
    /// Если true, значит объект удалён
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя состояния
    #[serde(rename = "name")]
    pub name: String,
    /// Цвет состояния
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl StringStateData {
    pub fn new(name: String) -> Self {
        Self {
            name,
            deleted: None,
            color: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringStateUpdate {
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringStickerState {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub data: StringStateData,
}

impl StringStickerState {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            data: StringStateData::new(name),
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringStickerData {
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<super::Icon>,
}

impl StringStickerData {
    pub fn new(name: String) -> Self {
        Self {
            name,
            deleted: None,
            icon: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringSticker {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub data: StringStickerData,
    #[serde(rename = "states", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<StringStickerState>>,
}

impl StringSticker {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            data: StringStickerData::new(name),
            states: None,
        }
    }
}

pub type StringStickerList = Page<StringSticker>;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateStringSticker {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<super::Icon>,
    #[serde(rename = "states", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<StringStateData>>, // ← без ID!
}

impl CreateStringSticker {
    pub fn new(name: String) -> Self {
        Self {
            name,
            icon: None,
            states: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateStringSticker {
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<super::Icon>,
}
