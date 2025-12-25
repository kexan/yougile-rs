use crate::config::Config;
use log::{error, info};
use yougile_client::models::{Project, Board, Task};
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

    pub async fn fetch_boards(&self, project_id: &str) -> Result<Vec<Board>, String> {
        info!("Fetching boards for project: {}", project_id);
        
        match self.client.search_boards(None, Some(100.0), None, None, Some(project_id)).await {
            Ok(page) => {
                let boards = page.content;
                info!("Successfully fetched {} boards", boards.len());
                Ok(boards)
            }
            Err(e) => {
                error!("Failed to fetch boards: {}", e);
                Err(format!("Failed to fetch boards: {}", e))
            }
        }
    }

    pub async fn fetch_board_tasks(&self, board_id: &str) -> Result<Vec<Task>, String> {
        info!("Fetching tasks for board: {}", board_id);
        
        match self.client.search_tasks(None, Some(100.0), None, None, Some(board_id), None, None, None).await {
            Ok(page) => {
                let tasks = page.content;
                info!("Successfully fetched {} tasks", tasks.len());
                Ok(tasks)
            }
            Err(e) => {
                error!("Failed to fetch tasks: {}", e);
                Err(format!("Failed to fetch tasks: {}", e))
            }
        }
    }
}
