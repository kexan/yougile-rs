mod styles;
mod utils;
mod views;
mod widgets;

pub use styles::*;
pub use utils::*;
pub use views::*;
pub use widgets::*;

use crate::app::{App, View};
use ratatui::Frame;

pub fn draw(f: &mut Frame, app: &App) {
    match app.current_view {
        View::Projects => draw_projects_view(f, app),
        View::Boards => draw_boards_view(f, app),
        View::Tasks => draw_kanban_view(f, app),
        View::TaskDetail => draw_kanban_view(f, app),
        View::Help => draw_help_view(f, app),
    }
}
