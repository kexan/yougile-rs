use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stickers {
    /// Таймер
    #[serde(rename = "timer", skip_serializing_if = "Option::is_none")]
    pub timer: Option<bool>,
    /// Дедлайн
    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<bool>,
    /// Секундомер
    #[serde(rename = "stopwatch", skip_serializing_if = "Option::is_none")]
    pub stopwatch: Option<bool>,
    /// Таймтрекинг
    #[serde(rename = "timeTracking", skip_serializing_if = "Option::is_none")]
    pub time_tracking: Option<bool>,
    /// Исполнитель
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<bool>,
    /// Регулярная задача
    #[serde(rename = "repeat", skip_serializing_if = "Option::is_none")]
    pub repeat: Option<bool>,
    /// Пользовательские стикеры доски
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, bool>>,
}

impl Stickers {
    pub fn new() -> Stickers {
        Stickers {
            timer: None,
            deadline: None,
            stopwatch: None,
            time_tracking: None,
            assignee: None,
            repeat: None,
            custom: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StickerStateId {
    /// ID состояния стикера
    #[serde(rename = "id")]
    pub id: String,
}

impl StickerStateId {
    pub fn new(id: String) -> StickerStateId {
        StickerStateId { id }
    }
}

/// Иконка стикера
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum Icon {
    #[serde(rename = "")]
    #[default]
    Empty,
    #[serde(rename = "star")]
    Star,
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "check")]
    Check,
    #[serde(rename = "cloud")]
    Cloud,
    #[serde(rename = "filter")]
    Filter,
    #[serde(rename = "alarm")]
    Alarm,
    #[serde(rename = "bolt")]
    Bolt,
    #[serde(rename = "bookmark")]
    Bookmark,
    #[serde(rename = "box")]
    Box,
    #[serde(rename = "bulb")]
    Bulb,
    #[serde(rename = "prio")]
    Prio,
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "ruble")]
    Ruble,
    #[serde(rename = "dollar")]
    Dollar,
    #[serde(rename = "euro")]
    Euro,
    #[serde(rename = "eye")]
    Eye,
    #[serde(rename = "flag")]
    Flag,
    #[serde(rename = "flame")]
    Flame,
    #[serde(rename = "history")]
    History,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "key")]
    Key,
    #[serde(rename = "anchor")]
    Anchor,
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "movie")]
    Movie,
    #[serde(rename = "mnote")]
    Mnote,
    #[serde(rename = "pencil")]
    Pencil,
    #[serde(rename = "picture")]
    Picture,
    #[serde(rename = "pin")]
    Pin,
    #[serde(rename = "clockwise")]
    Clockwise,
    #[serde(rename = "clockwiseDot")]
    ClockwiseDot,
    #[serde(rename = "rectangle")]
    Rectangle,
    #[serde(rename = "shield")]
    Shield,
    #[serde(rename = "stack")]
    Stack,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "timeStop")]
    TimeStop,
    #[serde(rename = "design")]
    Design,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "plus")]
    Plus,
    #[serde(rename = "gear")]
    Gear,
    #[serde(rename = "sort")]
    Sort,
    #[serde(rename = "calendar")]
    Calendar,
}

