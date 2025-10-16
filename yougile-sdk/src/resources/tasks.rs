use yougile_client::YouGileClient;
use crate::{SDKError, convenience::TaskSearchBuilder};

/// API for working with tasks
pub struct TasksAPI<'a> {
    client: &'a YouGileClient,
}

impl<'a> TasksAPI<'a> {
    pub fn new(client: &'a YouGileClient) -> Self {
        Self { client }
    }

    /// Get a specific task by ID
    pub async fn get(&self, id: &str) -> Result<yougile_client::models::Task, SDKError> {
        self.client.get_task(id).await.map_err(SDKError::from)
    }

    /// Create a new task
    pub async fn create(
        &self, 
        create_task: yougile_client::models::CreateTask
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client.create_task(create_task).await.map_err(SDKError::from)
    }

    /// Update an existing task
    pub async fn update(
        &self, 
        id: &str, 
        update_task: yougile_client::models::UpdateTask
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client.update_task(id, update_task).await.map_err(SDKError::from)
    }

    /// Search for tasks with various filters using a fluent API
    pub fn search(&self) -> TaskSearchBuilder<'_> {
        TaskSearchBuilder::new(self.client)
    }

    /// List all tasks (with default parameters)
    pub async fn list(&self) -> Result<yougile_client::models::TaskList, SDKError> {
        self.search().execute().await
    }

    /// Get task chat subscribers
    pub async fn get_chat_subscribers(&self, id: &str) -> Result<Vec<String>, SDKError> {
        self.client.get_task_chat_subscribers(id).await.map_err(SDKError::from)
    }

    /// Update task chat subscribers
    pub async fn update_chat_subscribers(
        &self,
        id: &str,
        subscribers: yougile_client::models::TaskChatSubscribers,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client.update_task_chat_subscribers(id, subscribers).await.map_err(SDKError::from)
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