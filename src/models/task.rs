use crate::models::{self, deadline::UpdateDeadline};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Название задачи
    #[serde(rename = "title")]
    pub title: String,
    /// Время создания задачи
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    /// Id колонки родителя
    #[serde(rename = "columnId", skip_serializing_if = "Option::is_none")]
    pub column_id: Option<String>,
    /// Описание задачи
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Задача перенесена в архив - да/нет
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Время, когда задача перенесена в архив
    #[serde(rename = "archivedTimestamp", skip_serializing_if = "Option::is_none")]
    pub archived_timestamp: Option<f64>,
    /// Задача выполнена - да/нет
    #[serde(rename = "completed", skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    /// Время, когда задача выполнена
    #[serde(rename = "completedTimestamp", skip_serializing_if = "Option::is_none")]
    pub completed_timestamp: Option<f64>,
    /// Массив Id подзадач
    #[serde(rename = "subtasks", skip_serializing_if = "Option::is_none")]
    pub subtasks: Option<Vec<String>>,
    /// Массив Id пользователей, на которых назначена задача
    #[serde(rename = "assigned", skip_serializing_if = "Option::is_none")]
    pub assigned: Option<Vec<String>>,
    /// Id пользователя, который создал задачу
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Стикер \"Дэдлайн\". Указывает на крайний срок выполнения задачи. Имеется возможность кроме даты указать время, а так же дату начала задачи.
    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Box<models::Deadline>>,
    /// Стикер \"Таймтрекинг\". Используется для указания ожидаемого и реального времени на выполнение задачи.
    #[serde(rename = "timeTracking", skip_serializing_if = "Option::is_none")]
    pub time_tracking: Option<Box<models::TimeTracking>>,
    /// Чеклисты. К задаче всегда будет присвоен переданный объект. Если необходимо внести изменения, нужно сначала получить чеклисты, затем произвести корректировку, а затем записать в задачу заново.
    #[serde(rename = "checklists", skip_serializing_if = "Option::is_none")]
    pub checklists: Option<Vec<models::CheckList>>,
    /// Пользовательские стикеры. Передаются в виде объекта ключ-значение, где ключ — ID стикера, значение — ID состояния для стикеров с состоянием или строка со значением для стикеров свободных полей.<br /> Для открепления стикера от задачи, используйте \"-\" как значение состояния.<br /> Для прикрепления пустого стикера (без состояния), используйте \"empty\" как значение состояния.<br /> Для стикеров типа \"свободное текстовое поле\" передавайте произвольную строку, например \"ООО «Производство»\".<br /> Для стикеров типа \"свободное числовое поле\" передавайте строку, содержащую число, например \"123\" или \"345.123\"; разделитель целой и дробной части — точка \".\"
    #[serde(rename = "stickers", skip_serializing_if = "Option::is_none")]
    pub stickers: Option<serde_json::Value>,
    /// Цвет карточки задач на доске, доступны цвета: task-primary, task-gray, task-red, task-pink, task-yellow, task-green, task-turquoise, task-blue, task-violet
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// ID задачи, сквозной через всю компанию
    #[serde(rename = "idTaskCommon", skip_serializing_if = "Option::is_none")]
    pub id_task_common: Option<String>,
    /// ID задачи, внутри проекта
    #[serde(rename = "idTaskProject", skip_serializing_if = "Option::is_none")]
    pub id_task_project: Option<String>,
    #[serde(rename = "stopwatch", skip_serializing_if = "Option::is_none")]
    pub stopwatch: Option<Box<models::Stopwatch>>,
    #[serde(rename = "timer", skip_serializing_if = "Option::is_none")]
    pub timer: Option<Box<models::Timer>>,
}

impl Task {
    pub fn new(id: String, title: String, timestamp: f64) -> Task {
        Task {
            id,
            deleted: None,
            title,
            timestamp,
            column_id: None,
            description: None,
            archived: None,
            archived_timestamp: None,
            completed: None,
            completed_timestamp: None,
            subtasks: None,
            assigned: None,
            created_by: None,
            deadline: None,
            time_tracking: None,
            checklists: None,
            stickers: None,
            color: None,
            id_task_common: None,
            id_task_project: None,
            stopwatch: None,
            timer: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTask {
    /// Название задачи
    #[serde(rename = "title")]
    pub title: String,
    /// Id колонки родителя
    #[serde(rename = "columnId", skip_serializing_if = "Option::is_none")]
    pub column_id: Option<String>,
    /// Описание задачи
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Задача перенесена в архив - да/нет
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Задача выполнена - да/нет
    #[serde(rename = "completed", skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    /// Массив Id подзадач
    #[serde(rename = "subtasks", skip_serializing_if = "Option::is_none")]
    pub subtasks: Option<Vec<String>>,
    /// Массив Id пользователей, на которых назначена задача
    #[serde(rename = "assigned", skip_serializing_if = "Option::is_none")]
    pub assigned: Option<Vec<String>>,
    /// Стикер \"Дэдлайн\". Указывает на крайний срок выполнения задачи. Имеется возможность кроме даты указать время, а так же дату начала задачи.
    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Box<models::Deadline>>,
    /// Стикер \"Таймтрекинг\". Используется для указания ожидаемого и реального времени на выполнение задачи.
    #[serde(rename = "timeTracking", skip_serializing_if = "Option::is_none")]
    pub time_tracking: Option<Box<models::TimeTracking>>,
    /// Чеклисты. К задаче всегда будет присвоен переданный объект. Если необходимо внести изменения, нужно сначала получить чеклисты, затем произвести корректировку, а затем записать в задачу заново.
    #[serde(rename = "checklists", skip_serializing_if = "Option::is_none")]
    pub checklists: Option<Vec<models::CheckList>>,
    /// Пользовательские стикеры. Передаются в виде объекта ключ-значение, где ключ — ID стикера, значение — ID состояния для стикеров с состоянием или строка со значением для стикеров свободных полей.<br /> Для открепления стикера от задачи, используйте \"-\" как значение состояния.<br /> Для прикрепления пустого стикера (без состояния), используйте \"empty\" как значение состояния.<br /> Для стикеров типа \"свободное текстовое поле\" передавайте произвольную строку, например \"ООО «Производство»\".<br /> Для стикеров типа \"свободное числовое поле\" передавайте строку, содержащую число, например \"123\" или \"345.123\"; разделитель целой и дробной части — точка \".\"
    #[serde(rename = "stickers", skip_serializing_if = "Option::is_none")]
    pub stickers: Option<serde_json::Value>,
    /// Цвет карточки задач на доске, доступны цвета: task-primary, task-gray, task-red, task-pink, task-yellow, task-green, task-turquoise, task-blue, task-violet
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// ID задачи, сквозной через всю компанию
    #[serde(rename = "idTaskCommon", skip_serializing_if = "Option::is_none")]
    pub id_task_common: Option<String>,
    /// ID задачи, внутри проекта
    #[serde(rename = "idTaskProject", skip_serializing_if = "Option::is_none")]
    pub id_task_project: Option<String>,
    /// Стикер \"Секундомер\". Позволяет запустить секундомер, а так же ставить его на паузу и запускать заново.
    #[serde(rename = "stopwatch", skip_serializing_if = "Option::is_none")]
    pub stopwatch: Option<Box<models::CreateStopwatch>>,
    /// Стикер \"Таймер\". Позволяет установить таймер на заданное время, а также возможность ставить его на паузу и запускать заново
    #[serde(rename = "timer", skip_serializing_if = "Option::is_none")]
    pub timer: Option<Box<models::CreateTimer>>,
}

impl CreateTask {
    pub fn new(title: String) -> CreateTask {
        CreateTask {
            title,
            column_id: None,
            description: None,
            archived: None,
            completed: None,
            subtasks: None,
            assigned: None,
            deadline: None,
            time_tracking: None,
            checklists: None,
            stickers: None,
            color: None,
            id_task_common: None,
            id_task_project: None,
            stopwatch: None,
            timer: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateTask {
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Название задачи
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Id колонки родителя. Для удаления задачи из колонки использовать \"-\"
    #[serde(rename = "columnId", skip_serializing_if = "Option::is_none")]
    pub column_id: Option<String>,
    /// Описание задачи
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Задача перенесена в архив - да/нет
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Задача выполнена - да/нет
    #[serde(rename = "completed", skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    /// Массив Id подзадач
    #[serde(rename = "subtasks", skip_serializing_if = "Option::is_none")]
    pub subtasks: Option<Vec<String>>,
    /// Массив Id пользователей, на которых назначена задача
    #[serde(rename = "assigned", skip_serializing_if = "Option::is_none")]
    pub assigned: Option<Vec<String>>,
    /// Стикер \"Дэдлайн\". Указывает на крайний срок выполнения задачи. Имеется возможность кроме даты указать время, а так же дату начала задачи.
    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Box<UpdateDeadline>>,
    /// Стикер \"Таймтрекинг\". Используется для указания ожидаемого и реального времени на выполнение задачи.
    #[serde(rename = "timeTracking", skip_serializing_if = "Option::is_none")]
    pub time_tracking: Option<Box<models::UpdateTimeTracking>>,
    /// Чеклисты. К задаче всегда будет присвоен переданный объект. Если необходимо внести изменения, нужно сначала получить чеклисты, затем произвести корректировку, а затем записать в задачу заново.
    #[serde(rename = "checklists", skip_serializing_if = "Option::is_none")]
    pub checklists: Option<Vec<models::CheckList>>,
    /// Пользовательские стикеры. Передаются в виде объекта ключ-значение, где ключ — ID стикера, значение — ID состояния для стикеров с состоянием или строка со значением для стикеров свободных полей.<br /> Для открепления стикера от задачи, используйте \"-\" как значение состояния.<br /> Для прикрепления пустого стикера (без состояния), используйте \"empty\" как значение состояния.<br /> Для стикеров типа \"свободное текстовое поле\" передавайте произвольную строку, например \"ООО «Производство»\".<br /> Для стикеров типа \"свободное числовое поле\" передавайте строку, содержащую число, например \"123\" или \"345.123\"; разделитель целой и дробной части — точка \".\"
    #[serde(rename = "stickers", skip_serializing_if = "Option::is_none")]
    pub stickers: Option<serde_json::Value>,
    /// Цвет карточки задач на доске, доступны цвета: task-primary, task-gray, task-red, task-pink, task-yellow, task-green, task-turquoise, task-blue, task-violet
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// ID задачи, сквозной через всю компанию
    #[serde(rename = "idTaskCommon", skip_serializing_if = "Option::is_none")]
    pub id_task_common: Option<String>,
    /// ID задачи, внутри проекта
    #[serde(rename = "idTaskProject", skip_serializing_if = "Option::is_none")]
    pub id_task_project: Option<String>,
    /// Стикер \"Таймер\". Позволяет установить таймер на заданное время, а также возможность ставить его на паузу и запускать заново
    #[serde(rename = "timer", skip_serializing_if = "Option::is_none")]
    pub timer: Option<Box<models::UpdateTimer>>,
    /// Стикер \"Секундомер\". Позволяет запустить секундомер, а так же ставить его на паузу и запускать заново.
    #[serde(rename = "stopwatch", skip_serializing_if = "Option::is_none")]
    pub stopwatch: Option<Box<models::UpdateStopwatch>>,
}

impl UpdateTask {
    pub fn new() -> UpdateTask {
        UpdateTask {
            deleted: None,
            title: None,
            column_id: None,
            description: None,
            archived: None,
            completed: None,
            subtasks: None,
            assigned: None,
            deadline: None,
            time_tracking: None,
            checklists: None,
            stickers: None,
            color: None,
            id_task_common: None,
            id_task_project: None,
            timer: None,
            stopwatch: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskChatSubscribers {
    /// Подписчики чата задачи
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<String>>,
}

impl TaskChatSubscribers {
    pub fn new() -> TaskChatSubscribers {
        TaskChatSubscribers { content: None }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskList {
    /// Дополнительная информация о странице
    #[serde(rename = "paging")]
    pub paging: Box<models::PagingMetadata>,
    /// Список задач
    #[serde(rename = "content")]
    pub content: Vec<TaskListBase>,
}

impl TaskList {
    pub fn new(paging: models::PagingMetadata, content: Vec<TaskListBase>) -> TaskList {
        TaskList {
            paging: Box::new(paging),
            content,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskListBase {
    /// ID объекта
    #[serde(rename = "id")]
    pub id: String,
    /// Если true, значит объект удален
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Название задачи
    #[serde(rename = "title")]
    pub title: String,
    /// Время создания задачи
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    /// Id колонки родителя
    #[serde(rename = "columnId", skip_serializing_if = "Option::is_none")]
    pub column_id: Option<String>,
    /// Описание задачи
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Задача перенесена в архив - да/нет
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Время, когда задача перенесена в архив
    #[serde(rename = "archivedTimestamp", skip_serializing_if = "Option::is_none")]
    pub archived_timestamp: Option<f64>,
    /// Задача выполнена - да/нет
    #[serde(rename = "completed", skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    /// Время, когда задача выполнена
    #[serde(rename = "completedTimestamp", skip_serializing_if = "Option::is_none")]
    pub completed_timestamp: Option<f64>,
    /// Массив Id подзадач
    #[serde(rename = "subtasks", skip_serializing_if = "Option::is_none")]
    pub subtasks: Option<Vec<String>>,
    /// Массив Id пользователей, на которых назначена задача
    #[serde(rename = "assigned", skip_serializing_if = "Option::is_none")]
    pub assigned: Option<Vec<String>>,
    /// Id пользователя, который создал задачу
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Стикер \"Дэдлайн\". Указывает на крайний срок выполнения задачи. Имеется возможность кроме даты указать время, а так же дату начала задачи.
    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Box<models::Deadline>>,
    /// Стикер \"Таймтрекинг\". Используется для указания ожидаемого и реального времени на выполнение задачи.
    #[serde(rename = "timeTracking", skip_serializing_if = "Option::is_none")]
    pub time_tracking: Option<Box<models::TimeTracking>>,
    /// Чеклисты. К задаче всегда будет присвоен переданный объект. Если необходимо внести изменения, нужно сначала получить чеклисты, затем произвести корректировку, а затем записать в задачу заново.
    #[serde(rename = "checklists", skip_serializing_if = "Option::is_none")]
    pub checklists: Option<Vec<models::CheckList>>,
    /// Пользовательские стикеры. Передаются в виде объекта ключ-значение, где ключ — ID стикера, значение — ID состояния для стикеров с состоянием или строка со значением для стикеров свободных полей.<br /> Для открепления стикера от задачи, используйте \"-\" как значение состояния.<br /> Для прикрепления пустого стикера (без состояния), используйте \"empty\" как значение состояния.<br /> Для стикеров типа \"свободное текстовое поле\" передавайте произвольную строку, например \"ООО «Производство»\".<br /> Для стикеров типа \"свободное числовое поле\" передавайте строку, содержащую число, например \"123\" или \"345.123\"; разделитель целой и дробной части — точка \".\"
    #[serde(rename = "stickers", skip_serializing_if = "Option::is_none")]
    pub stickers: Option<serde_json::Value>,
    /// Цвет карточки задач на доске, доступны цвета: task-primary, task-gray, task-red, task-pink, task-yellow, task-green, task-turquoise, task-blue, task-violet
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// ID задачи, сквозной через всю компанию
    #[serde(rename = "idTaskCommon", skip_serializing_if = "Option::is_none")]
    pub id_task_common: Option<String>,
    /// ID задачи, внутри проекта
    #[serde(rename = "idTaskProject", skip_serializing_if = "Option::is_none")]
    pub id_task_project: Option<String>,
    #[serde(rename = "stopwatch", skip_serializing_if = "Option::is_none")]
    pub stopwatch: Option<Box<models::Stopwatch>>,
    #[serde(rename = "timer", skip_serializing_if = "Option::is_none")]
    pub timer: Option<Box<models::Timer>>,
}

impl TaskListBase {
    pub fn new(id: String, title: String, timestamp: f64) -> TaskListBase {
        TaskListBase {
            id,
            deleted: None,
            title,
            timestamp,
            column_id: None,
            description: None,
            archived: None,
            archived_timestamp: None,
            completed: None,
            completed_timestamp: None,
            subtasks: None,
            assigned: None,
            created_by: None,
            deadline: None,
            time_tracking: None,
            checklists: None,
            stickers: None,
            color: None,
            id_task_common: None,
            id_task_project: None,
            stopwatch: None,
            timer: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskPermissions {
    #[serde(rename = "show")]
    pub show: bool,
    #[serde(rename = "delete")]
    pub delete: bool,
    #[serde(rename = "editTitle")]
    pub edit_title: bool,
    #[serde(rename = "editDescription")]
    pub edit_description: bool,
    #[serde(rename = "complete")]
    pub complete: bool,
    #[serde(rename = "close")]
    pub close: bool,
    #[serde(rename = "assignUsers")]
    pub assign_users: AssignUsers,
    #[serde(rename = "connect")]
    pub connect: bool,
    #[serde(rename = "editSubtasks")]
    pub edit_subtasks: EditSubtasks,
    #[serde(rename = "editStickers")]
    pub edit_stickers: bool,
    #[serde(rename = "editPins")]
    pub edit_pins: bool,
    #[serde(rename = "move")]
    pub r#move: Move,
    #[serde(rename = "sendMessages")]
    pub send_messages: bool,
    #[serde(rename = "sendFiles")]
    pub send_files: bool,
    #[serde(rename = "editWhoToNotify")]
    pub edit_who_to_notify: EditWhoToNotify,
}

impl TaskPermissions {
    pub fn new(
        show: bool,
        delete: bool,
        edit_title: bool,
        edit_description: bool,
        complete: bool,
        close: bool,
        assign_users: AssignUsers,
        connect: bool,
        edit_subtasks: EditSubtasks,
        edit_stickers: bool,
        edit_pins: bool,
        r#move: Move,
        send_messages: bool,
        send_files: bool,
        edit_who_to_notify: EditWhoToNotify,
    ) -> TaskPermissions {
        TaskPermissions {
            show,
            delete,
            edit_title,
            edit_description,
            complete,
            close,
            assign_users,
            connect,
            edit_subtasks,
            edit_stickers,
            edit_pins,
            r#move,
            send_messages,
            send_files,
            edit_who_to_notify,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssignUsers {
    #[serde(rename = "no")]
    No,
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "add-self")]
    AddSelf,
    #[serde(rename = "set-self")]
    SetSelf,
    #[serde(rename = "change-from-self")]
    ChangeFromSelf,
}

impl Default for AssignUsers {
    fn default() -> AssignUsers {
        Self::No
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EditSubtasks {
    #[serde(rename = "no")]
    No,
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "complete")]
    Complete,
}

impl Default for EditSubtasks {
    fn default() -> EditSubtasks {
        Self::No
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Move {
    #[serde(rename = "no")]
    No,
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "board")]
    Board,
}

impl Default for Move {
    fn default() -> Move {
        Self::No
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EditWhoToNotify {
    #[serde(rename = "no")]
    No,
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "self")]
    VariantSelf,
}

impl Default for EditWhoToNotify {
    fn default() -> EditWhoToNotify {
        Self::No
    }
}
