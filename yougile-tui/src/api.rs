use crate::config::Config;
use log::{error, info};
use yougile_client::models::ProjectResponse;
use yougile_sdk::YouGileClient;

pub struct YouGileAPI {
    client: YouGileClient,
}

impl YouGileAPI {
    pub fn new(config: &Config) -> Result<Self, String> {
        let client = YouGileClient::new(&config.api_url, &config.api_token);
        Ok(YouGileAPI { client })
    }

    pub async fn fetch_projects(&self) -> Result<Vec<ProjectResponse>, String> {
        info!("Fetching projects from YouGile API");
        
        match self.client.get_projects().await {
            Ok(projects) => {
                info!("Successfully fetched {} projects", projects.len());
                Ok(projects)
            }
            Err(e) => {
                error!("Failed to fetch projects: {}", e);
                Err(format!("Failed to fetch projects: {}", e))
            }
        }
    }

    pub async fn get_project_details(&self, project_id: &str) -> Result<ProjectResponse, String> {
        info!("Fetching details for project: {}", project_id);
        
        match self.client.get_project(project_id).await {
            Ok(project) => {
                info!("Successfully fetched project details");
                Ok(project)
            }
            Err(e) => {
                error!("Failed to fetch project details: {}", e);
                Err(format!("Failed to fetch project details: {}", e))
            }
        }
    }
}
