use crate::models::{PagingMetadata, common::Page};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SprintSticker {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub data: SprintStickerData,
    #[serde(rename = "states", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<SprintStickerState>>,
}

impl SprintSticker {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            data: SprintStickerData::new(name),
            states: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSprintSticker {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "states", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<SprintStateData>>,
}

impl CreateSprintSticker {
    pub fn new(name: String) -> Self {
        Self { name, states: None }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSprintSticker {
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SprintStickerData {
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "name")]
    pub name: String,
}

impl SprintStickerData {
    pub fn new(name: String) -> Self {
        Self {
            name,
            deleted: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SprintStickerState {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub data: SprintStateData,
}

impl SprintStickerState {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            data: SprintStateData::new(name),
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SprintStateData {
    /// Если true, значит объект удалён
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя состояния
    #[serde(rename = "name")]
    pub name: String,
    /// Дата начала спринта (Unix timestamp)
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<f64>,
    /// Дата окончания спринта (Unix timestamp)
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
}

impl SprintStateData {
    pub fn new(name: String) -> Self {
        Self {
            name,
            deleted: None,
            begin: None,
            end: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SprintStateUpdate {
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<f64>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
}

pub type SprintStickerList = Page<SprintSticker>;
