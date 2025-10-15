use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateChatMessage {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Быстрая ссылка
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Список реакций админа
    #[serde(rename = "react", skip_serializing_if = "Option::is_none")]
    pub react: Option<React>,
}

impl UpdateChatMessage {
    pub fn new() -> UpdateChatMessage {
        UpdateChatMessage {
            deleted: None,
            label: None,
            react: None,
        }
    }
}
/// Список реакций админа
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum React {
    #[serde(rename = "👍")]
    #[default]
    ThumbsUp,
    #[serde(rename = "👎")]
    ThumbsDown,
    #[serde(rename = "👏")]
    Clap,
    #[serde(rename = "🙂")]
    SlightlySmiling,
    #[serde(rename = "😀")]
    Grinning,
    #[serde(rename = "😕")]
    Confused,
    #[serde(rename = "🎉")]
    Tada,
    #[serde(rename = "❤")]
    Heart,
    #[serde(rename = "🚀")]
    Rocket,
    #[serde(rename = "✔")]
    CheckMark,
}
