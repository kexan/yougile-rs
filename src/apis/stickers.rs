use crate::{
    apis::{configuration::Configuration, parse_response, RequestBuilderExt},
    models::{
        CreateSprintSticker, CreateSprintStickerState, CreateStringSticker,
        CreateStringStickerState, SprintStickerState, SprintStickerWithStates,
        SprintStickerWithStatesList, StickerStateId, StringStickerState, StringStickerWithStates,
        StringStickerWithStatesList, UpdateSprintSticker, UpdateSprintStickerState,
        UpdateStringSticker, UpdateStringStickerState,
    },
    YougileError,
};

const SPRINT_STICKERS_PATH: &str = "/api-v2/sprint-stickers";

pub async fn create_sprint_sticker(
    configuration: &Configuration,
    create_sprint_sticker: CreateSprintSticker,
) -> Result<crate::models::Id, YougileError> {
    let url = format!("{}{}", configuration.base_path, SPRINT_STICKERS_PATH);

    let resp = configuration
        .client
        .post(&url)
        .json(&create_sprint_sticker)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn get_sprint_sticker(
    configuration: &Configuration,
    id: &str,
) -> Result<SprintStickerWithStates, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, SPRINT_STICKERS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .get(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn search_sprint_sticker(
    configuration: &Configuration,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    name: Option<&str>,
    board_id: Option<&str>,
) -> Result<SprintStickerWithStatesList, YougileError> {
    let url = format!("{}{}", configuration.base_path, SPRINT_STICKERS_PATH);

    let mut query_params = vec![];
    if let Some(val) = include_deleted {
        query_params.push(("includeDeleted", val.to_string()));
    }
    if let Some(val) = limit {
        query_params.push(("limit", val.to_string()));
    }
    if let Some(val) = offset {
        query_params.push(("offset", val.to_string()));
    }
    if let Some(val) = name {
        query_params.push(("name", val.to_string()));
    }
    if let Some(val) = board_id {
        query_params.push(("boardId", val.to_string()));
    }

    let resp = configuration
        .client
        .get(&url)
        .query(&query_params)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn update_sprint_sticker(
    configuration: &Configuration,
    id: &str,
    update_sprint_sticker: UpdateSprintSticker,
) -> Result<crate::models::Id, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, SPRINT_STICKERS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .put(&url)
        .json(&update_sprint_sticker)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

//TODO: сделать нормально
const SPRINT_STICKER_STATES_PATH: &str = "/api-v2/sprint-stickers/{}/states";

pub async fn create_sprint_sticker_state(
    configuration: &Configuration,
    sticker_id: &str,
    create_sprint_sticker_state: CreateSprintStickerState,
) -> Result<StickerStateId, YougileError> {
    let encoded_sticker_id = crate::apis::urlencode(sticker_id);
    let url = format!(
        "{}/api-v2/sprint-stickers/{}/states",
        configuration.base_path, encoded_sticker_id
    );

    let resp = configuration
        .client
        .post(&url)
        .json(&create_sprint_sticker_state)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn get_sprint_sticker_state(
    configuration: &Configuration,
    sticker_id: &str,
    sticker_state_id: &str,
    include_deleted: Option<bool>,
) -> Result<SprintStickerState, YougileError> {
    let encoded_sticker_id = crate::apis::urlencode(sticker_id);
    let encoded_sticker_state_id = crate::apis::urlencode(sticker_state_id);
    let url = format!(
        "{}/api-v2/sprint-stickers/{}/states/{}",
        configuration.base_path, encoded_sticker_id, encoded_sticker_state_id
    );

    let mut query_params = vec![];
    if let Some(val) = include_deleted {
        query_params.push(("includeDeleted", val.to_string()));
    }

    let resp = configuration
        .client
        .get(&url)
        .query(&query_params)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn update_sprint_sticker_state(
    configuration: &Configuration,
    sticker_id: &str,
    sticker_state_id: &str,
    update_sprint_sticker_state: UpdateSprintStickerState,
) -> Result<StickerStateId, YougileError> {
    let encoded_sticker_id = crate::apis::urlencode(sticker_id);
    let encoded_sticker_state_id = crate::apis::urlencode(sticker_state_id);
    let url = format!(
        "{}/api-v2/sprint-stickers/{}/states/{}",
        configuration.base_path, encoded_sticker_id, encoded_sticker_state_id
    );

    let resp = configuration
        .client
        .put(&url)
        .json(&update_sprint_sticker_state)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

const STRING_STICKERS_PATH: &str = "/api-v2/string-stickers";

pub async fn create_string_sticker(
    configuration: &Configuration,
    create_string_sticker: CreateStringSticker,
) -> Result<crate::models::Id, YougileError> {
    let url = format!("{}{}", configuration.base_path, STRING_STICKERS_PATH);

    let resp = configuration
        .client
        .post(&url)
        .json(&create_string_sticker)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn get_string_sticker(
    configuration: &Configuration,
    id: &str,
) -> Result<StringStickerWithStates, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, STRING_STICKERS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .get(&url)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn search_string_sticker(
    configuration: &Configuration,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    name: Option<&str>,
    board_id: Option<&str>,
) -> Result<StringStickerWithStatesList, YougileError> {
    let url = format!("{}{}", configuration.base_path, STRING_STICKERS_PATH);

    let mut query_params = vec![];
    if let Some(val) = include_deleted {
        query_params.push(("includeDeleted", val.to_string()));
    }
    if let Some(val) = limit {
        query_params.push(("limit", val.to_string()));
    }
    if let Some(val) = offset {
        query_params.push(("offset", val.to_string()));
    }
    if let Some(val) = name {
        query_params.push(("name", val.to_string()));
    }
    if let Some(val) = board_id {
        query_params.push(("boardId", val.to_string()));
    }

    let resp = configuration
        .client
        .get(&url)
        .query(&query_params)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn update_string_sticker(
    configuration: &Configuration,
    id: &str,
    update_string_sticker: UpdateStringSticker,
) -> Result<crate::models::Id, YougileError> {
    let encoded_id = crate::apis::urlencode(id);
    let url = format!(
        "{}{}/{}",
        configuration.base_path, STRING_STICKERS_PATH, encoded_id
    );

    let resp = configuration
        .client
        .put(&url)
        .json(&update_string_sticker)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

//TODO: сделать нормально
const STRING_STICKER_STATES_PATH: &str = "/api-v2/string-stickers/{}/states";

pub async fn create_string_sticker_state(
    configuration: &Configuration,
    sticker_id: &str,
    create_string_sticker_state: CreateStringStickerState,
) -> Result<StickerStateId, YougileError> {
    let encoded_sticker_id = crate::apis::urlencode(sticker_id);
    let url = format!(
        "{}/api-v2/string-stickers/{}/states",
        configuration.base_path, encoded_sticker_id
    );

    let resp = configuration
        .client
        .post(&url)
        .json(&create_string_sticker_state)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn get_string_sticker_state(
    configuration: &Configuration,
    sticker_id: &str,
    sticker_state_id: &str,
    include_deleted: Option<bool>,
) -> Result<StringStickerState, YougileError> {
    let encoded_sticker_id = crate::apis::urlencode(sticker_id);
    let encoded_sticker_state_id = crate::apis::urlencode(sticker_state_id);
    let url = format!(
        "{}/api-v2/string-stickers/{}/states/{}",
        configuration.base_path, encoded_sticker_id, encoded_sticker_state_id
    );

    let mut query_params = vec![];
    if let Some(val) = include_deleted {
        query_params.push(("includeDeleted", val.to_string()));
    }

    let resp = configuration
        .client
        .get(&url)
        .query(&query_params)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}

pub async fn update_string_sticker_state(
    configuration: &Configuration,
    sticker_id: &str,
    sticker_state_id: &str,
    update_string_sticker_state: UpdateStringStickerState,
) -> Result<StickerStateId, YougileError> {
    let encoded_sticker_id = crate::apis::urlencode(sticker_id);
    let encoded_sticker_state_id = crate::apis::urlencode(sticker_state_id);
    let url = format!(
        "{}/api-v2/string-stickers/{}/states/{}",
        configuration.base_path, encoded_sticker_id, encoded_sticker_state_id
    );

    let resp = configuration
        .client
        .put(&url)
        .json(&update_string_sticker_state)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}
