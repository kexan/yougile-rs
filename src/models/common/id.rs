use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Id {
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
}

impl Id {
    pub fn new(id: String) -> Id {
        Id { id }
    }
}
