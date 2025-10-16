use yougile_client::YouGileClient;
use crate::{SDKError, convenience::BoardSearchBuilder};

/// API for working with boards
pub struct BoardsAPI<'a> {
    client: &'a YouGileClient,
}

impl<'a> BoardsAPI<'a> {
    pub fn new(client: &'a YouGileClient) -> Self {
        Self { client }
    }

    /// Get a specific board by ID
    pub async fn get(&self, id: &str) -> Result<yougile_client::models::Board, SDKError> {
        self.client.get_board(id).await.map_err(SDKError::from)
    }

    /// Create a new board
    pub async fn create(
        &self, 
        create_board: yougile_client::models::CreateBoard
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client.create_board(create_board).await.map_err(SDKError::from)
    }

    /// Update an existing board
    pub async fn update(
        &self, 
        id: &str, 
        update_board: yougile_client::models::UpdateBoard
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client.update_board(id, update_board).await.map_err(SDKError::from)
    }

    /// Search for boards with various filters using a fluent API
    pub fn search(&self) -> BoardSearchBuilder<'_> {
        BoardSearchBuilder::new(self.client)
    }

    /// List all boards (with default parameters)
    pub async fn list(&self) -> Result<yougile_client::models::BoardList, SDKError> {
        self.search().execute().await
    }
}