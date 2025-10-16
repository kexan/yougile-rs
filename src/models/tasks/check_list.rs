use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckList {
    /// Название списка чеклистов
    #[serde(rename = "title")]
    pub title: String,
    /// Массив с чеклистами
    #[serde(rename = "items")]
    pub items: Vec<CheckListItem>,
}

impl CheckList {
    pub fn new(title: String, items: Vec<CheckListItem>) -> CheckList {
        CheckList { title, items }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckListItem {
    /// Название чеклиста
    #[serde(rename = "title")]
    pub title: String,
    /// Выполненность чеклиста
    #[serde(rename = "isCompleted")]
    pub is_completed: bool,
}

impl CheckListItem {
    pub fn new(title: String, is_completed: bool) -> CheckListItem {
        CheckListItem {
            title,
            is_completed,
        }
    }
}
