use serde::{Deserialize, Serialize};

use crate::models::{self, PagingMetadata};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatMessage {
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

impl ChatMessage {
    pub fn new(
        id: f64,
        from_user_id: String,
        text: String,
        text_html: String,
        label: String,
        edit_timestamp: f64,
        reactions: serde_json::Value,
    ) -> ChatMessage {
        ChatMessage {
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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatMessageList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<PagingMetadata>,
    /// История сообщений
    #[serde(rename = "content")]
    pub content: Vec<ChatMessageListBase>,
}

impl ChatMessageList {
    pub fn new(paging: PagingMetadata, content: Vec<ChatMessageListBase>) -> ChatMessageList {
        ChatMessageList {
            paging: Box::new(paging),
            content,
        }
    }
}

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
