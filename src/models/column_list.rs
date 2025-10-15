use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<models::PagingMetadata>,
    /// Список колонок
    #[serde(rename = "content")]
    pub content: Vec<models::ColumnListBase>,
}

impl ColumnList {
    pub fn new(paging: models::PagingMetadata, content: Vec<models::ColumnListBase>) -> ColumnList {
        ColumnList {
            paging: Box::new(paging),
            content,
        }
    }
}
