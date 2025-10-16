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
    pub fn create(&self) -> BoardCreateBuilder {
        BoardCreateBuilder::new(self.client.clone())
    }

    /// Update an existing board
    pub fn update(&self, id: impl Into<String>) -> BoardUpdateBuilder {
        BoardUpdateBuilder::new(self.client.clone(), id.into())
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

pub struct BoardCreateBuilder {
    client: Arc<YouGileClient>,
    data: CreateBoard,
}

impl BoardCreateBuilder {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self {
            client,
            data: CreateBoard {
                title: "".into(),
                project_id: "".into(),
                stickers: None,
            },
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.data.title = title.into();
        self
    }

    pub fn project_id(mut self, project_id: impl Into<String>) -> Self {
        self.data.project_id = project_id.into();
        self
    }

    pub fn stickers(mut self, stickers: yougile_client::models::Stickers) -> Self {
        self.data.stickers = Some(Box::new(stickers));
        self
    }

    pub async fn execute(self) -> Result<yougile_client::models::Id, SDKError> {
        self.client
            .create_board(self.data)
            .await
            .map_err(SDKError::from)
    }
}

pub struct BoardUpdateBuilder {
    client: Arc<YouGileClient>,
    id: String,
    data: UpdateBoard,
}

impl BoardUpdateBuilder {
    pub fn new(client: Arc<YouGileClient>, id: String) -> Self {
        Self {
            client,
            id,
            data: UpdateBoard::new(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.data.title = Some(title.into());
        self
    }

    pub fn deleted(mut self, deleted: bool) -> Self {
        self.data.deleted = Some(deleted);
        self
    }

    pub fn project_id(mut self, project_id: impl Into<String>) -> Self {
        self.data.project_id = Some(project_id.into());
        self
    }

    pub fn stickers(mut self, stickers: yougile_client::models::Stickers) -> Self {
        self.data.stickers = Some(Box::new(stickers));
        self
    }

    pub async fn execute(self) -> Result<yougile_client::models::Id, SDKError> {
        self.client
            .update_board(&self.id, self.data)
            .await
            .map_err(SDKError::from)
    }
}

//TODO: надо сделать так, чтобы можно было вызывать update на Board
pub trait BoardExt {
    /// Begin updating this board via the SDK.
    fn update(&self, client: Arc<YouGileClient>) -> BoardUpdateBuilder;
}

impl BoardExt for Board {
    fn update(&self, client: Arc<YouGileClient>) -> BoardUpdateBuilder {
        BoardUpdateBuilder::new(client.clone(), self.id.clone())
    }
}
