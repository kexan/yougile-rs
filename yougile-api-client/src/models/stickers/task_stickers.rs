use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type for stickers in Task - a mapping from sticker ID to their values
/// Values can be state IDs or string values depending on the sticker type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickerValue {
    StateId(String),  // For stickers with states (state ID)
    Text(String),     // For free text field stickers
    Number(f64),      // For free numeric field stickers
    Empty,            // For empty stickers
    Unassigned,       // For unassigned stickers (represented by "-" in API)
}

pub type TaskStickers = HashMap<String, StickerValue>;

impl Default for StickerValue {
    fn default() -> Self {
        StickerValue::Text(String::new())
    }
}

impl From<String> for StickerValue {
    fn from(value: String) -> Self {
        StickerValue::Text(value)
    }
}

impl From<f64> for StickerValue {
    fn from(value: f64) -> Self {
        StickerValue::Number(value)
    }
}