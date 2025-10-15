# UpdateDeadline

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deadline** | Option<**f64**> | Timestamp дэдлайна | [optional]
**start_date** | Option<**f64**> | Timestamp начала задачи | [optional]
**with_time** | Option<**bool**> | Отображать на стикере время, или только дату | [optional]
**history** | Option<**Vec<String>**> | История изменений дедлайна | [optional]
**blocked_points** | **Vec<String>** | Точки, которые блокируют дату дедлайна (Начало или Конец) | 
**links** | **Vec<String>** | Связанные задачи | 
**deleted** | Option<**bool**> | Открепить стикер от задачи (true) | [optional]
**empty** | Option<**bool**> | Прикрепить стикер дедлайна без значения | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


