use super::App;
use super::state::View;

impl App {
    pub(super) fn move_up(&mut self) {
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
            View::Tasks | View::TaskDetail => {
                if self.selected_task_idx > 0 {
                    self.selected_task_idx -= 1;
                }
            }
            _ => {}
        }
    }

    pub(super) fn move_down(&mut self) {
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
            View::Tasks | View::TaskDetail => {
                if let Some(column) = self.columns.get(self.selected_column_idx)
                    && self.selected_task_idx < column.tasks.len().saturating_sub(1)
                {
                    self.selected_task_idx += 1;
                }
            }
            _ => {}
        }
    }

    pub(super) fn next_column(&mut self) {
        if self.selected_column_idx < self.columns.len().saturating_sub(1) {
            self.selected_column_idx += 1;
            self.selected_task_idx = 0;
            self.task_scroll_offset = 0;
        }
    }

    pub(super) fn prev_column(&mut self) {
        if self.selected_column_idx > 0 {
            self.selected_column_idx -= 1;
            self.selected_task_idx = 0;
            self.task_scroll_offset = 0;
        }
    }
}
