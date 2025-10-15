# CreateProjectDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** | Название проекта | 
**users** | Option<[**serde_json::Value**](.md)> | Сотрудники на проекте и их роль. Возможные значения: <br/><div>1) системные роли: worker, admin, observer</div><div>2) ID пользовательской роли</div><div>3) \"-\" для удаления существующего пользователя из проекта</div> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


