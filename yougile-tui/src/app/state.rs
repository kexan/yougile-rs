use std::collections::HashMap;
use yougile_client::models::{Column, Task};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Projects,
    Boards,
    Tasks,
    TaskDetail,
    Help,
}

pub struct ColumnWithTasks {
    pub column: Column,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
pub struct StickerMeta {
    pub id: String,
    pub title: String,
    pub states: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusedWidget {
    ProjectList,
    BoardList,
    ColumnList,
}
