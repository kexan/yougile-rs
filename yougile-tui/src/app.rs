use crate::config::Config;
use crossterm::event::KeyEvent;
use std::io;
use yougile_client::models::ProjectResponse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Projects,
    Tasks,
    Help,
}

pub struct App {
    config: Config,
    pub current_view: View,
    pub projects: Vec<ProjectResponse>,
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
    pub fn new(config: Config) -> Result<Self, io::Error> {
        Ok(App {
            config,
            current_view: View::Projects,
            projects: Vec::new(),
            selected_project_idx: 0,
            quit: false,
            loading: false,
            error: None,
            focus: FocusedWidget::ProjectList,
        })
    }

    pub async fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()> {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Char('h') | KeyCode::F(1) => {
                self.current_view = View::Help;
            }
            KeyCode::Char('p') => {
                self.current_view = View::Projects;
            }
            KeyCode::Tab => {
                self.toggle_focus();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.move_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.move_down();
            }
            KeyCode::Enter => {
                // Handle selection logic here
                log::debug!("Selected item at index: {}", self.selected_project_idx);
            }
            KeyCode::Char('r') => {
                // Refresh
                self.load_projects().await?;
            }
            _ => {}
        }

        Ok(())
    }

    pub async fn process_pending(&mut self) -> io::Result<()> {
        // Handle any pending async operations
        // For now, this is a placeholder
        Ok(())
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    fn move_up(&mut self) {
        if self.selected_project_idx > 0 {
            self.selected_project_idx -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.selected_project_idx < self.projects.len().saturating_sub(1) {
            self.selected_project_idx += 1;
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

        // TODO: Implement actual API call using yougile-sdk
        // This is where you would:
        // 1. Create a client from config
        // 2. Call client.get_projects()
        // 3. Update self.projects
        // 4. Handle errors

        self.loading = false;
        Ok(())
    }
}
