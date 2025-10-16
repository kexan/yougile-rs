use crate::SDKError;
use std::sync::Arc;
use yougile_client::YouGileClient;

/// API for working with tasks
pub struct TasksAPI {
    client: Arc<YouGileClient>,
}

impl TasksAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    /// Get a specific task by ID
    pub async fn get(&self, id: &str) -> Result<yougile_client::models::Task, SDKError> {
        self.client.get_task(id).await.map_err(SDKError::from)
    }

    /// Create a new task
    pub async fn create(
        &self,
        create_task: yougile_client::models::CreateTask,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client
            .create_task(create_task)
            .await
            .map_err(SDKError::from)
    }

    /// Update an existing task
    pub async fn update(
        &self,
        id: &str,
        update_task: yougile_client::models::UpdateTask,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client
            .update_task(id, update_task)
            .await
            .map_err(SDKError::from)
    }

    /// Search for tasks with various filters using a fluent API
    pub fn search(&self) -> TaskSearchBuilder {
        TaskSearchBuilder::new(self.client.clone())
    }

    /// List all tasks (with default parameters)
    pub async fn list(&self) -> Result<yougile_client::models::TaskList, SDKError> {
        self.search().execute().await
    }

    /// Get task chat subscribers
    pub async fn get_chat_subscribers(&self, id: &str) -> Result<Vec<String>, SDKError> {
        self.client
            .get_task_chat_subscribers(id)
            .await
            .map_err(SDKError::from)
    }

    /// Update task chat subscribers
    pub async fn update_chat_subscribers(
        &self,
        id: &str,
        subscribers: yougile_client::models::TaskChatSubscribers,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client
            .update_task_chat_subscribers(id, subscribers)
            .await
            .map_err(SDKError::from)
    }

    /// Search for tasks in reverse order with various filters
    #[allow(clippy::too_many_arguments)]
    pub async fn search_reversed(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        title: Option<&str>,
        column_id: Option<&str>,
        assigned_to: Option<&str>,
        sticker_id: Option<&str>,
        sticker_state_id: Option<&str>,
    ) -> Result<yougile_client::models::TaskList, SDKError> {
        self.client
            .search_tasks_reversed(
                include_deleted,
                limit,
                offset,
                title,
                column_id,
                assigned_to,
                sticker_id,
                sticker_state_id,
            )
            .await
            .map_err(SDKError::from)
    }
}

/// Search builder for tasks with fluent API
pub struct TaskSearchBuilder {
    client: Arc<YouGileClient>,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<String>,
    column_id: Option<String>,
    assigned_to: Option<String>,
    sticker_id: Option<String>,
    sticker_state_id: Option<String>,
}

impl TaskSearchBuilder {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self {
            client,
            include_deleted: None,
            limit: Some(50.0), // Default limit
            offset: Some(0.0),
            title: None,
            column_id: None,
            assigned_to: None,
            sticker_id: None,
            sticker_state_id: None,
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

    pub fn column_id(mut self, column_id: impl Into<String>) -> Self {
        self.column_id = Some(column_id.into());
        self
    }

    pub fn assigned_to(mut self, user_id: impl Into<String>) -> Self {
        self.assigned_to = Some(user_id.into());
        self
    }

    pub fn sticker_id(mut self, sticker_id: impl Into<String>) -> Self {
        self.sticker_id = Some(sticker_id.into());
        self
    }

    pub fn sticker_state_id(mut self, state_id: impl Into<String>) -> Self {
        self.sticker_state_id = Some(state_id.into());
        self
    }

    pub async fn execute(self) -> Result<yougile_client::models::TaskList, SDKError> {
        self.client
            .search_tasks(
                self.include_deleted,
                self.limit,
                self.offset,
                self.title.as_deref(),
                self.column_id.as_deref(),
                self.assigned_to.as_deref(),
                self.sticker_id.as_deref(),
                self.sticker_state_id.as_deref(),
            )
            .await
            .map_err(SDKError::from)
    }
}
