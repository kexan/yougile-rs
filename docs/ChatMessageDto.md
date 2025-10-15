# ChatMessageDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deleted** | Option<**bool**> | Если true, значит объект удален | [optional]
**id** | **f64** | ID сообщения, также является временем создания | 
**from_user_id** | **String** | ID автора сообщения | 
**text** | **String** | Текст сообщения | 
**text_html** | **String** | Текст сообщения в формате HTML | 
**label** | **String** | Быстрая ссылка | 
**edit_timestamp** | **f64** | Время последнего редактирования | 
**reactions** | [**serde_json::Value**](.md) | Реакции на сообщение | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


