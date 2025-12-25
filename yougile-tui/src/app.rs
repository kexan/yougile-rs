use crate::api::YouGileAPI;
use crate::config::Config;
use crossterm::event::KeyEvent;
use std::io;
use yougile_client::models::{Project, Board, Task};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Projects,
    Boards,
    Tasks,
    Help,
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
    pub tasks: Vec<Task>,
    pub selected_task_idx: usize,
    pub quit: bool,
    pub loading: bool,
    pub error: Option<String>,
    pub focus: FocusedWidget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusedWidget {
    ProjectList,
    BoardList,
    TaskList,
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
            tasks: Vec::new(),
            selected_task_idx: 0,
            quit: false,
            loading: false,
            error: None,
            focus: FocusedWidget::ProjectList,
        };

        // Load projects on startup
        app.load_projects().await?;

        Ok(app)
    }

    pub async fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()> {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Char('h') | KeyCode::F(1) => {
                log::debug!("Switching to Help view");
                self.current_view = View::Help;
            }
            KeyCode::Char('p') => {
                log::debug!("Switching to Projects view");
                self.current_view = View::Projects;
            }
            KeyCode::Tab => {
                self.toggle_focus();
                log::debug!("Toggled focus to: {:?}", self.focus);
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.move_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.move_down();
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
                                self.load_tasks().await?;
                                self.current_view = View::Tasks;
                            }
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Backspace | KeyCode::Esc => {
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
                        self.tasks.clear();
                        self.selected_task_idx = 0;
                    }
                    View::Help => self.current_view = View::Projects,
                }
            }
            KeyCode::Char('r') => {
                log::info!("User requested refresh");
                match self.current_view {
                    View::Projects => self.load_projects().await?,
                    View::Boards => self.load_boards().await?,
                    View::Tasks => self.load_tasks().await?,
                    View::Help => {},
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub async fn process_pending(&mut self) -> io::Result<()> {
        // Handle any pending async operations
        // Currently unused, but placeholder for future improvements
        Ok(())
    }

    pub fn should_quit(&self) -> bool {
        self.quit
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
            View::Help => {}
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
                if self.selected_task_idx < self.tasks.len().saturating_sub(1) {
                    self.selected_task_idx += 1;
                }
            }
            View::Help => {}
        }
    }

    fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            FocusedWidget::ProjectList => FocusedWidget::BoardList,
            FocusedWidget::BoardList => FocusedWidget::TaskList,
            FocusedWidget::TaskList => FocusedWidget::ProjectList,
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

    async fn load_tasks(&mut self) -> io::Result<()> {
        self.loading = true;
        self.error = None;
        self.tasks.clear();
        self.selected_task_idx = 0;

        if let Some(ref board) = self.current_board {
            match &self.api {
                Some(api) => {
                    match api.fetch_board_tasks(&board.id).await {
                        Ok(tasks) => {
                            self.tasks = tasks;
                            log::info!("Loaded {} tasks for board {}", self.tasks.len(), board.title);
                        }
                        Err(e) => {
                            self.error = Some(e);
                            log::error!("Failed to load tasks: {:?}", self.error);
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
}
