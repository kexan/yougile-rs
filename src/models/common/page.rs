use serde::{Deserialize, Serialize};

use crate::models::PagingMetadata;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Page<T> {
    #[serde(rename = "paging")]
    pub paging: Box<PagingMetadata>,
    #[serde(rename = "content")]
    pub content: Vec<T>,
}

impl<T> Page<T> {
    pub fn new(paging: PagingMetadata, content: Vec<T>) -> Self {
        Self {
            paging: Box::new(paging),
            content,
        }
    }
}
