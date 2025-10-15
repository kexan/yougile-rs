# ProjectDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deleted** | Option<**bool**> | Если true, значит объект удален | [optional]
**id** | **String** | ID объекта | 
**title** | **String** | Название проекта | 
**timestamp** | **f64** | Время создания проекта | 
**users** | Option<[**serde_json::Value**](.md)> | Сотрудники на проекте и их роль. Возможные значения: <br/><div>1) системные роли: worker, admin, observer</div><div>2) ID пользовательской роли</div><div>3) \"-\" для удаления существующего пользователя из проекта</div> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


