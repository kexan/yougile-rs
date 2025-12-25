use crate::api::YouGileAPI;
use crate::config::Config;
use crossterm::event::KeyEvent;
use std::collections::HashMap;
use std::io;
use yougile_client::models::{Project, Board, Task, Column, User};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Projects,
    Boards,
    Tasks,
    TaskDetail,
    Help,
}

pub struct ColumnWithTasks {
    pub column: Column,
    pub tasks: Vec<Task>,
}

pub struct App {
    config: Config,
    api: Option<YouGileAPI>,
    pub current_view: View,
    pub projects: Vec<Project>,
    pub selected_project_idx: usize,
    pub current_project: Option<Project>,
    pub boards: Vec<Board>,
    pub selected_board_idx: usize,
    pub current_board: Option<Board>,
    pub columns: Vec<ColumnWithTasks>,
    pub selected_column_idx: usize,
    pub selected_task_idx: usize,
    pub current_task: Option<Task>,
    pub users: HashMap<String, User>,  // Cache of users by ID
    pub quit: bool,
    pub loading: bool,
    pub error: Option<String>,
    pub focus: FocusedWidget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusedWidget {
    ProjectList,
    BoardList,
    ColumnList,
}

impl App {
    pub async fn new(config: Config) -> Result<Self, io::Error> {
        // Initialize API client
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
            current_task: None,
            users: HashMap::new(),
            quit: false,
            loading: false,
            error: None,
            focus: FocusedWidget::ProjectList,
        };

        // Load projects and users on startup
        app.load_projects().await?;
        app.load_users().await?;

        Ok(app)
    }

    pub async fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()> {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Char('?') | KeyCode::F(1) => {
                log::debug!("Switching to Help view");
                self.current_view = View::Help;
            }
            KeyCode::Char('p') => {
                log::debug!("Switching to Projects view");
                self.current_view = View::Projects;
            }
            KeyCode::Char('q') => {
                self.quit = true;
            }
            KeyCode::Tab => {
                if self.current_view == View::Tasks {
                    self.next_column();
                } else {
                    self.toggle_focus();
                }
                log::debug!("Toggled focus to: {:?}", self.focus);
            }
            KeyCode::BackTab => {
                if self.current_view == View::Tasks {
                    self.prev_column();
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.move_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.move_down();
            }
            KeyCode::Left | KeyCode::Char('h') if self.current_view == View::Tasks => {
                self.prev_column();
            }
            KeyCode::Right | KeyCode::Char('l') if self.current_view == View::Tasks => {
                self.next_column();
            }
            KeyCode::Enter => {
                match self.current_view {
                    View::Projects => {
                        if self.selected_project_idx < self.projects.len() {
                            if let Some(project) = self.projects.get(self.selected_project_idx) {
                                log::info!("Selected project: {:?}", project.title);
                                self.current_project = Some(project.clone());
                                self.load_boards().await?;
                                self.current_view = View::Boards;
                            }
                        }
                    }
                    View::Boards => {
                        if self.selected_board_idx < self.boards.len() {
                            if let Some(board) = self.boards.get(self.selected_board_idx) {
                                log::info!("Selected board: {:?}", board.title);
                                self.current_board = Some(board.clone());
                                self.load_columns_with_tasks().await?;
                                self.current_view = View::Tasks;
                            }
                        }
                    }
                    View::Tasks => {
                        // Open task detail
                        if let Some(column) = self.columns.get(self.selected_column_idx) {
                            if let Some(task) = column.tasks.get(self.selected_task_idx) {
                                log::info!("Opening task: {:?}", task.title);
                                self.current_task = Some(task.clone());
                                self.current_view = View::TaskDetail;
                            }
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Backspace | KeyCode::Esc => {
                // First priority: close error popup if present
                if self.error.is_some() {
                    self.error = None;
                    log::debug!("Closed error popup");
                } else {
                    // Otherwise navigate back
                    match self.current_view {
                        View::Projects => self.quit = true,
                        View::Boards => {
                            self.current_view = View::Projects;
                            self.current_project = None;
                            self.boards.clear();
                            self.selected_board_idx = 0;
                        }
                        View::Tasks => {
                            self.current_view = View::Boards;
                            self.current_board = None;
                            self.columns.clear();
                            self.selected_column_idx = 0;
                            self.selected_task_idx = 0;
                        }
                        View::TaskDetail => {
                            self.current_view = View::Tasks;
                            self.current_task = None;
                        }
                        View::Help => self.current_view = View::Projects,
                    }
                }
            }
            KeyCode::Char('r') => {
                log::info!("User requested refresh");
                match self.current_view {
                    View::Projects => self.load_projects().await?,
                    View::Boards => self.load_boards().await?,
                    View::Tasks => self.load_columns_with_tasks().await?,
                    _ => {},
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub async fn process_pending(&mut self) -> io::Result<()> {
        // Handle any pending async operations
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

    fn move_up(&mut self) {
        match self.current_view {
            View::Projects => {
                if self.selected_project_idx > 0 {
                    self.selected_project_idx -= 1;
                }
            }
            View::Boards => {
                if self.selected_board_idx > 0 {
                    self.selected_board_idx -= 1;
                }
            }
            View::Tasks => {
                if self.selected_task_idx > 0 {
                    self.selected_task_idx -= 1;
                }
            }
            _ => {}
        }
    }

    fn move_down(&mut self) {
        match self.current_view {
            View::Projects => {
                if self.selected_project_idx < self.projects.len().saturating_sub(1) {
                    self.selected_project_idx += 1;
                }
            }
            View::Boards => {
                if self.selected_board_idx < self.boards.len().saturating_sub(1) {
                    self.selected_board_idx += 1;
                }
            }
            View::Tasks => {
                if let Some(column) = self.columns.get(self.selected_column_idx) {
                    if self.selected_task_idx < column.tasks.len().saturating_sub(1) {
                        self.selected_task_idx += 1;
                    }
                }
            }
            _ => {}
        }
    }

    fn next_column(&mut self) {
        if self.selected_column_idx < self.columns.len().saturating_sub(1) {
            self.selected_column_idx += 1;
            self.selected_task_idx = 0;
        }
    }

    fn prev_column(&mut self) {
        if self.selected_column_idx > 0 {
            self.selected_column_idx -= 1;
            self.selected_task_idx = 0;
        }
    }

    fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            FocusedWidget::ProjectList => FocusedWidget::BoardList,
            FocusedWidget::BoardList => FocusedWidget::ColumnList,
            FocusedWidget::ColumnList => FocusedWidget::ProjectList,
        };
    }

    async fn load_projects(&mut self) -> io::Result<()> {
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

    async fn load_boards(&mut self) -> io::Result<()> {
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

    async fn load_columns_with_tasks(&mut self) -> io::Result<()> {
        self.loading = true;
        self.error = None;
        self.columns.clear();
        self.selected_column_idx = 0;
        self.selected_task_idx = 0;

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

    async fn load_users(&mut self) -> io::Result<()> {
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
                        // Don't show error to user, just log it
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
