use crate::models::{self, PagingMetadata, common::Page};
use crate::models::chats::{UsersList, UserRoleMap, RoleConfigMap};
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
    pub users: UsersList,
    /// Роли сотрудников в чате
    #[serde(rename = "userRoleMap")]
    pub user_role_map: UserRoleMap,
    /// Настройки ролей
    #[serde(rename = "roleConfigMap")]
    pub role_config_map: RoleConfigMap,
}

impl GroupChat {
    pub fn new(
        id: String,
        title: String,
        users: UsersList,
        user_role_map: UserRoleMap,
        role_config_map: RoleConfigMap,
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
    pub users: UsersList,
    /// Роли сотрудников в чате
    #[serde(rename = "userRoleMap")]
    pub user_role_map: UserRoleMap,
    /// Настройки ролей
    #[serde(rename = "roleConfigMap")]
    pub role_config_map: RoleConfigMap,
}

impl CreateGroupChat {
    pub fn new(
        title: String,
        users: UsersList,
        user_role_map: UserRoleMap,
        role_config_map: RoleConfigMap,
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
    pub users: Option<UsersList>,
    /// Роли сотрудников в чате
    #[serde(rename = "userRoleMap", skip_serializing_if = "Option::is_none")]
    pub user_role_map: Option<UserRoleMap>,
    /// Настройки ролей
    #[serde(rename = "roleConfigMap", skip_serializing_if = "Option::is_none")]
    pub role_config_map: Option<RoleConfigMap>,
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

pub type GroupChatList = Page<GroupChat>;
