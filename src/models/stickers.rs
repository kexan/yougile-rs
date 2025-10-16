use crate::models::{self, PagingMetadata};
use serde::{Deserialize, Serialize};

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
    pub custom: Option<serde_json::Value>,
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
pub struct SprintStickerState {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// ID состояния стикера
    #[serde(rename = "id")]
    pub id: String,
    /// Имя состояния стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Дата начала спринта в секундах от 01.01.1970
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<f64>,
    /// Дата окончания спринта в секундах от 01.01.1970
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
}

impl SprintStickerState {
    pub fn new(id: String, name: String) -> SprintStickerState {
        SprintStickerState {
            deleted: None,
            id,
            name,
            begin: None,
            end: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SprintStickerWithStates {
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Состояния стикера.
    #[serde(rename = "states", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<SprintStickerState>>,
}

impl SprintStickerWithStates {
    pub fn new(id: String, name: String) -> SprintStickerWithStates {
        SprintStickerWithStates {
            id,
            deleted: None,
            name,
            states: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SprintStickerStateNoId {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя состояния стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Дата начала спринта в секундах от 01.01.1970
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<f64>,
    /// Дата окончания спринта в секундах от 01.01.1970
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
}

impl SprintStickerStateNoId {
    pub fn new(name: String) -> SprintStickerStateNoId {
        SprintStickerStateNoId {
            deleted: None,
            name,
            begin: None,
            end: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SprintStickerWithStatesList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<PagingMetadata>,
    /// Список спринтовых стикеров компании
    #[serde(rename = "content")]
    pub content: Vec<SprintStickerWithStatesListBase>,
}

impl SprintStickerWithStatesList {
    pub fn new(
        paging: PagingMetadata,
        content: Vec<SprintStickerWithStatesListBase>,
    ) -> SprintStickerWithStatesList {
        SprintStickerWithStatesList {
            paging: Box::new(paging),
            content,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SprintStickerWithStatesListBase {
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Состояния стикера.
    #[serde(rename = "states", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<SprintStickerState>>,
}

impl SprintStickerWithStatesListBase {
    pub fn new(id: String, name: String) -> SprintStickerWithStatesListBase {
        SprintStickerWithStatesListBase {
            id,
            deleted: None,
            name,
            states: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSprintSticker {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя стикера
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateSprintSticker {
    pub fn new() -> UpdateSprintSticker {
        UpdateSprintSticker {
            deleted: None,
            name: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSprintStickerState {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя состояния стикера
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Дата начала спринта в секундах от 01.01.1970
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<f64>,
    /// Дата окончания спринта в секундах от 01.01.1970
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
}

impl UpdateSprintStickerState {
    pub fn new() -> UpdateSprintStickerState {
        UpdateSprintStickerState {
            deleted: None,
            name: None,
            begin: None,
            end: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSprintSticker {
    /// Имя стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Состояния стикера.
    #[serde(rename = "states", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<SprintStickerStateNoId>>,
}

impl CreateSprintSticker {
    pub fn new(name: String) -> CreateSprintSticker {
        CreateSprintSticker { name, states: None }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSprintStickerState {
    /// Имя состояния стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Дата начала спринта в секундах от 01.01.1970
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<f64>,
    /// Дата окончания спринта в секундах от 01.01.1970
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
}

impl CreateSprintStickerState {
    pub fn new(name: String) -> CreateSprintStickerState {
        CreateSprintStickerState {
            name,
            begin: None,
            end: None,
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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringStickerState {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// ID состояния стикера
    #[serde(rename = "id")]
    pub id: String,
    /// Имя состояния стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Цвет состояния стикера
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl StringStickerState {
    pub fn new(id: String, name: String) -> StringStickerState {
        StringStickerState {
            deleted: None,
            id,
            name,
            color: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateStringSticker {
    /// Имя стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Иконка стикера
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    /// Состояния стикера.
    #[serde(rename = "states", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<models::StringStickerStateNoId>>,
}

impl CreateStringSticker {
    pub fn new(name: String) -> CreateStringSticker {
        CreateStringSticker {
            name,
            icon: None,
            states: None,
        }
    }
}
/// Иконка стикера
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Icon {
    #[serde(rename = "")]
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

impl Default for Icon {
    fn default() -> Icon {
        Self::Empty
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateStringStickerState {
    /// Имя состояния стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Цвет состояния стикера
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl CreateStringStickerState {
    pub fn new(name: String) -> CreateStringStickerState {
        CreateStringStickerState { name, color: None }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringStickerWithStates {
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Иконка стикера
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    /// Состояния стикера.
    #[serde(rename = "states", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<models::StringStickerState>>,
    /// Количество элементов, которые хочется получить. Максимум 1000.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    /// Индекс первого элемента страницы
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
}

impl StringStickerWithStates {
    pub fn new(id: String, name: String) -> StringStickerWithStates {
        StringStickerWithStates {
            id,
            deleted: None,
            name,
            icon: None,
            states: None,
            limit: None,
            offset: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringStickerWithStatesList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<models::PagingMetadata>,
    /// Список текстовых стикеров компании
    #[serde(rename = "content")]
    pub content: Vec<models::StringStickerWithStatesListBase>,
}

impl StringStickerWithStatesList {
    pub fn new(
        paging: models::PagingMetadata,
        content: Vec<models::StringStickerWithStatesListBase>,
    ) -> StringStickerWithStatesList {
        StringStickerWithStatesList {
            paging: Box::new(paging),
            content,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateStringSticker {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя стикера
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Иконка стикера
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
}

impl UpdateStringSticker {
    pub fn new() -> UpdateStringSticker {
        UpdateStringSticker {
            deleted: None,
            name: None,
            icon: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateStringStickerState {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя состояния стикера
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Цвет состояния стикера
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl UpdateStringStickerState {
    pub fn new() -> UpdateStringStickerState {
        UpdateStringStickerState {
            deleted: None,
            name: None,
            color: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringStickerStateNoId {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя состояния стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Цвет состояния стикера
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl StringStickerStateNoId {
    pub fn new(name: String) -> StringStickerStateNoId {
        StringStickerStateNoId {
            deleted: None,
            name,
            color: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringStickerWithStatesListBase {
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Имя стикера
    #[serde(rename = "name")]
    pub name: String,
    /// Иконка стикера
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    /// Состояния стикера.
    #[serde(rename = "states", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<models::StringStickerState>>,
    /// Количество элементов, которые хочется получить. Максимум 1000.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    /// Индекс первого элемента страницы
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
}

impl StringStickerWithStatesListBase {
    pub fn new(id: String, name: String) -> StringStickerWithStatesListBase {
        StringStickerWithStatesListBase {
            id,
            deleted: None,
            name,
            icon: None,
            states: None,
            limit: None,
            offset: None,
        }
    }
}
