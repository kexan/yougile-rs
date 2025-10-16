use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Почтовый ящик сотрудника
    #[serde(rename = "email")]
    pub email: String,
    /// Имеет ли пользователь права администратора
    #[serde(rename = "isAdmin", skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    /// ФИО
    #[serde(rename = "realName")]
    pub real_name: String,
    /// Статус online/offline
    #[serde(rename = "status")]
    pub status: String,
    /// Время последнего действия в системе
    #[serde(rename = "lastActivity")]
    pub last_activity: f64,
}

impl User {
    pub fn new(
        id: String,
        email: String,
        real_name: String,
        status: String,
        last_activity: f64,
    ) -> User {
        User {
            id,
            email,
            is_admin: None,
            real_name,
            status,
            last_activity,
        }
    }
}
