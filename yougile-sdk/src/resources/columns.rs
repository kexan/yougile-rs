use crate::SDKError;
use std::sync::Arc;
use yougile_client::YouGileClient;

/// API for working with columns
pub struct ColumnsAPI {
    client: Arc<YouGileClient>,
}

impl ColumnsAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    /// Get a specific column by ID
    pub async fn get(&self, id: &str) -> Result<yougile_client::models::Column, SDKError> {
        self.client.get_column(id).await.map_err(SDKError::from)
    }

    /// Create a new column
    pub async fn create(
        &self,
        create_column: yougile_client::models::CreateColumn,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client
            .create_column(create_column)
            .await
            .map_err(SDKError::from)
    }

    /// Update an existing column
    pub async fn update(
        &self,
        id: &str,
        update_column: yougile_client::models::UpdateColumn,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client
            .update_column(id, update_column)
            .await
            .map_err(SDKError::from)
    }

    /// Search for columns with various filters
    pub async fn search(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        title: Option<&str>,
        board_id: Option<&str>,
    ) -> Result<yougile_client::models::ColumnList, SDKError> {
        self.client
            .search_columns(include_deleted, limit, offset, title, board_id)
            .await
            .map_err(SDKError::from)
    }

    /// List all columns (with default parameters)
    pub async fn list(&self) -> Result<yougile_client::models::ColumnList, SDKError> {
        self.search(None, Some(100.0), Some(0.0), None, None).await
    }
}

