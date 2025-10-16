use std::sync::Arc;
use yougile_client::{YouGileClient, apis::departments};
use crate::SDKError;

/// API for working with departments
pub struct DepartmentsAPI {
    client: Arc<YouGileClient>,
}

impl DepartmentsAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { 
            client: Arc::clone(&client),
        }
    }

    /// Get a specific department by ID
    pub async fn get(&self, id: &str) -> Result<yougile_client::models::Department, SDKError> {
        departments::get_department(self.client.configuration(), id)
            .await
            .map_err(SDKError::from)
    }

    /// Create a new department
    pub async fn create(
        &self, 
        create_department: yougile_client::models::CreateDepartment
    ) -> Result<yougile_client::models::Id, SDKError> {
        departments::create_department(self.client.configuration(), create_department)
            .await
            .map_err(SDKError::from)
    }

    /// Update an existing department
    pub async fn update(
        &self, 
        id: &str, 
        update_department: yougile_client::models::UpdateDepartment
    ) -> Result<yougile_client::models::Id, SDKError> {
        departments::update_department(self.client.configuration(), id, update_department)
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
    ) -> Result<yougile_client::models::DepartmentList, SDKError> {
        departments::search_department(
            self.client.configuration(),
            include_deleted,
            limit,
            offset,
            title,
            parent_id,
        )
        .await
        .map_err(SDKError::from)
    }

    /// List all departments (with default parameters)
    pub async fn list(&self) -> Result<yougile_client::models::DepartmentList, SDKError> {
        self.search(None, Some(100.0), Some(0.0), None, None).await
    }
}