use crate::SDKError;
use std::sync::Arc;
use yougile_client::{
    YouGileClient,
    models::{
        CreateSprintSticker, CreateStringSticker, Id, SprintSticker, SprintStickerList,
        SprintStickerState, StringSticker, StringStickerList, StringStickerState,
        UpdateSprintSticker, UpdateStringSticker,
    },
    SprintStateData, SprintStateUpdate, StringStateData, StringStateUpdate,
};

/// API for working with stickers (both sprint and string types)
pub struct StickersAPI {
    client: Arc<YouGileClient>,
}

impl StickersAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    // Sprint Sticker Methods

    /// Get a specific sprint sticker by ID
    pub async fn get_sprint(&self, id: &str) -> Result<SprintSticker, SDKError> {
        self.client
            .get_sprint_sticker(id)
            .await
            .map_err(SDKError::from)
    }

    /// Create a new sprint sticker
    pub async fn create_sprint(
        &self,
        create_sprint_sticker: CreateSprintSticker,
    ) -> Result<Id, SDKError> {
        self.client
            .create_sprint_sticker(create_sprint_sticker)
            .await
            .map_err(SDKError::from)
    }

    /// Update an existing sprint sticker
    pub async fn update_sprint(
        &self,
        id: &str,
        update_sprint_sticker: UpdateSprintSticker,
    ) -> Result<Id, SDKError> {
        self.client
            .update_sprint_sticker(id, update_sprint_sticker)
            .await
            .map_err(SDKError::from)
    }

    /// Search for sprint stickers
    pub fn search_sprint(&self) -> SprintStickerSearchBuilder {
        SprintStickerSearchBuilder::new(self.client.clone())
    }

    /// List all sprint stickers for a specific board
    pub async fn list_sprint_by_board(
        &self,
        board_id: &str,
    ) -> Result<Vec<SprintSticker>, SDKError> {
        self.search_sprint().board_id(board_id).all().await
    }

    // Sprint Sticker State Methods

    /// Create a sprint sticker state
    pub async fn create_sprint_state(
        &self,
        sticker_id: &str,
        data: SprintStateData,
    ) -> Result<Id, SDKError> {
        self.client
            .create_sprint_sticker_state(sticker_id, data)
            .await
            .map_err(SDKError::from)
    }

    /// Get a sprint sticker state
    pub async fn get_sprint_state(
        &self,
        sticker_id: &str,
        state_id: &str,
        include_deleted: Option<bool>,
    ) -> Result<SprintStickerState, SDKError> {
        self.client
            .get_sprint_sticker_state(sticker_id, state_id, include_deleted)
            .await
            .map_err(SDKError::from)
    }

    /// Update a sprint sticker state
    pub async fn update_sprint_state(
        &self,
        sticker_id: &str,
        state_id: &str,
        update: SprintStateUpdate,
    ) -> Result<Id, SDKError> {
        self.client
            .update_sprint_sticker_state(sticker_id, state_id, update)
            .await
            .map_err(SDKError::from)
    }

    // String Sticker Methods

    /// Get a specific string sticker by ID
    pub async fn get_string(&self, id: &str) -> Result<StringSticker, SDKError> {
        self.client
            .get_string_sticker(id)
            .await
            .map_err(SDKError::from)
    }

    /// Create a new string sticker
    pub async fn create_string(
        &self,
        create_string_sticker: CreateStringSticker,
    ) -> Result<Id, SDKError> {
        self.client
            .create_string_sticker(create_string_sticker)
            .await
            .map_err(SDKError::from)
    }

    /// Update an existing string sticker
    pub async fn update_string(
        &self,
        id: &str,
        update_string_sticker: UpdateStringSticker,
    ) -> Result<Id, SDKError> {
        self.client
            .update_string_sticker(id, update_string_sticker)
            .await
            .map_err(SDKError::from)
    }

    /// Search for string stickers
    pub fn search_string(&self) -> StringStickerSearchBuilder {
        StringStickerSearchBuilder::new(self.client.clone())
    }

    /// List all string stickers for a specific board
    pub async fn list_string_by_board(
        &self,
        board_id: &str,
    ) -> Result<Vec<StringSticker>, SDKError> {
        self.search_string().board_id(board_id).all().await
    }

    // String Sticker State Methods

    /// Create a string sticker state
    pub async fn create_string_state(
        &self,
        sticker_id: &str,
        data: StringStateData,
    ) -> Result<Id, SDKError> {
        self.client
            .create_string_sticker_state(sticker_id, data)
            .await
            .map_err(SDKError::from)
    }

    /// Get a string sticker state
    pub async fn get_string_state(
        &self,
        sticker_id: &str,
        state_id: &str,
        include_deleted: Option<bool>,
    ) -> Result<StringStickerState, SDKError> {
        self.client
            .get_string_sticker_state(sticker_id, state_id, include_deleted)
            .await
            .map_err(SDKError::from)
    }

    /// Update a string sticker state
    pub async fn update_string_state(
        &self,
        sticker_id: &str,
        state_id: &str,
        update: StringStateUpdate,
    ) -> Result<Id, SDKError> {
        self.client
            .update_string_sticker_state(sticker_id, state_id, update)
            .await
            .map_err(SDKError::from)
    }
}

/// Search builder for sprint stickers
#[derive(Clone)]
pub struct SprintStickerSearchBuilder {
    client: Arc<YouGileClient>,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    name: Option<String>,
    board_id: Option<String>,
}

impl SprintStickerSearchBuilder {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self {
            client,
            include_deleted: None,
            limit: Some(100.0),
            offset: Some(0.0),
            name: None,
            board_id: None,
        }
    }

    pub fn include_deleted(mut self, include: bool) -> Self {
        self.include_deleted = Some(include);
        self
    }

    pub fn limit(mut self, limit: f64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: f64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn board_id(mut self, board_id: impl Into<String>) -> Self {
        self.board_id = Some(board_id.into());
        self
    }

    /// Execute the search with current parameters
    pub async fn execute(self) -> Result<SprintStickerList, SDKError> {
        self.client
            .search_sprint_stickers(
                self.include_deleted,
                self.limit,
                self.offset,
                self.name.as_deref(),
                self.board_id.as_deref(),
            )
            .await
            .map_err(SDKError::from)
    }

    /// Get all sprint stickers matching the search criteria with automatic pagination
    pub async fn all(self) -> Result<Vec<SprintSticker>, SDKError> {
        let mut all_stickers = Vec::new();
        let mut offset = 0.0;
        let limit = self.limit.unwrap_or(100.0);

        loop {
            let result = self.clone().offset(offset).execute().await?;
            let count = result.content.len() as f64;
            all_stickers.extend(result.content);

            if count < limit {
                break;
            }
            offset += limit;
        }

        Ok(all_stickers)
    }
}

/// Search builder for string stickers
#[derive(Clone)]
pub struct StringStickerSearchBuilder {
    client: Arc<YouGileClient>,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    name: Option<String>,
    board_id: Option<String>,
}

impl StringStickerSearchBuilder {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self {
            client,
            include_deleted: None,
            limit: Some(100.0),
            offset: Some(0.0),
            name: None,
            board_id: None,
        }
    }

    pub fn include_deleted(mut self, include: bool) -> Self {
        self.include_deleted = Some(include);
        self
    }

    pub fn limit(mut self, limit: f64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: f64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn board_id(mut self, board_id: impl Into<String>) -> Self {
        self.board_id = Some(board_id.into());
        self
    }

    /// Execute the search with current parameters
    pub async fn execute(self) -> Result<StringStickerList, SDKError> {
        self.client
            .search_string_stickers(
                self.include_deleted,
                self.limit,
                self.offset,
                self.name.as_deref(),
                self.board_id.as_deref(),
            )
            .await
            .map_err(SDKError::from)
    }

    /// Get all string stickers matching the search criteria with automatic pagination
    pub async fn all(self) -> Result<Vec<StringSticker>, SDKError> {
        let mut all_stickers = Vec::new();
        let mut offset = 0.0;
        let limit = self.limit.unwrap_or(100.0);

        loop {
            let result = self.clone().offset(offset).execute().await?;
            let count = result.content.len() as f64;
            all_stickers.extend(result.content);

            if count < limit {
                break;
            }
            offset += limit;
        }

        Ok(all_stickers)
    }
}
