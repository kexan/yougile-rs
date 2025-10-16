use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagingMetadata {
    /// Количество элементов в результате
    #[serde(rename = "count")]
    pub count: f64,
    /// Количество элементов на страницу
    #[serde(rename = "limit")]
    pub limit: f64,
    /// Индекс первого элемента страницы
    #[serde(rename = "offset")]
    pub offset: f64,
    /// Есть ли элементы после данной страницы
    #[serde(rename = "next")]
    pub next: bool,
}

impl PagingMetadata {
    pub fn new(count: f64, limit: f64, offset: f64, next: bool) -> PagingMetadata {
        PagingMetadata {
            count,
            limit,
            offset,
            next,
        }
    }
}
