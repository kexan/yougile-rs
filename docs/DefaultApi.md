# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_key_controller_create**](DefaultApi.md#auth_key_controller_create) | **POST** /api-v2/auth/keys | Создать ключ
[**auth_key_controller_delete**](DefaultApi.md#auth_key_controller_delete) | **DELETE** /api-v2/auth/keys/{key} | Удалить ключ
[**auth_key_controller_search**](DefaultApi.md#auth_key_controller_search) | **POST** /api-v2/auth/keys/get | Получить список ключей
[**board_controller_create**](DefaultApi.md#board_controller_create) | **POST** /api-v2/boards | Создать
[**board_controller_get**](DefaultApi.md#board_controller_get) | **GET** /api-v2/boards/{id} | Получить по ID
[**board_controller_search**](DefaultApi.md#board_controller_search) | **GET** /api-v2/boards | Получить список
[**board_controller_update**](DefaultApi.md#board_controller_update) | **PUT** /api-v2/boards/{id} | Изменить
[**chat_message_controller_get**](DefaultApi.md#chat_message_controller_get) | **GET** /api-v2/chats/{chatId}/messages/{id} | Получить сообщение по ID
[**chat_message_controller_search**](DefaultApi.md#chat_message_controller_search) | **GET** /api-v2/chats/{chatId}/messages | Получить историю сообщений
[**chat_message_controller_send_message**](DefaultApi.md#chat_message_controller_send_message) | **POST** /api-v2/chats/{chatId}/messages | Написать в чат
[**chat_message_controller_update**](DefaultApi.md#chat_message_controller_update) | **PUT** /api-v2/chats/{chatId}/messages/{id} | Изменить сообщение
[**column_controller_create**](DefaultApi.md#column_controller_create) | **POST** /api-v2/columns | Создать
[**column_controller_get**](DefaultApi.md#column_controller_get) | **GET** /api-v2/columns/{id} | Получить по ID
[**column_controller_search**](DefaultApi.md#column_controller_search) | **GET** /api-v2/columns | Получить список
[**column_controller_update**](DefaultApi.md#column_controller_update) | **PUT** /api-v2/columns/{id} | Изменить
[**company_controller_get**](DefaultApi.md#company_controller_get) | **GET** /api-v2/companies* | Получить детали
[**company_controller_update**](DefaultApi.md#company_controller_update) | **PUT** /api-v2/companies* | Изменить
[**department_controller_create**](DefaultApi.md#department_controller_create) | **POST** /api-v2/departments | Создать
[**department_controller_get**](DefaultApi.md#department_controller_get) | **GET** /api-v2/departments/{id} | Получить по ID
[**department_controller_search**](DefaultApi.md#department_controller_search) | **GET** /api-v2/departments | Получить список
[**department_controller_update**](DefaultApi.md#department_controller_update) | **PUT** /api-v2/departments/{id} | Изменить
[**file_controller_upload_file**](DefaultApi.md#file_controller_upload_file) | **POST** /api-v2/upload-file | Загрузить
[**get_companies**](DefaultApi.md#get_companies) | **POST** /api-v2/auth/companies | Получить список компаний
[**group_chat_controller_create**](DefaultApi.md#group_chat_controller_create) | **POST** /api-v2/group-chats | Создать чат
[**group_chat_controller_get**](DefaultApi.md#group_chat_controller_get) | **GET** /api-v2/group-chats/{id} | Получить чат по ID
[**group_chat_controller_search**](DefaultApi.md#group_chat_controller_search) | **GET** /api-v2/group-chats | Получить список чатов
[**group_chat_controller_update**](DefaultApi.md#group_chat_controller_update) | **PUT** /api-v2/group-chats/{id} | Изменить чат
[**project_controller_create**](DefaultApi.md#project_controller_create) | **POST** /api-v2/projects | Создать
[**project_controller_get**](DefaultApi.md#project_controller_get) | **GET** /api-v2/projects/{id} | Получить по ID
[**project_controller_search**](DefaultApi.md#project_controller_search) | **GET** /api-v2/projects | Получить список
[**project_controller_update**](DefaultApi.md#project_controller_update) | **PUT** /api-v2/projects/{id} | Изменить
[**project_roles_controller_create**](DefaultApi.md#project_roles_controller_create) | **POST** /api-v2/projects/{projectId}/roles | Создать
[**project_roles_controller_delete**](DefaultApi.md#project_roles_controller_delete) | **DELETE** /api-v2/projects/{projectId}/roles/{id} | Удалить
[**project_roles_controller_get**](DefaultApi.md#project_roles_controller_get) | **GET** /api-v2/projects/{projectId}/roles/{id} | Получить по ID
[**project_roles_controller_search**](DefaultApi.md#project_roles_controller_search) | **GET** /api-v2/projects/{projectId}/roles | Получить список
[**project_roles_controller_update**](DefaultApi.md#project_roles_controller_update) | **PUT** /api-v2/projects/{projectId}/roles/{id} | Изменить
[**sprint_sticker_controller_create**](DefaultApi.md#sprint_sticker_controller_create) | **POST** /api-v2/sprint-stickers | Создать
[**sprint_sticker_controller_get_sticker**](DefaultApi.md#sprint_sticker_controller_get_sticker) | **GET** /api-v2/sprint-stickers/{id} | Получить по ID
[**sprint_sticker_controller_search**](DefaultApi.md#sprint_sticker_controller_search) | **GET** /api-v2/sprint-stickers | Получить список
[**sprint_sticker_controller_update**](DefaultApi.md#sprint_sticker_controller_update) | **PUT** /api-v2/sprint-stickers/{id} | Изменить
[**sprint_sticker_state_controller_create**](DefaultApi.md#sprint_sticker_state_controller_create) | **POST** /api-v2/sprint-stickers/{stickerId}/states | Создать
[**sprint_sticker_state_controller_get**](DefaultApi.md#sprint_sticker_state_controller_get) | **GET** /api-v2/sprint-stickers/{stickerId}/states/{stickerStateId} | Получить по ID
[**sprint_sticker_state_controller_update**](DefaultApi.md#sprint_sticker_state_controller_update) | **PUT** /api-v2/sprint-stickers/{stickerId}/states/{stickerStateId} | Изменить
[**string_sticker_controller_create**](DefaultApi.md#string_sticker_controller_create) | **POST** /api-v2/string-stickers | Создать
[**string_sticker_controller_get**](DefaultApi.md#string_sticker_controller_get) | **GET** /api-v2/string-stickers/{id} | Получить по ID
[**string_sticker_controller_search**](DefaultApi.md#string_sticker_controller_search) | **GET** /api-v2/string-stickers | Получить список
[**string_sticker_controller_update**](DefaultApi.md#string_sticker_controller_update) | **PUT** /api-v2/string-stickers/{id} | Изменить
[**string_sticker_state_controller_create**](DefaultApi.md#string_sticker_state_controller_create) | **POST** /api-v2/string-stickers/{stickerId}/states | Создать
[**string_sticker_state_controller_get**](DefaultApi.md#string_sticker_state_controller_get) | **GET** /api-v2/string-stickers/{stickerId}/states/{stickerStateId} | Получить по ID
[**string_sticker_state_controller_update**](DefaultApi.md#string_sticker_state_controller_update) | **PUT** /api-v2/string-stickers/{stickerId}/states/{stickerStateId} | Изменить
[**task_controller_create**](DefaultApi.md#task_controller_create) | **POST** /api-v2/tasks | Создать
[**task_controller_get**](DefaultApi.md#task_controller_get) | **GET** /api-v2/tasks/{id} | Получить по ID
[**task_controller_get_chat_subscribers**](DefaultApi.md#task_controller_get_chat_subscribers) | **GET** /api-v2/tasks/{id}/chat-subscribers | Получить список участников чата задачи
[**task_controller_search**](DefaultApi.md#task_controller_search) | **GET** /api-v2/task-list | Получить список задач
[**task_controller_search_reversed**](DefaultApi.md#task_controller_search_reversed) | **GET** /api-v2/tasks | Получить список задач в обратном порядке
[**task_controller_update**](DefaultApi.md#task_controller_update) | **PUT** /api-v2/tasks/{id} | Изменить
[**task_controller_update_chat_subscribers**](DefaultApi.md#task_controller_update_chat_subscribers) | **PUT** /api-v2/tasks/{id}/chat-subscribers | Изменить список участников чата задачи
[**user_controller_create**](DefaultApi.md#user_controller_create) | **POST** /api-v2/users | Пригласить в компанию
[**user_controller_delete**](DefaultApi.md#user_controller_delete) | **DELETE** /api-v2/users/{id} | Удалить из компании
[**user_controller_get**](DefaultApi.md#user_controller_get) | **GET** /api-v2/users/{id} | Получить по ID
[**user_controller_search**](DefaultApi.md#user_controller_search) | **GET** /api-v2/users | Получить список
[**user_controller_update**](DefaultApi.md#user_controller_update) | **PUT** /api-v2/users/{id} | Изменить
[**webhook_controller_create**](DefaultApi.md#webhook_controller_create) | **POST** /api-v2/webhooks | Создать подписку
[**webhook_controller_put**](DefaultApi.md#webhook_controller_put) | **PUT** /api-v2/webhooks/{id} | Изменить подписку
[**webhook_controller_search**](DefaultApi.md#webhook_controller_search) | **GET** /api-v2/webhooks | Получить список подписок



## auth_key_controller_create

> models::AuthKeyDto auth_key_controller_create(credentials_with_company_dto)
Создать ключ

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credentials_with_company_dto** | [**CredentialsWithCompanyDto**](CredentialsWithCompanyDto.md) |  | [required] |

### Return type

[**models::AuthKeyDto**](AuthKeyDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_key_controller_delete

> auth_key_controller_delete(key)
Удалить ключ

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_key_controller_search

> Vec<models::AuthKeyWithDetailsDto> auth_key_controller_search(credentials_with_company_optional_dto)
Получить список ключей

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credentials_with_company_optional_dto** | [**CredentialsWithCompanyOptionalDto**](CredentialsWithCompanyOptionalDto.md) |  | [required] |

### Return type

[**Vec<models::AuthKeyWithDetailsDto>**](AuthKeyWithDetailsDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## board_controller_create

> models::WithIdDto board_controller_create(create_board_dto)
Создать

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_board_dto** | [**CreateBoardDto**](CreateBoardDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## board_controller_get

> models::BoardDto board_controller_get(id)
Получить по ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::BoardDto**](BoardDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## board_controller_search

> models::BoardListDto board_controller_search(include_deleted, limit, offset, title, project_id)
Получить список

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_deleted** | Option<**bool**> | По умолчанию, если объект был отмечен как удаленный, то он не будет найден.    Поставьте true, чтобы удаленные объекты возвращались. |  |
**limit** | Option<**f64**> | Количество элементов, которые хочется получить. Максимум 1000. |  |[default to 50]
**offset** | Option<**f64**> | Индекс первого элемента страницы |  |[default to 0]
**title** | Option<**String**> | Имя доски |  |
**project_id** | Option<**String**> | ID проекта |  |

### Return type

[**models::BoardListDto**](BoardListDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## board_controller_update

> models::WithIdDto board_controller_update(id, update_board_dto)
Изменить

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_board_dto** | [**UpdateBoardDto**](UpdateBoardDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_message_controller_get

> models::ChatMessageDto chat_message_controller_get(chat_id, id)
Получить сообщение по ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_id** | **String** | Идентификатор чата | [required] |
**id** | **f64** | Идентификатор сообщения | [required] |

### Return type

[**models::ChatMessageDto**](ChatMessageDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_message_controller_search

> models::ChatMessageListDto chat_message_controller_search(chat_id, include_deleted, limit, offset, from_user_id, text, label, since, include_system)
Получить историю сообщений

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_id** | **String** | Идентификатор чата | [required] |
**include_deleted** | Option<**bool**> | По умолчанию, если объект был отмечен как удаленный, то он не будет найден.    Поставьте true, чтобы удаленные объекты возвращались. |  |
**limit** | Option<**f64**> | Количество элементов, которые хочется получить. Максимум 1000. |  |[default to 50]
**offset** | Option<**f64**> | Индекс первого элемента страницы |  |[default to 0]
**from_user_id** | Option<**String**> | ID сотрудника от кого сообщение |  |
**text** | Option<**String**> | Строка, которую сообщение должно содержать |  |
**label** | Option<**String**> | Поиск по быстрой ссылке сообщения |  |
**since** | Option<**f64**> | Искать среди сообщений, время создания которых позже указанного времени (timestamp) |  |
**include_system** | Option<**bool**> | Включать ли системные сообщения. По умолчанию они не включаются. |  |

### Return type

[**models::ChatMessageListDto**](ChatMessageListDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_message_controller_send_message

> models::ChatIdDto chat_message_controller_send_message(chat_id, create_chat_message_dto)
Написать в чат

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_id** | **String** | Идентификатор чата | [required] |
**create_chat_message_dto** | [**CreateChatMessageDto**](CreateChatMessageDto.md) |  | [required] |

### Return type

[**models::ChatIdDto**](ChatIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_message_controller_update

> models::ChatIdDto chat_message_controller_update(chat_id, id, update_chat_message_dto)
Изменить сообщение

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_id** | **String** | Идентификатор чата | [required] |
**id** | **f64** | Идентификатор сообщения | [required] |
**update_chat_message_dto** | [**UpdateChatMessageDto**](UpdateChatMessageDto.md) |  | [required] |

### Return type

[**models::ChatIdDto**](ChatIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## column_controller_create

> models::WithIdDto column_controller_create(create_column_dto)
Создать

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_column_dto** | [**CreateColumnDto**](CreateColumnDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## column_controller_get

> models::ColumnDto column_controller_get(id)
Получить по ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ColumnDto**](ColumnDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## column_controller_search

> models::ColumnListDto column_controller_search(include_deleted, limit, offset, title, board_id)
Получить список

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_deleted** | Option<**bool**> | По умолчанию, если объект был отмечен как удаленный, то он не будет найден.    Поставьте true, чтобы удаленные объекты возвращались. |  |
**limit** | Option<**f64**> | Количество элементов, которые хочется получить. Максимум 1000. |  |[default to 50]
**offset** | Option<**f64**> | Индекс первого элемента страницы |  |[default to 0]
**title** | Option<**String**> | Имя колонки |  |
**board_id** | Option<**String**> | ID доски |  |

### Return type

[**models::ColumnListDto**](ColumnListDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## column_controller_update

> models::WithIdDto column_controller_update(id, update_column_dto)
Изменить

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_column_dto** | [**UpdateColumnDto**](UpdateColumnDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_controller_get

> models::CompanyDto company_controller_get()
Получить детали

Получить детали текущей компании

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CompanyDto**](CompanyDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_controller_update

> models::WithIdDto company_controller_update(update_company_dto)
Изменить

Изменить детали текущей компании

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_company_dto** | [**UpdateCompanyDto**](UpdateCompanyDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## department_controller_create

> models::WithIdDto department_controller_create(create_department_dto)
Создать

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_department_dto** | [**CreateDepartmentDto**](CreateDepartmentDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## department_controller_get

> models::DepartmentDto department_controller_get(id)
Получить по ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DepartmentDto**](DepartmentDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## department_controller_search

> models::DepartmentListDto department_controller_search(include_deleted, limit, offset, title, parent_id)
Получить список

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_deleted** | Option<**bool**> | По умолчанию, если объект был отмечен как удаленный, то он не будет найден.    Поставьте true, чтобы удаленные объекты возвращались. |  |
**limit** | Option<**f64**> | Количество элементов, которые хочется получить. Максимум 1000. |  |[default to 50]
**offset** | Option<**f64**> | Индекс первого элемента страницы |  |[default to 0]
**title** | Option<**String**> | Имя отдела |  |
**parent_id** | Option<**String**> | ID родительского отдела |  |

### Return type

[**models::DepartmentListDto**](DepartmentListDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## department_controller_update

> models::WithIdDto department_controller_update(id, update_department_dto)
Изменить

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_department_dto** | [**UpdateDepartmentDto**](UpdateDepartmentDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_controller_upload_file

> models::FileUploadDto file_controller_upload_file()
Загрузить

Загружает файл на сервер и возвращает его URL. Если вы хотите использовать curl из командной строки, не указывайте явно boundary в Content-Type — curl сам выставит нужный заголовок.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::FileUploadDto**](FileUploadDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_companies

> models::CompanyListDto get_companies(credentials_with_name_dto, limit, offset)
Получить список компаний

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credentials_with_name_dto** | [**CredentialsWithNameDto**](CredentialsWithNameDto.md) |  | [required] |
**limit** | Option<**f64**> | Количество элементов, которые хочется получить. Максимум 1000. |  |[default to 50]
**offset** | Option<**f64**> | Индекс первого элемента страницы |  |[default to 0]

### Return type

[**models::CompanyListDto**](CompanyListDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_chat_controller_create

> models::WithIdDto group_chat_controller_create(create_group_chat_dto)
Создать чат

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_group_chat_dto** | [**CreateGroupChatDto**](CreateGroupChatDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_chat_controller_get

> models::GroupChatDto group_chat_controller_get(id)
Получить чат по ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Идентификатор чата | [required] |

### Return type

[**models::GroupChatDto**](GroupChatDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_chat_controller_search

> models::GroupChatListDto group_chat_controller_search(include_deleted, limit, offset, title)
Получить список чатов

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_deleted** | Option<**bool**> | По умолчанию, если объект был отмечен как удаленный, то он не будет найден.    Поставьте true, чтобы удаленные объекты возвращались. |  |
**limit** | Option<**f64**> | Количество элементов, которые хочется получить. Максимум 1000. |  |[default to 50]
**offset** | Option<**f64**> | Индекс первого элемента страницы |  |[default to 0]
**title** | Option<**String**> | Имя чата |  |

### Return type

[**models::GroupChatListDto**](GroupChatListDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_chat_controller_update

> models::WithIdDto group_chat_controller_update(id, update_group_chat_dto)
Изменить чат

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Идентификатор чата | [required] |
**update_group_chat_dto** | [**UpdateGroupChatDto**](UpdateGroupChatDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_controller_create

> models::WithIdDto project_controller_create(create_project_dto)
Создать

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_project_dto** | [**CreateProjectDto**](CreateProjectDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_controller_get

> models::ProjectDto project_controller_get(id)
Получить по ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ProjectDto**](ProjectDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_controller_search

> models::ProjectListDto project_controller_search(include_deleted, limit, offset, title)
Получить список

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_deleted** | Option<**bool**> | По умолчанию, если объект был отмечен как удаленный, то он не будет найден.    Поставьте true, чтобы удаленные объекты возвращались. |  |
**limit** | Option<**f64**> | Количество элементов, которые хочется получить. Максимум 1000. |  |[default to 50]
**offset** | Option<**f64**> | Индекс первого элемента страницы |  |[default to 0]
**title** | Option<**String**> | Имя проекта |  |

### Return type

[**models::ProjectListDto**](ProjectListDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_controller_update

> models::WithIdDto project_controller_update(id, update_project_dto)
Изменить

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_project_dto** | [**UpdateProjectDto**](UpdateProjectDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_controller_create

> models::WithIdDto project_roles_controller_create(project_id, create_project_role_dto)
Создать

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**create_project_role_dto** | [**CreateProjectRoleDto**](CreateProjectRoleDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_controller_delete

> models::ProjectRoleDto project_roles_controller_delete(project_id, id)
Удалить

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::ProjectRoleDto**](ProjectRoleDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_controller_get

> models::ProjectRoleDto project_roles_controller_get(project_id, id)
Получить по ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::ProjectRoleDto**](ProjectRoleDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_controller_search

> models::ProjectRoleListDto project_roles_controller_search(project_id, limit, offset, name)
Получить список

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**limit** | Option<**f64**> | Количество элементов, которые хочется получить. Максимум 1000. |  |[default to 50]
**offset** | Option<**f64**> | Индекс первого элемента страницы |  |[default to 0]
**name** | Option<**String**> | Имя роли |  |

### Return type

[**models::ProjectRoleListDto**](ProjectRoleListDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_controller_update

> models::WithIdDto project_roles_controller_update(project_id, id, update_project_role_dto)
Изменить

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_project_role_dto** | [**UpdateProjectRoleDto**](UpdateProjectRoleDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sprint_sticker_controller_create

> models::WithIdDto sprint_sticker_controller_create(create_sprint_sticker_dto)
Создать

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_sprint_sticker_dto** | [**CreateSprintStickerDto**](CreateSprintStickerDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sprint_sticker_controller_get_sticker

> models::SprintStickerWithStatesDto sprint_sticker_controller_get_sticker(id)
Получить по ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::SprintStickerWithStatesDto**](SprintStickerWithStatesDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sprint_sticker_controller_search

> models::SprintStickerWithStatesListDto sprint_sticker_controller_search(include_deleted, limit, offset, name, board_id)
Получить список

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_deleted** | Option<**bool**> | По умолчанию, если объект был отмечен как удаленный, то он не будет найден.    Поставьте true, чтобы удаленные объекты возвращались. |  |
**limit** | Option<**f64**> | Количество элементов, которые хочется получить. Максимум 1000. |  |[default to 50]
**offset** | Option<**f64**> | Индекс первого элемента страницы |  |[default to 0]
**name** | Option<**String**> | Имя стикера |  |
**board_id** | Option<**String**> | ID доски |  |

### Return type

[**models::SprintStickerWithStatesListDto**](SprintStickerWithStatesListDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sprint_sticker_controller_update

> models::WithIdDto sprint_sticker_controller_update(id, update_sprint_sticker_dto)
Изменить

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_sprint_sticker_dto** | [**UpdateSprintStickerDto**](UpdateSprintStickerDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sprint_sticker_state_controller_create

> models::WithStickerStateIdDto sprint_sticker_state_controller_create(sticker_id, create_sprint_sticker_state_dto)
Создать

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sticker_id** | **String** |  | [required] |
**create_sprint_sticker_state_dto** | [**CreateSprintStickerStateDto**](CreateSprintStickerStateDto.md) |  | [required] |

### Return type

[**models::WithStickerStateIdDto**](WithStickerStateIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sprint_sticker_state_controller_get

> models::SprintStickerStateDto sprint_sticker_state_controller_get(sticker_id, sticker_state_id, include_deleted)
Получить по ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sticker_id** | **String** |  | [required] |
**sticker_state_id** | **String** |  | [required] |
**include_deleted** | Option<**bool**> |  |  |

### Return type

[**models::SprintStickerStateDto**](SprintStickerStateDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sprint_sticker_state_controller_update

> models::WithStickerStateIdDto sprint_sticker_state_controller_update(sticker_id, sticker_state_id, update_sprint_sticker_state_dto)
Изменить

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sticker_id** | **String** |  | [required] |
**sticker_state_id** | **String** |  | [required] |
**update_sprint_sticker_state_dto** | [**UpdateSprintStickerStateDto**](UpdateSprintStickerStateDto.md) |  | [required] |

### Return type

[**models::WithStickerStateIdDto**](WithStickerStateIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## string_sticker_controller_create

> models::WithIdDto string_sticker_controller_create(create_string_sticker_dto)
Создать

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_string_sticker_dto** | [**CreateStringStickerDto**](CreateStringStickerDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## string_sticker_controller_get

> models::StringStickerWithStatesDto string_sticker_controller_get(id)
Получить по ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::StringStickerWithStatesDto**](StringStickerWithStatesDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## string_sticker_controller_search

> models::StringStickerWithStatesListDto string_sticker_controller_search(include_deleted, limit, offset, name, board_id)
Получить список

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_deleted** | Option<**bool**> | По умолчанию, если объект был отмечен как удаленный, то он не будет найден.    Поставьте true, чтобы удаленные объекты возвращались. |  |
**limit** | Option<**f64**> | Количество элементов, которые хочется получить. Максимум 1000. |  |[default to 50]
**offset** | Option<**f64**> | Индекс первого элемента страницы |  |[default to 0]
**name** | Option<**String**> | Имя стикера |  |
**board_id** | Option<**String**> | ID доски |  |

### Return type

[**models::StringStickerWithStatesListDto**](StringStickerWithStatesListDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## string_sticker_controller_update

> models::WithIdDto string_sticker_controller_update(id, update_string_sticker_dto)
Изменить

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_string_sticker_dto** | [**UpdateStringStickerDto**](UpdateStringStickerDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## string_sticker_state_controller_create

> models::WithStickerStateIdDto string_sticker_state_controller_create(sticker_id, create_string_sticker_state_dto)
Создать

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sticker_id** | **String** |  | [required] |
**create_string_sticker_state_dto** | [**CreateStringStickerStateDto**](CreateStringStickerStateDto.md) |  | [required] |

### Return type

[**models::WithStickerStateIdDto**](WithStickerStateIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## string_sticker_state_controller_get

> models::StringStickerStateDto string_sticker_state_controller_get(sticker_id, sticker_state_id, include_deleted)
Получить по ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sticker_id** | **String** |  | [required] |
**sticker_state_id** | **String** |  | [required] |
**include_deleted** | Option<**bool**> |  |  |

### Return type

[**models::StringStickerStateDto**](StringStickerStateDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## string_sticker_state_controller_update

> models::WithStickerStateIdDto string_sticker_state_controller_update(sticker_id, sticker_state_id, update_string_sticker_state_dto)
Изменить

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sticker_id** | **String** |  | [required] |
**sticker_state_id** | **String** |  | [required] |
**update_string_sticker_state_dto** | [**UpdateStringStickerStateDto**](UpdateStringStickerStateDto.md) |  | [required] |

### Return type

[**models::WithStickerStateIdDto**](WithStickerStateIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## task_controller_create

> models::WithIdDto task_controller_create(create_task_dto)
Создать

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_task_dto** | [**CreateTaskDto**](CreateTaskDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## task_controller_get

> models::TaskDto task_controller_get(id)
Получить по ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Идентификатор задачи | [required] |

### Return type

[**models::TaskDto**](TaskDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## task_controller_get_chat_subscribers

> Vec<String> task_controller_get_chat_subscribers(id)
Получить список участников чата задачи

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**Vec<String>**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## task_controller_search

> models::TaskListDto task_controller_search(include_deleted, limit, offset, title, column_id, assigned_to, sticker_id, sticker_state_id)
Получить список задач

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_deleted** | Option<**bool**> | По умолчанию, если объект был отмечен как удаленный, то он не будет найден.    Поставьте true, чтобы удаленные объекты возвращались. |  |
**limit** | Option<**f64**> | Количество элементов, которые хочется получить. Максимум 1000. |  |[default to 50]
**offset** | Option<**f64**> | Индекс первого элемента страницы |  |[default to 0]
**title** | Option<**String**> | Заголовок задачи |  |
**column_id** | Option<**String**> | ID колонки |  |
**assigned_to** | Option<**String**> | ID исполнителей через запятую |  |
**sticker_id** | Option<**String**> | ID стикера для фильтрации |  |
**sticker_state_id** | Option<**String**> | ID состояния стикера для фильтрации |  |

### Return type

[**models::TaskListDto**](TaskListDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## task_controller_search_reversed

> models::TaskListDto task_controller_search_reversed(include_deleted, limit, offset, title, column_id, assigned_to, sticker_id, sticker_state_id)
Получить список задач в обратном порядке

Используйте /task-list вместо этого

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_deleted** | Option<**bool**> | По умолчанию, если объект был отмечен как удаленный, то он не будет найден.    Поставьте true, чтобы удаленные объекты возвращались. |  |
**limit** | Option<**f64**> | Количество элементов, которые хочется получить. Максимум 1000. |  |[default to 50]
**offset** | Option<**f64**> | Индекс первого элемента страницы |  |[default to 0]
**title** | Option<**String**> | Заголовок задачи |  |
**column_id** | Option<**String**> | ID колонки |  |
**assigned_to** | Option<**String**> | ID исполнителей через запятую |  |
**sticker_id** | Option<**String**> | ID стикера для фильтрации |  |
**sticker_state_id** | Option<**String**> | ID состояния стикера для фильтрации |  |

### Return type

[**models::TaskListDto**](TaskListDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## task_controller_update

> models::WithIdDto task_controller_update(id, update_task_dto)
Изменить

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Идентификатор задачи | [required] |
**update_task_dto** | [**UpdateTaskDto**](UpdateTaskDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## task_controller_update_chat_subscribers

> models::WithIdDto task_controller_update_chat_subscribers(id, task_chat_subscribers_dto)
Изменить список участников чата задачи

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**task_chat_subscribers_dto** | [**TaskChatSubscribersDto**](TaskChatSubscribersDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_create

> models::WithIdDto user_controller_create(create_user_dto)
Пригласить в компанию

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_dto** | [**CreateUserDto**](CreateUserDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_delete

> models::WithIdDto user_controller_delete(id)
Удалить из компании

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_get

> models::UserDto user_controller_get(id)
Получить по ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::UserDto**](UserDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_search

> models::UserListDto user_controller_search(limit, offset, email, project_id)
Получить список

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**f64**> | Количество элементов, которые хочется получить. Максимум 1000. |  |[default to 50]
**offset** | Option<**f64**> | Индекс первого элемента страницы |  |[default to 0]
**email** | Option<**String**> | Почта сотрудника |  |
**project_id** | Option<**String**> | ID проекта |  |

### Return type

[**models::UserListDto**](UserListDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_controller_update

> models::WithIdDto user_controller_update(id, update_user_dto)
Изменить

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_user_dto** | [**UpdateUserDto**](UpdateUserDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_controller_create

> models::WithIdDto webhook_controller_create(create_webhook_dto)
Создать подписку

Создает подписку на события в компании

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_webhook_dto** | [**CreateWebhookDto**](CreateWebhookDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_controller_put

> models::WithIdDto webhook_controller_put(id, update_webhook_dto)
Изменить подписку

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_webhook_dto** | [**UpdateWebhookDto**](UpdateWebhookDto.md) |  | [required] |

### Return type

[**models::WithIdDto**](WithIdDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_controller_search

> models::WebhookDto webhook_controller_search(include_deleted)
Получить список подписок

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_deleted** | Option<**bool**> | По умолчанию, если объект был отмечен как удаленный, то он не будет найден.    Поставьте true, чтобы удаленные объекты возвращались. |  |

### Return type

[**models::WebhookDto**](WebhookDto.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

