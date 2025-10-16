use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Timer {
    /// Сколько секунд осталось до конца таймера.
    #[serde(rename = "seconds")]
    pub seconds: f64,
    /// Timestamp момента времени, от которого отсчитывается значение в поле seconds
    #[serde(rename = "since")]
    pub since: f64,
    /// Статус таймера - запущен/остановлен.
    #[serde(rename = "running")]
    pub running: bool,
}

impl Timer {
    pub fn new(seconds: f64, since: f64, running: bool) -> Timer {
        Timer {
            seconds,
            since,
            running,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTimer {
    /// Установить время таймера в секундах.
    #[serde(rename = "seconds")]
    pub seconds: f64,
    /// Запустить или остановить таймер.
    #[serde(rename = "running")]
    pub running: bool,
}

impl CreateTimer {
    pub fn new(seconds: f64, running: bool) -> CreateTimer {
        CreateTimer { seconds, running }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateTimer {
    /// Установить время таймера в секундах.
    #[serde(rename = "seconds", skip_serializing_if = "Option::is_none")]
    pub seconds: Option<f64>,
    /// Запустить или остановить таймер.
    #[serde(rename = "running", skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
    /// Открепить стикер от задачи (true)
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

impl UpdateTimer {
    pub fn new() -> UpdateTimer {
        UpdateTimer {
            seconds: None,
            running: None,
            deleted: None,
        }
    }
}
