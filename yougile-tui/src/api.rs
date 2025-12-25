use crate::app::{ColumnWithTasks, StickerMeta};
use crate::config::Config;
use log::{debug, error, info};
use std::collections::HashMap;
use yougile_client::YouGileClient;
use yougile_client::apis::configuration::Configuration;
use yougile_client::models::{Board, Project, User};

pub struct YouGileAPI {
    client: YouGileClient,
}

impl YouGileAPI {
    pub fn new(config: &Config) -> Result<Self, String> {
        // Create configuration with token
        let configuration =
            Configuration::new(config.api_token.clone()).with_base_path(&config.api_url);

        let client = YouGileClient::new(configuration);
        Ok(YouGileAPI { client })
    }

    pub async fn fetch_projects(&self) -> Result<Vec<Project>, String> {
        info!("Fetching projects from YouGile API");

        match self
            .client
            .search_projects(None, Some(100.0), None, None)
            .await
        {
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

    pub async fn fetch_boards(&self, project_id: &str) -> Result<Vec<Board>, String> {
        info!("Fetching boards for project: {}", project_id);

        match self
            .client
            .search_boards(None, Some(100.0), None, None, Some(project_id))
            .await
        {
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

    pub async fn fetch_columns_with_tasks(
        &self,
        board_id: &str,
    ) -> Result<Vec<ColumnWithTasks>, String> {
        info!("Fetching columns with tasks for board: {}", board_id);

        // First, get all columns for this board
        let columns = match self
            .client
            .search_columns(None, Some(100.0), None, None, Some(board_id))
            .await
        {
            Ok(page) => page.content,
            Err(e) => {
                error!("Failed to fetch columns for board {}: {}", board_id, e);
                return Err(format!("Failed to fetch columns: {}", e));
            }
        };

        info!("Found {} columns in board", columns.len());

        // Now fetch tasks for each column
        let mut columns_with_tasks = Vec::new();
        for column in columns {
            let tasks = match self
                .client
                .search_tasks(
                    None,             // include_deleted
                    Some(100.0),      // limit
                    None,             // offset
                    None,             // title
                    Some(&column.id), // column_id
                    None,             // assigned_to
                    None,             // sticker_id
                    None,             // sticker_state_id
                )
                .await
            {
                Ok(page) => {
                    info!(
                        "Found {} tasks in column {}",
                        page.content.len(),
                        column.title
                    );
                    page.content
                }
                Err(e) => {
                    error!("Failed to fetch tasks for column {}: {}", column.id, e);
                    Vec::new() // Continue with empty tasks if fetch fails
                }
            };

            columns_with_tasks.push(ColumnWithTasks { column, tasks });
        }

        info!(
            "Successfully fetched {} columns with tasks",
            columns_with_tasks.len()
        );
        Ok(columns_with_tasks)
    }

    pub async fn fetch_users(&self) -> Result<Vec<User>, String> {
        info!("Fetching users from YouGile API");

        match self
            .client
            .search_users(Some(100.0), None, None, None)
            .await
        {
            Ok(page) => {
                let users = page.content;
                info!("Successfully fetched {} users", users.len());
                Ok(users)
            }
            Err(e) => {
                error!("Failed to fetch users: {}", e);
                Err(format!("Failed to fetch users: {}", e))
            }
        }
    }

    pub async fn fetch_stickers(&self, board_id: Option<&str>) -> Result<Vec<StickerMeta>, String> {
        info!(
            "Fetching all stickers from YouGile API (board filter: {})",
            board_id.unwrap_or("none")
        );

        let mut all_stickers = Vec::new();

        // Fetch sprint stickers WITHOUT board filter to get all stickers (global + board-specific)
        match self
            .client
            .search_sprint_stickers(None, Some(100.0), None, None, None)
            .await
        {
            Ok(page) => {
                let count = page.content.len();
                for sticker in page.content {
                    let mut states = HashMap::new();
                    if let Some(sticker_states) = sticker.states {
                        for state in sticker_states {
                            debug!(
                                "Sprint sticker state: id={}, name={}",
                                state.id, state.data.name
                            );
                            states.insert(state.id.clone(), state.data.name.clone());
                        }
                    }

                    debug!(
                        "Sprint sticker: id={}, name={}, states_count={}",
                        sticker.id,
                        sticker.data.name,
                        states.len()
                    );

                    all_stickers.push(StickerMeta {
                        id: sticker.id.clone(),
                        title: sticker.data.name.clone(),
                        states,
                    });
                }
                info!("Fetched {} sprint stickers", count);
            }
            Err(e) => {
                error!("Failed to fetch sprint stickers: {}", e);
                // Don't fail completely, continue to string stickers
            }
        }

        // Fetch string stickers WITHOUT board filter to get all stickers (global + board-specific)
        match self
            .client
            .search_string_stickers(None, Some(100.0), None, None, None)
            .await
        {
            Ok(page) => {
                let count = page.content.len();
                for sticker in page.content {
                    let mut states = HashMap::new();
                    if let Some(sticker_states) = sticker.states {
                        for state in sticker_states {
                            debug!(
                                "String sticker state: id={}, name={}",
                                state.id, state.data.name
                            );
                            states.insert(state.id.clone(), state.data.name.clone());
                        }
                    }

                    debug!(
                        "String sticker: id={}, name={}, states_count={}",
                        sticker.id,
                        sticker.data.name,
                        states.len()
                    );

                    all_stickers.push(StickerMeta {
                        id: sticker.id.clone(),
                        title: sticker.data.name.clone(),
                        states,
                    });
                }
                info!("Fetched {} string stickers", count);
            }
            Err(e) => {
                error!("Failed to fetch string stickers: {}", e);
                // Don't fail completely
            }
        }

        info!(
            "Successfully fetched {} total stickers (sprint + string)",
            all_stickers.len()
        );
        debug!(
            "Note: String stickers can have both state-based values (state_id) and free-text values"
        );

        // Log all loaded stickers for debugging
        for sticker in &all_stickers {
            debug!(
                "Loaded sticker: id='{}', title='{}', states={}",
                sticker.id,
                sticker.title,
                sticker.states.len()
            );
        }

        Ok(all_stickers)
    }
}
