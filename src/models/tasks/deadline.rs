use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Deadline {
    /// Timestamp дэдлайна
    #[serde(rename = "deadline")]
    pub deadline: f64,
    /// Timestamp начала задачи
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    /// Отображать на стикере время, или только дату
    #[serde(rename = "withTime", skip_serializing_if = "Option::is_none")]
    pub with_time: Option<bool>,
    /// История изменений дедлайна
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<String>>,
    /// Точки, которые блокируют дату дедлайна (Начало или Конец)
    #[serde(rename = "blockedPoints")]
    pub blocked_points: Vec<String>,
    /// Связанные задачи
    #[serde(rename = "links")]
    pub links: Vec<String>,
}

impl Deadline {
    pub fn new(deadline: f64, blocked_points: Vec<String>, links: Vec<String>) -> Deadline {
        Deadline {
            deadline,
            start_date: None,
            with_time: None,
            history: None,
            blocked_points,
            links,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDeadline {
    /// Timestamp дэдлайна
    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<f64>,
    /// Timestamp начала задачи
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    /// Отображать на стикере время, или только дату
    #[serde(rename = "withTime", skip_serializing_if = "Option::is_none")]
    pub with_time: Option<bool>,
    /// История изменений дедлайна
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<String>>,
    /// Точки, которые блокируют дату дедлайна (Начало или Конец)
    #[serde(rename = "blockedPoints")]
    pub blocked_points: Vec<String>,
    /// Связанные задачи
    #[serde(rename = "links")]
    pub links: Vec<String>,
    /// Открепить стикер от задачи (true)
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Прикрепить стикер дедлайна без значения
    #[serde(rename = "empty", skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
}

impl UpdateDeadline {
    pub fn new(blocked_points: Vec<String>, links: Vec<String>) -> UpdateDeadline {
        UpdateDeadline {
            deadline: None,
            start_date: None,
            with_time: None,
            history: None,
            blocked_points,
            links,
            deleted: None,
            empty: None,
        }
    }
}
