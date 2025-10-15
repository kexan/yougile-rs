use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoardList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<models::PagingMetadata>,
    /// Список досок
    #[serde(rename = "content")]
    pub content: Vec<models::BoardListBase>,
}

impl BoardList {
    pub fn new(paging: models::PagingMetadata, content: Vec<models::BoardListBase>) -> BoardList {
        BoardList {
            paging: Box::new(paging),
            content,
        }
    }
}
