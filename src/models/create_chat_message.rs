use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateChatMessage {
    /// Текст сообщения
    #[serde(rename = "text")]
    pub text: String,
    /// Текст сообщения в формате HTML
    #[serde(rename = "textHtml")]
    pub text_html: String,
    /// Быстрая ссылка
    #[serde(rename = "label")]
    pub label: String,
}

impl CreateChatMessage {
    pub fn new(text: String, text_html: String, label: String) -> CreateChatMessage {
        CreateChatMessage {
            text,
            text_html,
            label,
        }
    }
}
