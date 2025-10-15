use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatId {
    /// ID сообщения, также является временем создания
    #[serde(rename = "id")]
    pub id: f64,
}

impl ChatId {
    pub fn new(id: f64) -> ChatId {
        ChatId { id }
    }
}
