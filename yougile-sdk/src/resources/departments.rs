use crate::SDKError;
use std::sync::Arc;
use yougile_api_client::YouGileClient;
use yougile_api_client::models::*;

/// API for working with departments
pub struct DepartmentsAPI {
    client: Arc<YouGileClient>,
}

impl DepartmentsAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    /// Get a specific department by ID
    pub async fn get(&self, id: &str) -> Result<Department, SDKError> {
        self.client.get_department(id).await.map_err(SDKError::from)
    }

    /// Create a new department
    pub async fn create(&self, create_department: CreateDepartment) -> Result<Id, SDKError> {
        self.client
            .create_department(create_department)
            .await
            .map_err(SDKError::from)
    }

    /// Update an existing department
    pub async fn update(
        &self,
        id: &str,
        update_department: UpdateDepartment,
    ) -> Result<Id, SDKError> {
        self.client
            .update_department(id, update_department)
            .await
            .map_err(SDKError::from)
    }

    /// Search for departments with various filters
    pub async fn search(
        &self,
        include_deleted: Option<bool>,
        limit: Option<f64>,
        offset: Option<f64>,
        title: Option<&str>,
        parent_id: Option<&str>,
    ) -> Result<DepartmentList, SDKError> {
        self.client
            .search_department(include_deleted, limit, offset, title, parent_id)
            .await
            .map_err(SDKError::from)
    }

    /// List all departments (with default parameters)
    pub async fn list(&self) -> Result<DepartmentList, SDKError> {
        self.search(None, Some(100.0), Some(0.0), None, None).await
    }
}
