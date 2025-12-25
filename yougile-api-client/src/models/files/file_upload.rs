use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileUpload {
    /// Результат загрузки
    #[serde(rename = "result")]
    pub result: String,
    /// URL файла
    #[serde(rename = "url")]
    pub url: String,
    /// Полный URL файла
    #[serde(rename = "fullUrl")]
    pub full_url: String,
}

impl FileUpload {
    pub fn new(result: String, url: String, full_url: String) -> FileUpload {
        FileUpload {
            result,
            url,
            full_url,
        }
    }
}
