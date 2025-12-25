use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stopwatch {
    /// Статус секундомера - запущен/остановлен
    #[serde(rename = "running")]
    pub running: bool,
    /// Сколько секунд прошло, пока таймер был запущен.
    #[serde(rename = "seconds")]
    pub seconds: f64,
    /// Момент времени, на который значение seconds было актуально
    #[serde(rename = "atMoment")]
    pub at_moment: f64,
}

impl Stopwatch {
    pub fn new(running: bool, seconds: f64, at_moment: f64) -> Stopwatch {
        Stopwatch {
            running,
            seconds,
            at_moment,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateStopwatch {
    /// Запустить или остановить секундомер
    #[serde(rename = "running")]
    pub running: bool,
}

impl CreateStopwatch {
    pub fn new(running: bool) -> CreateStopwatch {
        CreateStopwatch { running }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateStopwatch {
    /// Запустить или остановить секундомер
    #[serde(rename = "running", skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
    /// Открепить стикер от задачи (true)
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

impl UpdateStopwatch {
    pub fn new() -> UpdateStopwatch {
        UpdateStopwatch {
            running: None,
            deleted: None,
        }
    }
}
