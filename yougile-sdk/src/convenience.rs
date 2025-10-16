use crate::SDKError;
use std::sync::Arc;
use yougile_client::YouGileClient;

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
    pub fn new(client: &Arc<YouGileClient>) -> Self {
        Self {
            client: Arc::clone(client),
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

/// Search builder for projects with fluent API
pub struct ProjectSearchBuilder {
    client: Arc<YouGileClient>,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<String>,
}

impl ProjectSearchBuilder {
    pub fn new(client: &Arc<YouGileClient>) -> Self {
        Self {
            client: Arc::clone(client),
            include_deleted: None,
            limit: Some(50.0), // Default limit
            offset: Some(0.0),
            title: None,
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

    pub async fn execute(self) -> Result<yougile_client::models::ProjectList, SDKError> {
        self.client
            .search_projects(
                self.include_deleted,
                self.limit,
                self.offset,
                self.title.as_deref(),
            )
            .await
            .map_err(SDKError::from)
    }
}

/// Search builder for users with fluent API
pub struct UserSearchBuilder {
    client: Arc<YouGileClient>,
    limit: Option<f64>,
    offset: Option<f64>,
    email: Option<String>,
    project_id: Option<String>,
}

impl UserSearchBuilder {
    pub fn new(client: &Arc<YouGileClient>) -> Self {
        Self {
            client: Arc::clone(client),
            limit: Some(50.0), // Default limit
            offset: Some(0.0),
            email: None,
            project_id: None,
        }
    }

    pub fn limit(mut self, limit: f64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: f64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn project_id(mut self, project_id: impl Into<String>) -> Self {
        self.project_id = Some(project_id.into());
        self
    }

    pub async fn execute(self) -> Result<yougile_client::models::UserList, SDKError> {
        self.client
            .search_users(
                self.limit,
                self.offset,
                self.email.as_deref(),
                self.project_id.as_deref(),
            )
            .await
            .map_err(SDKError::from)
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
    pub fn new(client: &Arc<YouGileClient>) -> Self {
        Self {
            client: Arc::clone(client),
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

    pub async fn execute(self) -> Result<yougile_client::models::BoardList, SDKError> {
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