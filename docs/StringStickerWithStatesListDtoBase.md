# StringStickerWithStatesListDtoBase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ID объекта | 
**deleted** | Option<**bool**> | Если true, значит объект удален | [optional]
**name** | **String** | Имя стикера | 
**icon** | Option<**String**> | Иконка стикера | [optional]
**states** | Option<[**Vec<models::StringStickerStateDto>**](StringStickerStateDto.md)> | Состояния стикера. | [optional]
**limit** | Option<**f64**> | Количество элементов, которые хочется получить. Максимум 1000. | [optional][default to 50]
**offset** | Option<**f64**> | Индекс первого элемента страницы | [optional][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


