use crate::SDKError;
use std::sync::Arc;
use yougile_client::{
    YouGileClient,
    models::{
        CreateProject, CreateProjectRole, Id, Project, ProjectList, ProjectRole, ProjectRoleList,
        UpdateProject, UpdateProjectRole,
    },
};

/// API for working with projects
pub struct ProjectsAPI {
    client: Arc<YouGileClient>,
}

impl ProjectsAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    /// Get a specific project by ID
    pub async fn get(&self, id: &str) -> Result<Project, SDKError> {
        self.client.get_project(id).await.map_err(SDKError::from)
    }

    /// Create a new project
    pub async fn create(&self, create_project: CreateProject) -> Result<Id, SDKError> {
        self.client
            .create_project(create_project)
            .await
            .map_err(SDKError::from)
    }

    /// Update an existing project
    pub async fn update(&self, id: &str, update_project: UpdateProject) -> Result<Id, SDKError> {
        self.client
            .update_project(id, update_project)
            .await
            .map_err(SDKError::from)
    }

    /// Search for projects with various filters using a fluent API
    pub fn search(&self) -> ProjectSearchBuilder {
        ProjectSearchBuilder::new(self.client.clone())
    }

    /// List all projects (with default parameters)
    pub async fn list(&self) -> Result<ProjectList, SDKError> {
        self.search().execute().await
    }

    /// List all projects with automatic pagination
    pub async fn list_all(&self) -> Result<Vec<Project>, SDKError> {
        self.search().all().await
    }

    // Project Role methods

    /// Create a project role
    pub async fn create_role(
        &self,
        project_id: &str,
        create_project_role: CreateProjectRole,
    ) -> Result<Id, SDKError> {
        self.client
            .create_project_role(project_id, create_project_role)
            .await
            .map_err(SDKError::from)
    }

    /// Get a project role
    pub async fn get_role(&self, project_id: &str, id: &str) -> Result<ProjectRole, SDKError> {
        self.client
            .get_project_role(project_id, id)
            .await
            .map_err(SDKError::from)
    }

    /// Update a project role
    pub async fn update_role(
        &self,
        project_id: &str,
        id: &str,
        update_project_role: UpdateProjectRole,
    ) -> Result<Id, SDKError> {
        self.client
            .update_project_role(project_id, id, update_project_role)
            .await
            .map_err(SDKError::from)
    }

    /// Delete a project role
    pub async fn delete_role(&self, project_id: &str, id: &str) -> Result<ProjectRole, SDKError> {
        self.client
            .delete_project_role(project_id, id)
            .await
            .map_err(SDKError::from)
    }

    /// Search for project roles
    pub fn search_roles(&self, project_id: &str) -> ProjectRoleSearchBuilder {
        ProjectRoleSearchBuilder::new(self.client.clone(), project_id.to_string())
    }

    /// List project roles
    pub async fn list_roles(&self, project_id: &str) -> Result<ProjectRoleList, SDKError> {
        self.search_roles(project_id).execute().await
    }

    /// List all project roles with automatic pagination
    pub async fn list_roles_all(&self, project_id: &str) -> Result<Vec<ProjectRole>, SDKError> {
        self.search_roles(project_id).all().await
    }
}

/// Search builder for projects with fluent API
#[derive(Clone)]
pub struct ProjectSearchBuilder {
    client: Arc<YouGileClient>,
    include_deleted: Option<bool>,
    limit: Option<f64>,
    offset: Option<f64>,
    title: Option<String>,
}

impl ProjectSearchBuilder {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self {
            client,
            include_deleted: None,
            limit: Some(100.0),
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

    /// Execute the search with current parameters
    pub async fn execute(self) -> Result<ProjectList, SDKError> {
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

    /// Get all projects matching the search criteria with automatic pagination
    pub async fn all(self) -> Result<Vec<Project>, SDKError> {
        let mut all_projects = Vec::new();
        let mut offset = 0.0;
        let limit = self.limit.unwrap_or(100.0);

        loop {
            let result = self.clone().offset(offset).execute().await?;
            let count = result.content.len() as f64;
            all_projects.extend(result.content);

            if count < limit {
                break;
            }
            offset += limit;
        }

        Ok(all_projects)
    }
}

/// Search builder for project roles with fluent API
#[derive(Clone)]
pub struct ProjectRoleSearchBuilder {
    client: Arc<YouGileClient>,
    project_id: String,
    limit: Option<f64>,
    offset: Option<f64>,
    name: Option<String>,
}

impl ProjectRoleSearchBuilder {
    pub fn new(client: Arc<YouGileClient>, project_id: String) -> Self {
        Self {
            client,
            project_id,
            limit: Some(100.0),
            offset: Some(0.0),
            name: None,
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

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Execute the search with current parameters
    pub async fn execute(self) -> Result<ProjectRoleList, SDKError> {
        self.client
            .search_project_roles(
                &self.project_id,
                self.limit,
                self.offset,
                self.name.as_deref(),
            )
            .await
            .map_err(SDKError::from)
    }

    /// Get all project roles matching the search criteria with automatic pagination
    pub async fn all(self) -> Result<Vec<ProjectRole>, SDKError> {
        let mut all_roles = Vec::new();
        let mut offset = 0.0;
        let limit = self.limit.unwrap_or(100.0);

        loop {
            let result = self.clone().offset(offset).execute().await?;
            let count = result.content.len() as f64;
            all_roles.extend(result.content);

            if count < limit {
                break;
            }
            offset += limit;
        }

        Ok(all_roles)
    }
}
