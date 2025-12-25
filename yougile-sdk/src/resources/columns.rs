use crate::SDKError;
use std::sync::Arc;

use yougile_api_client::YouGileClient;
use yougile_api_client::models::*;

/// API for working with columns
pub struct ColumnsAPI {
    client: Arc<YouGileClient>,
}

impl ColumnsAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    /// Get a specific column by ID
    pub async fn get(&self, id: &str) -> Result<Column, SDKError> {
        self.client.get_column(id).await.map_err(SDKError::from)
    }

    /// Create a new column
    pub async fn create(&self, create_column: CreateColumn) -> Result<Id, SDKError> {
        self.client
            .create_column(create_column)
            .await
            .map_err(SDKError::from)
    }

    /// Update an existing column
    pub async fn update(&self, id: &str, update_column: UpdateColumn) -> Result<Id, SDKError> {
        self.client
            .update_column(id, update_column)
            .await
            .map_err(SDKError::from)
    }

    /// Search for columns with various filters using a fluent API
    pub fn search(&self) -> ColumnSearchBuilder {
        ColumnSearchBuilder::new(self.client.clone())
    }

    /// List all columns (with default parameters)
    pub async fn list(&self) -> Result<ColumnList, SDKError> {
        self.search().execute().await
    }

    /// List all columns for a specific board
    pub async fn list_by_board(&self, board_id: &str) -> Result<Vec<Column>, SDKError> {
        self.search().board_id(board_id).all().await
    }
}

/// Search builder for columns with fluent API
#[derive(Clone)]
pub struct ColumnSearchBuilder {
    client: Arc<YouGileClient>,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<String>,
    board_id: Option<String>,
}

impl ColumnSearchBuilder {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self {
            client,
            include_deleted: None,
            limit: Some(100.0),
            offset: Some(0.0),
            title: None,
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

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn board_id(mut self, board_id: impl Into<String>) -> Self {
        self.board_id = Some(board_id.into());
        self
    }

    /// Execute the search with current parameters
    pub async fn execute(self) -> Result<ColumnList, SDKError> {
        self.client
            .search_columns(
                self.include_deleted,
                self.limit,
                self.offset,
                self.title.as_deref(),
                self.board_id.as_deref(),
            )
            .await
            .map_err(SDKError::from)
    }

    /// Get all columns matching the search criteria with automatic pagination
    pub async fn all(self) -> Result<Vec<Column>, SDKError> {
        let mut all_columns = Vec::new();
        let mut offset = 0.0;
        let limit = self.limit.unwrap_or(100.0);

        loop {
            let result = self.clone().offset(offset).execute().await?;
            let count = result.content.len() as f64;
            all_columns.extend(result.content);

            if count < limit {
                break;
            }
            offset += limit;
        }

        Ok(all_columns)
    }
}
