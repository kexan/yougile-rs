use std::sync::Arc;
use yougile_client::YouGileClient;
use crate::{SDKError, convenience::ProjectSearchBuilder};

/// API for working with projects
pub struct ProjectsAPI {
    client: Arc<YouGileClient>,
}

impl ProjectsAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { 
            client: Arc::clone(&client),
        }
    }

    /// Get a specific project by ID
    pub async fn get(&self, id: &str) -> Result<yougile_client::models::Project, SDKError> {
        self.client.get_project(id).await.map_err(SDKError::from)
    }

    /// Create a new project
    pub async fn create(
        &self, 
        create_project: yougile_client::models::CreateProject
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client.create_project(create_project).await.map_err(SDKError::from)
    }

    /// Update an existing project
    pub async fn update(
        &self, 
        id: &str, 
        update_project: yougile_client::models::UpdateProject
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client.update_project(id, update_project).await.map_err(SDKError::from)
    }

    /// Search for projects with various filters using a fluent API
    pub fn search(&self) -> ProjectSearchBuilder {
        ProjectSearchBuilder::new(&self.client)
    }

    /// List all projects (with default parameters)
    pub async fn list(&self) -> Result<yougile_client::models::ProjectList, SDKError> {
        self.search().execute().await
    }

    /// Create a project role
    pub async fn create_role(
        &self,
        project_id: &str,
        create_role: yougile_client::models::CreateProjectRole,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client.create_project_role(project_id, create_role).await.map_err(SDKError::from)
    }

    /// Get a project role
    pub async fn get_role(
        &self,
        project_id: &str,
        role_id: &str,
    ) -> Result<yougile_client::models::ProjectRole, SDKError> {
        self.client.get_project_role(project_id, role_id).await.map_err(SDKError::from)
    }

    /// Update a project role
    pub async fn update_role(
        &self,
        project_id: &str,
        role_id: &str,
        update_role: yougile_client::models::UpdateProjectRole,
    ) -> Result<yougile_client::models::Id, SDKError> {
        self.client.update_project_role(project_id, role_id, update_role).await.map_err(SDKError::from)
    }

    /// Delete a project role
    pub async fn delete_role(
        &self,
        project_id: &str,
        role_id: &str,
    ) -> Result<yougile_client::models::ProjectRole, SDKError> {
        self.client.delete_project_role(project_id, role_id).await.map_err(SDKError::from)
    }

    /// Search for project roles
    pub async fn search_roles(
        &self,
        project_id: &str,
        limit: Option<f64>,
        offset: Option<f64>,
        name: Option<&str>,
    ) -> Result<yougile_client::models::ProjectRoleList, SDKError> {
        self.client
            .search_project_roles(project_id, limit, offset, name)
            .await
            .map_err(SDKError::from)
    }
}