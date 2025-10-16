use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeTracking {
    /// Сколько часов было запланировано на выполнение задачи
    #[serde(rename = "plan")]
    pub plan: f64,
    /// Сколько часов было затрачено на выполнение задачи
    #[serde(rename = "work")]
    pub work: f64,
}

impl TimeTracking {
    pub fn new(plan: f64, work: f64) -> TimeTracking {
        TimeTracking { plan, work }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateTimeTracking {
    /// Сколько часов было запланировано на выполнение задачи
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<f64>,
    /// Сколько часов было затрачено на выполнение задачи
    #[serde(rename = "work", skip_serializing_if = "Option::is_none")]
    pub work: Option<f64>,
    /// Открепить стикер от задачи (true)
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

impl UpdateTimeTracking {
    pub fn new() -> UpdateTimeTracking {
        UpdateTimeTracking {
            plan: None,
            work: None,
            deleted: None,
        }
    }
}
