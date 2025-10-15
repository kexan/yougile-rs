# CreateWebhookDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** |  | 
**event** | **String** | Событие подписки. Подписаться можно только на события в компании. Подписаться на личные чаты  не получится, потому что они не относятся к событиям в компании. Формат: `<тип_объекта>-<событие>`. Для  объектов `project,board,column,task,sticker,department,group_chat,chat_message`, возможные события: `created,deleted,restored,moved,renamed,updated`. Для объектов `user`,  возможные события: `added`, `removed`. Может использоваться javascript regexp как значение.  Например, `task-*` - подписка на все события по задачам, или `.*` - подписка на все события. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


