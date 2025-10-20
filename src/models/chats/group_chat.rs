use crate::models::{self, PagingMetadata};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupChat {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Название чата
    #[serde(rename = "title")]
    pub title: String,
    /// Сотрудники в чате
    #[serde(rename = "users")]
    pub users: serde_json::Value,
    /// Роли сотрудников в чате
    #[serde(rename = "userRoleMap")]
    pub user_role_map: serde_json::Value,
    /// Настройки ролей
    #[serde(rename = "roleConfigMap")]
    pub role_config_map: serde_json::Value,
}

impl GroupChat {
    pub fn new(
        id: String,
        title: String,
        users: serde_json::Value,
        user_role_map: serde_json::Value,
        role_config_map: serde_json::Value,
    ) -> GroupChat {
        GroupChat {
            deleted: None,
            id,
            title,
            users,
            user_role_map,
            role_config_map,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateGroupChat {
    /// Название чата
    #[serde(rename = "title")]
    pub title: String,
    /// Сотрудники в чате
    #[serde(rename = "users")]
    pub users: serde_json::Value,
    /// Роли сотрудников в чате
    #[serde(rename = "userRoleMap")]
    pub user_role_map: serde_json::Value,
    /// Настройки ролей
    #[serde(rename = "roleConfigMap")]
    pub role_config_map: serde_json::Value,
}

impl CreateGroupChat {
    pub fn new(
        title: String,
        users: serde_json::Value,
        user_role_map: serde_json::Value,
        role_config_map: serde_json::Value,
    ) -> CreateGroupChat {
        CreateGroupChat {
            title,
            users,
            user_role_map,
            role_config_map,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateGroupChat {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Название чата
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Сотрудники в чате
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<serde_json::Value>,
    /// Роли сотрудников в чате
    #[serde(rename = "userRoleMap", skip_serializing_if = "Option::is_none")]
    pub user_role_map: Option<serde_json::Value>,
    /// Настройки ролей
    #[serde(rename = "roleConfigMap", skip_serializing_if = "Option::is_none")]
    pub role_config_map: Option<serde_json::Value>,
}

impl UpdateGroupChat {
    pub fn new() -> UpdateGroupChat {
        UpdateGroupChat {
            deleted: None,
            title: None,
            users: None,
            user_role_map: None,
            role_config_map: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupChatList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<models::PagingMetadata>,
    /// Список чатов
    #[serde(rename = "content")]
    pub content: Vec<GroupChat>,
}

impl GroupChatList {
    pub fn new(paging: models::PagingMetadata, content: Vec<GroupChat>) -> GroupChatList {
        GroupChatList {
            paging: Box::new(paging),
            content,
        }
    }
}

