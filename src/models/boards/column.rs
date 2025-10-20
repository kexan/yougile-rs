use crate::models::{self, tasks::TaskPermissions};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column {
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Название колонки
    #[serde(rename = "title")]
    pub title: String,
    /// Цвет колонки. Указывается в виде числа. Примеры цветов представлены ниже <br/><div>1 - #7B869E <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #7B869E       \">  </div> </div><div>2 - #FF8C8C <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #FF8C8C       \">  </div> </div><div>3 - #E9A24F <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #E9A24F       \">  </div> </div><div>4 - #FCE258 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #FCE258       \">  </div> </div><div>5 - #7CAE5E <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #7CAE5E       \">  </div> </div><div>6 - #49C5BC <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #49C5BC       \">  </div> </div><div>7 - #8CACFF <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #8CACFF       \">  </div> </div><div>8 - #CC8CFF <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #CC8CFF       \">  </div> </div><div>9 - #667085 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #667085       \">  </div> </div><div>10 - #EB3737 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #EB3737       \">  </div> </div><div>11 - #F2732B <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #F2732B       \">  </div> </div><div>12 - #F5CC00 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #F5CC00       \">  </div> </div><div>13 - #5CDC11 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #5CDC11       \">  </div> </div><div>14 - #08A7A9 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #08A7A9       \">  </div> </div><div>15 - #5089F2 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #5089F2       \">  </div> </div><div>16 - #E25EF2 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #E25EF2       \">  </div> </div>
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<f64>,
    /// Id доски, в которой находится колонка
    #[serde(rename = "boardId")]
    pub board_id: String,
}

impl Column {
    pub fn new(id: String, title: String, board_id: String) -> Column {
        Column {
            id,
            deleted: None,
            title,
            color: None,
            board_id,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<models::PagingMetadata>,
    /// Список колонок
    #[serde(rename = "content")]
    pub content: Vec<Column>,
}

impl ColumnList {
    pub fn new(paging: models::PagingMetadata, content: Vec<Column>) -> ColumnList {
        ColumnList {
            paging: Box::new(paging),
            content,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateColumn {
    /// Название колонки
    #[serde(rename = "title")]
    pub title: String,
    /// Цвет колонки. Указывается в виде числа. Примеры цветов представлены ниже <br/><div>1 - #7B869E <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #7B869E       \">  </div> </div><div>2 - #FF8C8C <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #FF8C8C       \">  </div> </div><div>3 - #E9A24F <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #E9A24F       \">  </div> </div><div>4 - #FCE258 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #FCE258       \">  </div> </div><div>5 - #7CAE5E <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #7CAE5E       \">  </div> </div><div>6 - #49C5BC <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #49C5BC       \">  </div> </div><div>7 - #8CACFF <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #8CACFF       \">  </div> </div><div>8 - #CC8CFF <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #CC8CFF       \">  </div> </div><div>9 - #667085 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #667085       \">  </div> </div><div>10 - #EB3737 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #EB3737       \">  </div> </div><div>11 - #F2732B <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #F2732B       \">  </div> </div><div>12 - #F5CC00 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #F5CC00       \">  </div> </div><div>13 - #5CDC11 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #5CDC11       \">  </div> </div><div>14 - #08A7A9 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #08A7A9       \">  </div> </div><div>15 - #5089F2 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #5089F2       \">  </div> </div><div>16 - #E25EF2 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #E25EF2       \">  </div> </div>
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<f64>,
    /// Id доски, в которой находится колонка
    #[serde(rename = "boardId")]
    pub board_id: String,
}

impl CreateColumn {
    pub fn new(title: String, board_id: String) -> CreateColumn {
        CreateColumn {
            title,
            color: None,
            board_id,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateColumn {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Название колонки
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Цвет колонки. Указывается в виде числа. Примеры цветов представлены ниже <br/><div>1 - #7B869E <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #7B869E       \">  </div> </div><div>2 - #FF8C8C <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #FF8C8C       \">  </div> </div><div>3 - #E9A24F <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #E9A24F       \">  </div> </div><div>4 - #FCE258 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #FCE258       \">  </div> </div><div>5 - #7CAE5E <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #7CAE5E       \">  </div> </div><div>6 - #49C5BC <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #49C5BC       \">  </div> </div><div>7 - #8CACFF <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #8CACFF       \">  </div> </div><div>8 - #CC8CFF <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #CC8CFF       \">  </div> </div><div>9 - #667085 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #667085       \">  </div> </div><div>10 - #EB3737 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #EB3737       \">  </div> </div><div>11 - #F2732B <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #F2732B       \">  </div> </div><div>12 - #F5CC00 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #F5CC00       \">  </div> </div><div>13 - #5CDC11 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #5CDC11       \">  </div> </div><div>14 - #08A7A9 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #08A7A9       \">  </div> </div><div>15 - #5089F2 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #5089F2       \">  </div> </div><div>16 - #E25EF2 <div style=\"         display: inline-block;          width: 10px;          height: 10px;         background-color: #E25EF2       \">  </div> </div>
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<f64>,
    /// Id доски, в которой находится колонка
    #[serde(rename = "boardId", skip_serializing_if = "Option::is_none")]
    pub board_id: Option<String>,
}

impl UpdateColumn {
    pub fn new() -> UpdateColumn {
        UpdateColumn {
            deleted: None,
            title: None,
            color: None,
            board_id: None,
        }
    }
}
