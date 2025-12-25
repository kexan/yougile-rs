use crate::api::YouGileAPI;
use crate::config::Config;
use crossterm::event::KeyEvent;
use std::io;
use yougile_client::models::Project;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Projects,
    Tasks,
    Help,
}

pub struct App {
    config: Config,
    api: Option<YouGileAPI>,
    pub current_view: View,
    pub projects: Vec<Project>,
    pub selected_project_idx: usize,
    pub quit: bool,
    pub loading: bool,
    pub error: Option<String>,
    pub focus: FocusedWidget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusedWidget {
    ProjectList,
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
                if self.selected_project_idx < self.projects.len() {
                    if let Some(project) = self.projects.get(self.selected_project_idx) {
                        log::info!("Selected project: {:?}", project.title);
                    }
                }
            }
            KeyCode::Char('r') => {
                log::info!("User requested refresh");
                self.load_projects().await?;
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
        if self.selected_project_idx > 0 {
            self.selected_project_idx -= 1;
            log::debug!("Moved up, now at index: {}", self.selected_project_idx);
        }
    }

    fn move_down(&mut self) {
        if self.selected_project_idx < self.projects.len().saturating_sub(1) {
            self.selected_project_idx += 1;
            log::debug!("Moved down, now at index: {}", self.selected_project_idx);
        }
    }

    fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            FocusedWidget::ProjectList => FocusedWidget::TaskList,
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
}
