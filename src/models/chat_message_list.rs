use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatMessageList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<models::PagingMetadata>,
    /// История сообщений
    #[serde(rename = "content")]
    pub content: Vec<models::ChatMessageListBase>,
}

impl ChatMessageList {
    pub fn new(
        paging: models::PagingMetadata,
        content: Vec<models::ChatMessageListBase>,
    ) -> ChatMessageList {
        ChatMessageList {
            paging: Box::new(paging),
            content,
        }
    }
}
