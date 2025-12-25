use crate::config::Config;
use log::{error, info};
use yougile_client::models::Project;
use yougile_client::apis::configuration::Configuration;
use yougile_client::YouGileClient;

pub struct YouGileAPI {
    client: YouGileClient,
}

impl YouGileAPI {
    pub fn new(config: &Config) -> Result<Self, String> {
        // Create configuration with token
        let configuration = Configuration::new(config.api_token.clone())
            .with_base_path(&config.api_url);

        let client = YouGileClient::new(configuration);
        Ok(YouGileAPI { client })
    }

    pub async fn fetch_projects(&self) -> Result<Vec<Project>, String> {
        info!("Fetching projects from YouGile API");
        
        match self.client.search_projects(None, Some(100.0), None, None).await {
            Ok(page) => {
                let projects = page.content;
                info!("Successfully fetched {} projects", projects.len());
                Ok(projects)
            }
            Err(e) => {
                error!("Failed to fetch projects: {}", e);
                Err(format!("Failed to fetch projects: {}", e))
            }
        }
    }

    pub async fn get_project_details(&self, project_id: &str) -> Result<Project, String> {
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
