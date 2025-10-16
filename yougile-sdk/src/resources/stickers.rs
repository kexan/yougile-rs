use crate::SDKError;
use std::sync::Arc;
use yougile_client::YouGileClient;

/// API for working with stickers
pub struct StickersAPI {
    client: Arc<YouGileClient>,
}

impl StickersAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    /// Create a sprint sticker
    pub async fn create_sprint_sticker(
        &self,
        create_sprint_sticker: yougile_client::models::CreateSprintSticker,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client
            .create_sprint_sticker(create_sprint_sticker)
            .await
            .map_err(SDKError::from)
    }

    /// Get a sprint sticker by ID
    pub async fn get_sprint_sticker(
        &self,
        id: &str,
    ) -> Result<yougile_client::models::SprintStickerWithStates, SDKError> {
        self.client
            .get_sprint_sticker(id)
            .await
            .map_err(SDKError::from)
    }

    /// Search for sprint stickers with various filters
    pub async fn search_sprint_stickers(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        name: Option<&str>,
        board_id: Option<&str>,
    ) -> Result<yougile_client::models::SprintStickerWithStatesList, SDKError> {
        self.client
            .search_sprint_stickers(include_deleted, limit, offset, name, board_id)
            .await
            .map_err(SDKError::from)
    }

    /// Update a sprint sticker
    pub async fn update_sprint_sticker(
        &self,
        id: &str,
        update_sprint_sticker: yougile_client::models::UpdateSprintSticker,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client
            .update_sprint_sticker(id, update_sprint_sticker)
            .await
            .map_err(SDKError::from)
    }

    /// Create a sprint sticker state
    pub async fn create_sprint_sticker_state(
        &self,
        sticker_id: &str,
        create_sprint_sticker_state: yougile_client::models::CreateSprintStickerState,
    ) -> Result<yougile_client::models::StickerStateId, SDKError> {
        self.client
            .create_sprint_sticker_state(sticker_id, create_sprint_sticker_state)
            .await
            .map_err(SDKError::from)
    }

    /// Get a sprint sticker state
    pub async fn get_sprint_sticker_state(
        &self,
        sticker_id: &str,
        sticker_state_id: &str,
        include_deleted: Option<bool>,
    ) -> Result<yougile_client::models::SprintStickerState, SDKError> {
        self.client
            .get_sprint_sticker_state(sticker_id, sticker_state_id, include_deleted)
            .await
            .map_err(SDKError::from)
    }

    /// Update a sprint sticker state
    pub async fn update_sprint_sticker_state(
        &self,
        sticker_id: &str,
        sticker_state_id: &str,
        update_sprint_sticker_state: yougile_client::models::UpdateSprintStickerState,
    ) -> Result<yougile_client::models::StickerStateId, SDKError> {
        self.client
            .update_sprint_sticker_state(sticker_id, sticker_state_id, update_sprint_sticker_state)
            .await
            .map_err(SDKError::from)
    }

    /// Create a string sticker
    pub async fn create_string_sticker(
        &self,
        create_string_sticker: yougile_client::models::CreateStringSticker,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client
            .create_string_sticker(create_string_sticker)
            .await
            .map_err(SDKError::from)
    }

    /// Get a string sticker by ID
    pub async fn get_string_sticker(
        &self,
        id: &str,
    ) -> Result<yougile_client::models::StringStickerWithStates, SDKError> {
        self.client
            .get_string_sticker(id)
            .await
            .map_err(SDKError::from)
    }

    /// Search for string stickers with various filters
    pub async fn search_string_stickers(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        name: Option<&str>,
        board_id: Option<&str>,
    ) -> Result<yougile_client::models::StringStickerWithStatesList, SDKError> {
        self.client
            .search_string_stickers(include_deleted, limit, offset, name, board_id)
            .await
            .map_err(SDKError::from)
    }

    /// Update a string sticker
    pub async fn update_string_sticker(
        &self,
        id: &str,
        update_string_sticker: yougile_client::models::UpdateStringSticker,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client
            .update_string_sticker(id, update_string_sticker)
            .await
            .map_err(SDKError::from)
    }

    /// Create a string sticker state
    pub async fn create_string_sticker_state(
        &self,
        sticker_id: &str,
        create_string_sticker_state: yougile_client::models::CreateStringStickerState,
    ) -> Result<yougile_client::models::StickerStateId, SDKError> {
        self.client
            .create_string_sticker_state(sticker_id, create_string_sticker_state)
            .await
            .map_err(SDKError::from)
    }

    /// Get a string sticker state
    pub async fn get_string_sticker_state(
        &self,
        sticker_id: &str,
        sticker_state_id: &str,
        include_deleted: Option<bool>,
    ) -> Result<yougile_client::models::StringStickerState, SDKError> {
        self.client
            .get_string_sticker_state(sticker_id, sticker_state_id, include_deleted)
            .await
            .map_err(SDKError::from)
    }

    /// Update a string sticker state
    pub async fn update_string_sticker_state(
        &self,
        sticker_id: &str,
        sticker_state_id: &str,
        update_string_sticker_state: yougile_client::models::UpdateStringStickerState,
    ) -> Result<yougile_client::models::StickerStateId, SDKError> {
        self.client
            .update_string_sticker_state(sticker_id, sticker_state_id, update_string_sticker_state)
            .await
            .map_err(SDKError::from)
    }
}

