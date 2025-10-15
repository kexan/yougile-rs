use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatMessageListBase {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// ID сообщения, также является временем создания
    #[serde(rename = "id")]
    pub id: f64,
    /// ID автора сообщения
    #[serde(rename = "fromUserId")]
    pub from_user_id: String,
    /// Текст сообщения
    #[serde(rename = "text")]
    pub text: String,
    /// Текст сообщения в формате HTML
    #[serde(rename = "textHtml")]
    pub text_html: String,
    /// Быстрая ссылка
    #[serde(rename = "label")]
    pub label: String,
    /// Время последнего редактирования
    #[serde(rename = "editTimestamp")]
    pub edit_timestamp: f64,
    /// Реакции на сообщение
    #[serde(rename = "reactions")]
    pub reactions: serde_json::Value,
}

impl ChatMessageListBase {
    pub fn new(
        id: f64,
        from_user_id: String,
        text: String,
        text_html: String,
        label: String,
        edit_timestamp: f64,
        reactions: serde_json::Value,
    ) -> ChatMessageListBase {
        ChatMessageListBase {
            deleted: None,
            id,
            from_user_id,
            text,
            text_html,
            label,
            edit_timestamp,
            reactions,
        }
    }
}
