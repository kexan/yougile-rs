use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Webhook {
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "url")]
    pub url: String,
    /// Событие подписки. Подписаться можно только на события в компании. Подписаться на личные чаты  не получится, потому что они не относятся к событиям в компании. Формат: `<тип_объекта>-<событие>`. Для  объектов `project,board,column,task,sticker,department,group_chat,chat_message`, возможные события: `created,deleted,restored,moved,renamed,updated`. Для объектов `user`,  возможные события: `added`, `removed`. Может использоваться javascript regexp как значение.  Например, `task-*` - подписка на все события по задачам, или `.*` - подписка на все события.
    #[serde(rename = "event")]
    pub event: String,
    /// Если true, то вызываться не будет
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Время последнего успешного вызова
    #[serde(rename = "lastSuccess", skip_serializing_if = "Option::is_none")]
    pub last_success: Option<f64>,
    /// Количество неуспешных вызовов. Сбрасывается до 0 при любом успешном вызове
    #[serde(rename = "failuresSinceLastSuccess")]
    pub failures_since_last_success: f64,
}

impl Webhook {
    pub fn new(
        id: String,
        url: String,
        event: String,
        failures_since_last_success: f64,
    ) -> Webhook {
        Webhook {
            id,
            deleted: None,
            url,
            event,
            disabled: None,
            last_success: None,
            failures_since_last_success,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateWebhook {
    #[serde(rename = "url")]
    pub url: String,
    /// Событие подписки. Подписаться можно только на события в компании. Подписаться на личные чаты  не получится, потому что они не относятся к событиям в компании. Формат: `<тип_объекта>-<событие>`. Для  объектов `project,board,column,task,sticker,department,group_chat,chat_message`, возможные события: `created,deleted,restored,moved,renamed,updated`. Для объектов `user`,  возможные события: `added`, `removed`. Может использоваться javascript regexp как значение.  Например, `task-*` - подписка на все события по задачам, или `.*` - подписка на все события.
    #[serde(rename = "event")]
    pub event: String,
}

impl CreateWebhook {
    pub fn new(url: String, event: String) -> CreateWebhook {
        CreateWebhook { url, event }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateWebhook {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Событие подписки. Подписаться можно только на события в компании. Подписаться на личные чаты  не получится, потому что они не относятся к событиям в компании. Формат: `<тип_объекта>-<событие>`. Для  объектов `project,board,column,task,sticker,department,group_chat,chat_message`, возможные события: `created,deleted,restored,moved,renamed,updated`. Для объектов `user`,  возможные события: `added`, `removed`. Может использоваться javascript regexp как значение.  Например, `task-*` - подписка на все события по задачам, или `.*` - подписка на все события.
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// Если true, то вызываться не будет
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

impl UpdateWebhook {
    pub fn new() -> UpdateWebhook {
        UpdateWebhook {
            deleted: None,
            url: None,
            event: None,
            disabled: None,
        }
    }
}
