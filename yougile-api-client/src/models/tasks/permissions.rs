use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskPermissions {
    #[serde(rename = "show")]
    pub show: bool,
    #[serde(rename = "delete")]
    pub delete: bool,
    #[serde(rename = "editTitle")]
    pub edit_title: bool,
    #[serde(rename = "editDescription")]
    pub edit_description: bool,
    #[serde(rename = "complete")]
    pub complete: bool,
    #[serde(rename = "close")]
    pub close: bool,
    #[serde(rename = "assignUsers")]
    pub assign_users: AssignUsers,
    #[serde(rename = "connect")]
    pub connect: bool,
    #[serde(rename = "editSubtasks")]
    pub edit_subtasks: EditSubtasks,
    #[serde(rename = "editStickers")]
    pub edit_stickers: bool,
    #[serde(rename = "editPins")]
    pub edit_pins: bool,
    #[serde(rename = "move")]
    pub r#move: Move,
    #[serde(rename = "sendMessages")]
    pub send_messages: bool,
    #[serde(rename = "sendFiles")]
    pub send_files: bool,
    #[serde(rename = "editWhoToNotify")]
    pub edit_who_to_notify: EditWhoToNotify,
}

impl TaskPermissions {
    pub fn new(
        show: bool,
        delete: bool,
        edit_title: bool,
        edit_description: bool,
        complete: bool,
        close: bool,
        assign_users: AssignUsers,
        connect: bool,
        edit_subtasks: EditSubtasks,
        edit_stickers: bool,
        edit_pins: bool,
        r#move: Move,
        send_messages: bool,
        send_files: bool,
        edit_who_to_notify: EditWhoToNotify,
    ) -> TaskPermissions {
        TaskPermissions {
            show,
            delete,
            edit_title,
            edit_description,
            complete,
            close,
            assign_users,
            connect,
            edit_subtasks,
            edit_stickers,
            edit_pins,
            r#move,
            send_messages,
            send_files,
            edit_who_to_notify,
        }
    }
}

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum AssignUsers {
    #[serde(rename = "no")]
    #[default]
    No,
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "add-self")]
    AddSelf,
    #[serde(rename = "set-self")]
    SetSelf,
    #[serde(rename = "change-from-self")]
    ChangeFromSelf,
}

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum EditSubtasks {
    #[serde(rename = "no")]
    #[default]
    No,
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "complete")]
    Complete,
}

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum Move {
    #[serde(rename = "no")]
    #[default]
    No,
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "board")]
    Board,
}

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum EditWhoToNotify {
    #[serde(rename = "no")]
    #[default]
    No,
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "self")]
    VariantSelf,
}
