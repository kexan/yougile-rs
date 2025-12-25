use crate::SDKError;
use std::sync::Arc;

use yougile_api_client::YouGileClient;
use yougile_api_client::models::*;

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

    /// Create a new board
    pub async fn create(&self, create_board: CreateBoard) -> Result<Id, SDKError> {
        self.client
            .create_board(create_board)
            .await
            .map_err(SDKError::from)
    }

    /// Update an existing board
    pub async fn update(&self, id: &str, update_board: UpdateBoard) -> Result<Id, SDKError> {
        self.client
            .update_board(id, update_board)
            .await
            .map_err(SDKError::from)
    }

    /// Search for boards with various filters using a fluent API
    pub fn search(&self) -> BoardSearchBuilder {
        BoardSearchBuilder::new(self.client.clone())
    }

    /// List all boards (with default parameters)
    pub async fn list(&self) -> Result<BoardList, SDKError> {
        self.search().execute().await
    }

    /// List all boards for a specific project
    pub async fn list_for_project(&self, project_id: &str) -> Result<Vec<Board>, SDKError> {
        self.search().project_id(project_id).all().await
    }
}

/// Search builder for boards with fluent API
#[derive(Clone)]
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
            limit: Some(100.0),
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

    /// Execute the search with current parameters
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

    /// Get all boards matching the search criteria with automatic pagination
    pub async fn all(self) -> Result<Vec<Board>, SDKError> {
        let mut all_boards = Vec::new();
        let mut offset = 0.0;
        let limit = self.limit.unwrap_or(100.0);

        loop {
            let result = self.clone().offset(offset).execute().await?;
            let count = result.content.len() as f64;
            all_boards.extend(result.content);

            if count < limit {
                break;
            }
            offset += limit;
        }

        Ok(all_boards)
    }
}
