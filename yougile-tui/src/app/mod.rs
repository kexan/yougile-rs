mod state;
mod navigation;
mod input;
mod data_loading;

pub use state::*;
use navigation::*;
use input::*;
use data_loading::*;

use crate::api::YouGileAPI;
use crate::config::Config;
use std::collections::HashMap;
use std::io;
use yougile_client::models::User;

pub struct App {
    config: Config,
    api: Option<YouGileAPI>,
    pub current_view: View,
    pub projects: Vec<yougile_client::models::Project>,
    pub selected_project_idx: usize,
    pub current_project: Option<yougile_client::models::Project>,
    pub boards: Vec<yougile_client::models::Board>,
    pub selected_board_idx: usize,
    pub current_board: Option<yougile_client::models::Board>,
    pub columns: Vec<ColumnWithTasks>,
    pub selected_column_idx: usize,
    pub selected_task_idx: usize,
    pub task_scroll_offset: usize,
    pub current_task: Option<yougile_client::models::Task>,
    pub users: HashMap<String, User>,
    pub stickers: HashMap<String, StickerMeta>,
    pub quit: bool,
    pub loading: bool,
    pub error: Option<String>,
    pub focus: FocusedWidget,
}

impl App {
    pub async fn new(config: Config) -> Result<Self, io::Error> {
        let api = match YouGileAPI::new(&config) {
            Ok(api) => Some(api),
            Err(e) => {
                log::warn!("Failed to initialize API client: {}", e);
                None
            }
        };

        let mut app = App {
            config,
            api,
            current_view: View::Projects,
            projects: Vec::new(),
            selected_project_idx: 0,
            current_project: None,
            boards: Vec::new(),
            selected_board_idx: 0,
            current_board: None,
            columns: Vec::new(),
            selected_column_idx: 0,
            selected_task_idx: 0,
            task_scroll_offset: 0,
            current_task: None,
            users: HashMap::new(),
            stickers: HashMap::new(),
            quit: false,
            loading: false,
            error: None,
            focus: FocusedWidget::ProjectList,
        };

        app.load_projects().await?;
        app.load_users().await?;
        app.load_stickers().await?;

        Ok(app)
    }

    pub async fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> io::Result<()> {
        handle_key_event(self, key).await
    }

    pub async fn process_pending(&mut self) -> io::Result<()> {
        Ok(())
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    pub fn get_user_name(&self, user_id: &str) -> String {
        self.users
            .get(user_id)
            .map(|u| u.real_name.clone())
            .unwrap_or_else(|| format!("User({})", &user_id[..8.min(user_id.len())]))
    }

    pub fn get_sticker_title(&self, sticker_id: &str) -> String {
        self.stickers
            .get(sticker_id)
            .map(|s| s.title.clone())
            .unwrap_or_else(|| format!("Sticker({})", &sticker_id[..8.min(sticker_id.len())]))
    }

    pub fn get_sticker_state_title(&self, sticker_id: &str, state_id: &str) -> String {
        if let Some(sticker) = self.stickers.get(sticker_id) {
            sticker.states
                .get(state_id)
                .cloned()
                .unwrap_or_else(|| format!("State({})", &state_id[..8.min(state_id.len())]))
        } else {
            format!("State({})", &state_id[..8.min(state_id.len())])
        }
    }
}
