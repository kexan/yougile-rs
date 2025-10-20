use crate::SDKError;
use std::sync::Arc;
use yougile_client::{
    YouGileClient,
    models::{Board, BoardList, CreateBoard, UpdateBoard},
};

/// API for working with boards
pub struct BoardsAPI {
    client: Arc<YouGileClient>,
}

impl BoardsAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    /// Get a specific board by ID
    pub async fn get(&self, id: &str) -> Result<Board, SDKError> {
        self.client.get_board(id).await.map_err(SDKError::from)
    }

    /// List all boards (with default parameters)
    pub async fn list(&self) -> Result<BoardList, SDKError> {
        self.search().execute().await
    }

    /// Create a new board
    pub async fn create(&self, create_board: CreateBoard) -> Result<Board, SDKError> {
        let created_board = self.client.create_board(create_board).await?;
        self.get(&created_board.id).await
    }

    /// Update an existing board
    pub async fn update(&self, id: &str, update_board: UpdateBoard) -> Result<Board, SDKError> {
        let updated_board = self.client.update_board(id, update_board).await?;
        self.get(&updated_board.id).await
    }

    /// Search for boards with various filters
    pub fn search(&self) -> BoardSearchBuilder {
        BoardSearchBuilder::new(self.client.clone())
    }
}

/// Search builder for boards with fluent API
pub struct BoardSearchBuilder {
    client: Arc<YouGileClient>,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<String>,
    project_id: Option<String>,
}

impl BoardSearchBuilder {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self {
            client,
            include_deleted: None,
            limit: Some(50.0), // Default limit
            offset: Some(0.0),
            title: None,
            project_id: None,
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

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn project_id(mut self, project_id: impl Into<String>) -> Self {
        self.project_id = Some(project_id.into());
        self
    }

    pub async fn execute(self) -> Result<BoardList, SDKError> {
        self.client
            .search_boards(
                self.include_deleted,
                self.limit,
                self.offset,
                self.title.as_deref(),
                self.project_id.as_deref(),
            )
            .await
            .map_err(SDKError::from)
    }
}
