# UpdateTaskDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deleted** | Option<**bool**> | Если true, значит объект удален | [optional]
**title** | Option<**String**> | Название задачи | [optional]
**column_id** | Option<**String**> | Id колонки родителя. Для удаления задачи из колонки использовать \"-\" | [optional]
**description** | Option<**String**> | Описание задачи | [optional]
**archived** | Option<**bool**> | Задача перенесена в архив - да/нет | [optional]
**completed** | Option<**bool**> | Задача выполнена - да/нет | [optional]
**subtasks** | Option<**Vec<String>**> | Массив Id подзадач | [optional]
**assigned** | Option<**Vec<String>**> | Массив Id пользователей, на которых назначена задача | [optional]
**deadline** | Option<[**models::UpdateDeadline**](UpdateDeadline.md)> | Стикер \"Дэдлайн\". Указывает на крайний срок выполнения задачи. Имеется возможность кроме даты указать время, а так же дату начала задачи. | [optional]
**time_tracking** | Option<[**models::UpdateTimeTracking**](UpdateTimeTracking.md)> | Стикер \"Таймтрекинг\". Используется для указания ожидаемого и реального времени на выполнение задачи. | [optional]
**checklists** | Option<[**Vec<models::CheckList>**](CheckList.md)> | Чеклисты. К задаче всегда будет присвоен переданный объект. Если необходимо внести изменения, нужно сначала получить чеклисты, затем произвести корректировку, а затем записать в задачу заново. | [optional]
**stickers** | Option<[**serde_json::Value**](.md)> | Пользовательские стикеры. Передаются в виде объекта ключ-значение, где ключ — ID стикера, значение — ID состояния для стикеров с состоянием или строка со значением для стикеров свободных полей.<br /> Для открепления стикера от задачи, используйте \"-\" как значение состояния.<br /> Для прикрепления пустого стикера (без состояния), используйте \"empty\" как значение состояния.<br /> Для стикеров типа \"свободное текстовое поле\" передавайте произвольную строку, например \"ООО «Производство»\".<br /> Для стикеров типа \"свободное числовое поле\" передавайте строку, содержащую число, например \"123\" или \"345.123\"; разделитель целой и дробной части — точка \".\" | [optional]
**color** | Option<**String**> | Цвет карточки задач на доске, доступны цвета: task-primary, task-gray, task-red, task-pink, task-yellow, task-green, task-turquoise, task-blue, task-violet | [optional]
**id_task_common** | Option<**String**> | ID задачи, сквозной через всю компанию | [optional]
**id_task_project** | Option<**String**> | ID задачи, внутри проекта | [optional]
**timer** | Option<[**models::UpdateTimer**](UpdateTimer.md)> | Стикер \"Таймер\". Позволяет установить таймер на заданное время, а также возможность ставить его на паузу и запускать заново | [optional]
**stopwatch** | Option<[**models::UpdateStopwatch**](UpdateStopwatch.md)> | Стикер \"Секундомер\". Позволяет запустить секундомер, а так же ставить его на паузу и запускать заново. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


