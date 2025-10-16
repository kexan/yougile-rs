use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

pub async fn sprint_sticker_controller_create(
    configuration: &configuration::Configuration,
    create_sprint_sticker_dto: models::CreateSprintSticker,
) -> Result<models::Id, Error<SprintStickerControllerCreateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_sprint_sticker_dto = create_sprint_sticker_dto;

    let uri_str = format!("{}/api-v2/sprint-stickers", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_sprint_sticker_dto);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::WithIdDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::WithIdDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SprintStickerControllerCreateError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn sprint_sticker_controller_get_sticker(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::SprintStickerWithStates, Error<SprintStickerControllerGetStickerError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!(
        "{}/api-v2/sprint-stickers/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(p_path_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SprintStickerWithStatesDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SprintStickerWithStatesDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SprintStickerControllerGetStickerError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn sprint_sticker_controller_search(
    configuration: &configuration::Configuration,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    name: Option<&str>,
    board_id: Option<&str>,
) -> Result<models::SprintStickerWithStatesList, Error<SprintStickerControllerSearchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_include_deleted = include_deleted;
    let p_query_limit = limit;
    let p_query_offset = offset;
    let p_query_name = name;
    let p_query_board_id = board_id;

    let uri_str = format!("{}/api-v2/sprint-stickers", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_include_deleted {
        req_builder = req_builder.query(&[("includeDeleted", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_board_id {
        req_builder = req_builder.query(&[("boardId", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SprintStickerWithStatesListDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SprintStickerWithStatesListDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SprintStickerControllerSearchError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn sprint_sticker_controller_update(
    configuration: &configuration::Configuration,
    id: &str,
    update_sprint_sticker_dto: models::UpdateSprintSticker,
) -> Result<models::Id, Error<SprintStickerControllerUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_update_sprint_sticker_dto = update_sprint_sticker_dto;

    let uri_str = format!(
        "{}/api-v2/sprint-stickers/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(p_path_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_sprint_sticker_dto);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::WithIdDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::WithIdDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SprintStickerControllerUpdateError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn sprint_sticker_state_controller_create(
    configuration: &configuration::Configuration,
    sticker_id: &str,
    create_sprint_sticker_state_dto: models::CreateSprintStickerState,
) -> Result<models::StickerStateId, Error<SprintStickerStateControllerCreateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_sticker_id = sticker_id;
    let p_body_create_sprint_sticker_state_dto = create_sprint_sticker_state_dto;

    let uri_str = format!(
        "{}/api-v2/sprint-stickers/{stickerId}/states",
        configuration.base_path,
        stickerId = crate::apis::urlencode(p_path_sticker_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_sprint_sticker_state_dto);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::WithStickerStateIdDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::WithStickerStateIdDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SprintStickerStateControllerCreateError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn sprint_sticker_state_controller_get(
    configuration: &configuration::Configuration,
    sticker_id: &str,
    sticker_state_id: &str,
    include_deleted: Option<bool>,
) -> Result<models::SprintStickerState, Error<SprintStickerStateControllerGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_sticker_id = sticker_id;
    let p_path_sticker_state_id = sticker_state_id;
    let p_query_include_deleted = include_deleted;

    let uri_str = format!(
        "{}/api-v2/sprint-stickers/{stickerId}/states/{stickerStateId}",
        configuration.base_path,
        stickerId = crate::apis::urlencode(p_path_sticker_id),
        stickerStateId = crate::apis::urlencode(p_path_sticker_state_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_include_deleted {
        req_builder = req_builder.query(&[("includeDeleted", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SprintStickerStateDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SprintStickerStateDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SprintStickerStateControllerGetError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn sprint_sticker_state_controller_update(
    configuration: &configuration::Configuration,
    sticker_id: &str,
    sticker_state_id: &str,
    update_sprint_sticker_state_dto: models::UpdateSprintStickerState,
) -> Result<models::StickerStateId, Error<SprintStickerStateControllerUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_sticker_id = sticker_id;
    let p_path_sticker_state_id = sticker_state_id;
    let p_body_update_sprint_sticker_state_dto = update_sprint_sticker_state_dto;

    let uri_str = format!(
        "{}/api-v2/sprint-stickers/{stickerId}/states/{stickerStateId}",
        configuration.base_path,
        stickerId = crate::apis::urlencode(p_path_sticker_id),
        stickerStateId = crate::apis::urlencode(p_path_sticker_state_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_sprint_sticker_state_dto);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::WithStickerStateIdDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::WithStickerStateIdDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SprintStickerStateControllerUpdateError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn string_sticker_controller_create(
    configuration: &configuration::Configuration,
    create_string_sticker_dto: models::CreateStringSticker,
) -> Result<models::Id, Error<StringStickerControllerCreateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_string_sticker_dto = create_string_sticker_dto;

    let uri_str = format!("{}/api-v2/string-stickers", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_string_sticker_dto);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::WithIdDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::WithIdDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StringStickerControllerCreateError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn string_sticker_controller_get(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::StringStickerWithStates, Error<StringStickerControllerGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!(
        "{}/api-v2/string-stickers/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(p_path_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StringStickerWithStatesDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StringStickerWithStatesDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StringStickerControllerGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn string_sticker_controller_search(
    configuration: &configuration::Configuration,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    name: Option<&str>,
    board_id: Option<&str>,
) -> Result<models::StringStickerWithStatesList, Error<StringStickerControllerSearchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_include_deleted = include_deleted;
    let p_query_limit = limit;
    let p_query_offset = offset;
    let p_query_name = name;
    let p_query_board_id = board_id;

    let uri_str = format!("{}/api-v2/string-stickers", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_include_deleted {
        req_builder = req_builder.query(&[("includeDeleted", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_board_id {
        req_builder = req_builder.query(&[("boardId", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StringStickerWithStatesListDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StringStickerWithStatesListDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StringStickerControllerSearchError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn string_sticker_controller_update(
    configuration: &configuration::Configuration,
    id: &str,
    update_string_sticker_dto: models::UpdateStringSticker,
) -> Result<models::Id, Error<StringStickerControllerUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_update_string_sticker_dto = update_string_sticker_dto;

    let uri_str = format!(
        "{}/api-v2/string-stickers/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(p_path_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_string_sticker_dto);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::WithIdDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::WithIdDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StringStickerControllerUpdateError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn string_sticker_state_controller_create(
    configuration: &configuration::Configuration,
    sticker_id: &str,
    create_string_sticker_state_dto: models::CreateStringStickerState,
) -> Result<models::StickerStateId, Error<StringStickerStateControllerCreateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_sticker_id = sticker_id;
    let p_body_create_string_sticker_state_dto = create_string_sticker_state_dto;

    let uri_str = format!(
        "{}/api-v2/string-stickers/{stickerId}/states",
        configuration.base_path,
        stickerId = crate::apis::urlencode(p_path_sticker_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_string_sticker_state_dto);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::WithStickerStateIdDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::WithStickerStateIdDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StringStickerStateControllerCreateError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn string_sticker_state_controller_get(
    configuration: &configuration::Configuration,
    sticker_id: &str,
    sticker_state_id: &str,
    include_deleted: Option<bool>,
) -> Result<models::StringStickerState, Error<StringStickerStateControllerGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_sticker_id = sticker_id;
    let p_path_sticker_state_id = sticker_state_id;
    let p_query_include_deleted = include_deleted;

    let uri_str = format!(
        "{}/api-v2/string-stickers/{stickerId}/states/{stickerStateId}",
        configuration.base_path,
        stickerId = crate::apis::urlencode(p_path_sticker_id),
        stickerStateId = crate::apis::urlencode(p_path_sticker_state_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_include_deleted {
        req_builder = req_builder.query(&[("includeDeleted", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StringStickerStateDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StringStickerStateDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StringStickerStateControllerGetError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn string_sticker_state_controller_update(
    configuration: &configuration::Configuration,
    sticker_id: &str,
    sticker_state_id: &str,
    update_string_sticker_state_dto: models::UpdateStringStickerState,
) -> Result<models::StickerStateId, Error<StringStickerStateControllerUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_sticker_id = sticker_id;
    let p_path_sticker_state_id = sticker_state_id;
    let p_body_update_string_sticker_state_dto = update_string_sticker_state_dto;

    let uri_str = format!(
        "{}/api-v2/string-stickers/{stickerId}/states/{stickerStateId}",
        configuration.base_path,
        stickerId = crate::apis::urlencode(p_path_sticker_id),
        stickerStateId = crate::apis::urlencode(p_path_sticker_state_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_string_sticker_state_dto);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::WithStickerStateIdDto`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::WithStickerStateIdDto`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StringStickerStateControllerUpdateError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
