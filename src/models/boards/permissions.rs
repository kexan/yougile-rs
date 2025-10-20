use crate::models::{stickers::Stickers, tasks::TaskPermissions};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoardPermissions {
    #[serde(rename = "editTitle")]
    pub edit_title: bool,
    #[serde(rename = "delete")]
    pub delete: bool,
    #[serde(rename = "move")]
    pub r#move: bool,
    #[serde(rename = "showStickers")]
    pub show_stickers: bool,
    #[serde(rename = "editStickers")]
    pub edit_stickers: bool,
    #[serde(rename = "addColumn")]
    pub add_column: bool,
    #[serde(rename = "columns")]
    pub columns: ColumnPermissions,
    #[serde(rename = "settings")]
    pub settings: bool,
}

impl BoardPermissions {
    pub fn new(
        edit_title: bool,
        delete: bool,
        r#move: bool,
        show_stickers: bool,
        edit_stickers: bool,
        add_column: bool,
        columns: ColumnPermissions,
        settings: bool,
    ) -> BoardPermissions {
        BoardPermissions {
            edit_title,
            delete,
            r#move,
            show_stickers,
            edit_stickers,
            add_column,
            columns,
            settings,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnPermissions {
    #[serde(rename = "editTitle")]
    pub edit_title: bool,
    #[serde(rename = "delete")]
    pub delete: bool,
    #[serde(rename = "move")]
    pub r#move: Move,
    #[serde(rename = "addTask")]
    pub add_task: bool,
    #[serde(rename = "allTasks")]
    pub all_tasks: TaskPermissions,
    #[serde(rename = "withMeTasks")]
    pub with_me_tasks: TaskPermissions,
    #[serde(rename = "myTasks")]
    pub my_tasks: TaskPermissions,
    #[serde(rename = "createdByMeTasks")]
    pub created_by_me_tasks: TaskPermissions,
}

impl ColumnPermissions {
    pub fn new(
        edit_title: bool,
        delete: bool,
        r#move: Move,
        add_task: bool,
        all_tasks: TaskPermissions,
        with_me_tasks: TaskPermissions,
        my_tasks: TaskPermissions,
        created_by_me_tasks: TaskPermissions,
    ) -> ColumnPermissions {
        ColumnPermissions {
            edit_title,
            delete,
            r#move,
            add_task,
            all_tasks,
            with_me_tasks,
            my_tasks,
            created_by_me_tasks,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Move {
    #[serde(rename = "no")]
    No,
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "yes")]
    Yes,
}

impl Default for Move {
    fn default() -> Move {
        Self::No
    }
}

