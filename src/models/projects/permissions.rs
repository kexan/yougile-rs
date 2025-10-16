use crate::models::{self, boards::BoardPermissions};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectPermissions {
    #[serde(rename = "editTitle")]
    pub edit_title: bool,
    #[serde(rename = "delete")]
    pub delete: bool,
    #[serde(rename = "addBoard")]
    pub add_board: bool,
    #[serde(rename = "boards")]
    pub boards: Box<BoardPermissions>,
    #[serde(rename = "children")]
    pub children: serde_json::Value,
}

impl ProjectPermissions {
    pub fn new(
        edit_title: bool,
        delete: bool,
        add_board: bool,
        boards: BoardPermissions,
        children: serde_json::Value,
    ) -> ProjectPermissions {
        ProjectPermissions {
            edit_title,
            delete,
            add_board,
            boards: Box::new(boards),
            children,
        }
    }
}