# WebhookDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ID объекта | 
**deleted** | Option<**bool**> | Если true, значит объект удален | [optional]
**url** | **String** |  | 
**event** | **String** | Событие подписки. Подписаться можно только на события в компании. Подписаться на личные чаты  не получится, потому что они не относятся к событиям в компании. Формат: `<тип_объекта>-<событие>`. Для  объектов `project,board,column,task,sticker,department,group_chat,chat_message`, возможные события: `created,deleted,restored,moved,renamed,updated`. Для объектов `user`,  возможные события: `added`, `removed`. Может использоваться javascript regexp как значение.  Например, `task-*` - подписка на все события по задачам, или `.*` - подписка на все события. | 
**disabled** | Option<**bool**> | Если true, то вызываться не будет | [optional][default to false]
**last_success** | Option<**f64**> | Время последнего успешного вызова | [optional]
**failures_since_last_success** | **f64** | Количество неуспешных вызовов. Сбрасывается до 0 при любом успешном вызове | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


