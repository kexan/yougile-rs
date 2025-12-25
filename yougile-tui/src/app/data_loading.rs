use super::{App, StickerMeta};
use std::io;

impl App {
    pub(super) async fn load_projects(&mut self) -> io::Result<()> {
        self.loading = true;
        self.error = None;

        match &self.api {
            Some(api) => {
                match api.fetch_projects().await {
                    Ok(projects) => {
                        self.projects = projects;
                        self.selected_project_idx = 0;
                        log::info!("Loaded {} projects", self.projects.len());
                    }
                    Err(e) => {
                        self.error = Some(e);
                        log::error!("Failed to load projects: {:?}", self.error);
                    }
                }
            }
            None => {
                self.error = Some("API client not initialized".to_string());
                log::error!("API client is not initialized");
            }
        }

        self.loading = false;
        Ok(())
    }

    pub(super) async fn load_boards(&mut self) -> io::Result<()> {
        self.loading = true;
        self.error = None;
        self.boards.clear();
        self.selected_board_idx = 0;

        if let Some(ref project) = self.current_project {
            match &self.api {
                Some(api) => {
                    match api.fetch_boards(&project.id).await {
                        Ok(boards) => {
                            self.boards = boards;
                            log::info!("Loaded {} boards for project {}", self.boards.len(), project.title);
                        }
                        Err(e) => {
                            self.error = Some(e);
                            log::error!("Failed to load boards: {:?}", self.error);
                        }
                    }
                }
                None => {
                    self.error = Some("API client not initialized".to_string());
                    log::error!("API client is not initialized");
                }
            }
        }

        self.loading = false;
        Ok(())
    }

    pub(super) async fn load_columns_with_tasks(&mut self) -> io::Result<()> {
        self.loading = true;
        self.error = None;
        self.columns.clear();
        self.selected_column_idx = 0;
        self.selected_task_idx = 0;
        self.task_scroll_offset = 0;

        if let Some(ref board) = self.current_board {
            match &self.api {
                Some(api) => {
                    match api.fetch_columns_with_tasks(&board.id).await {
                        Ok(columns) => {
                            self.columns = columns;
                            log::info!("Loaded {} columns for board {}", self.columns.len(), board.title);
                        }
                        Err(e) => {
                            self.error = Some(e);
                            log::error!("Failed to load columns: {:?}", self.error);
                        }
                    }
                }
                None => {
                    self.error = Some("API client not initialized".to_string());
                    log::error!("API client is not initialized");
                }
            }
        }

        self.loading = false;
        Ok(())
    }

    pub(super) async fn load_users(&mut self) -> io::Result<()> {
        log::info!("Loading users...");
        
        match &self.api {
            Some(api) => {
                match api.fetch_users().await {
                    Ok(users) => {
                        self.users = users.into_iter().map(|u| (u.id.clone(), u)).collect();
                        log::info!("Loaded {} users", self.users.len());
                    }
                    Err(e) => {
                        log::error!("Failed to load users: {}", e);
                    }
                }
            }
            None => {
                log::error!("API client is not initialized");
            }
        }

        Ok(())
    }

    pub(super) async fn load_stickers(&mut self) -> io::Result<()> {
        log::info!("Loading all stickers...");
        
        match &self.api {
            Some(api) => {
                match api.fetch_stickers(None).await {
                    Ok(stickers) => {
                        self.stickers = stickers.into_iter().map(|s| (s.id.clone(), s)).collect();
                        log::info!("Loaded {} stickers", self.stickers.len());
                    }
                    Err(e) => {
                        log::error!("Failed to load stickers: {}", e);
                    }
                }
            }
            None => {
                log::error!("API client is not initialized");
            }
        }

        Ok(())
    }
}
