# UpdateDepartmentDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deleted** | Option<**bool**> | Если true, значит объект удален | [optional]
**title** | Option<**String**> | Название отдела | [optional]
**parent_id** | Option<**String**> | Id родительского отдела. Оставить пустым или \"-\", если это отдел верхнего уровня | [optional]
**users** | Option<[**serde_json::Value**](.md)> | Сотрудники на отделе и их роль. Возможные значения: <br/><div>1) manager или member</div><div>2) \"-\" или \"\" для удаления существующего пользователя из отдела</div> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


