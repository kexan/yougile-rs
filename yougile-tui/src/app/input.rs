use super::App;
use super::state::View;
use crossterm::event::{KeyCode, KeyEvent};
use std::io;

pub(super) async fn handle_key_event(app: &mut App, key: KeyEvent) -> io::Result<()> {
    match key.code {
        KeyCode::Char('?') | KeyCode::F(1) => {
            log::debug!("Switching to Help view");
            app.current_view = View::Help;
        }
        KeyCode::Char('p') => {
            log::debug!("Switching to Projects view");
            app.current_view = View::Projects;
        }
        KeyCode::Char('q') => {
            app.quit = true;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.move_up();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.move_down();
        }
        KeyCode::Left | KeyCode::Char('h')
            if app.current_view == View::Tasks || app.current_view == View::TaskDetail =>
        {
            app.prev_column();
        }
        KeyCode::Right | KeyCode::Char('l')
            if app.current_view == View::Tasks || app.current_view == View::TaskDetail =>
        {
            app.next_column();
        }
        KeyCode::Enter => {
            handle_enter(app).await?;
        }
        KeyCode::Backspace | KeyCode::Esc => {
            handle_escape(app);
        }
        KeyCode::Char('r') => {
            handle_refresh(app).await?;
        }
        _ => {}
    }

    Ok(())
}

async fn handle_enter(app: &mut App) -> io::Result<()> {
    match app.current_view {
        View::Projects => {
            if app.selected_project_idx < app.projects.len()
                && let Some(project) = app.projects.get(app.selected_project_idx)
            {
                log::info!("Selected project: {:?}", project.title);
                app.current_project = Some(project.clone());
                app.load_boards().await?;
                app.current_view = View::Boards;
            }
        }
        View::Boards => {
            if app.selected_board_idx < app.boards.len()
                && let Some(board) = app.boards.get(app.selected_board_idx)
            {
                log::info!("Selected board: {:?}", board.title);
                app.current_board = Some(board.clone());
                app.load_columns_with_tasks().await?;
                app.current_view = View::Tasks;
            }
        }
        View::Tasks | View::TaskDetail => {
            if let Some(column) = app.columns.get(app.selected_column_idx)
                && let Some(task) = column.tasks.get(app.selected_task_idx)
            {
                log::info!("Opening task: {:?}", task.title);
                app.current_task = Some(task.clone());
                app.current_view = View::TaskDetail;
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_escape(app: &mut App) {
    if app.error.is_some() {
        app.error = None;
        log::debug!("Closed error popup");
    } else {
        match app.current_view {
            View::Projects => app.quit = true,
            View::Boards => {
                app.current_view = View::Projects;
                app.current_project = None;
                app.boards.clear();
                app.selected_board_idx = 0;
            }
            View::Tasks | View::TaskDetail => {
                if app.current_task.is_some() {
                    app.current_task = None;
                    app.current_view = View::Tasks;
                    log::info!("Closed task detail");
                } else {
                    app.current_view = View::Boards;
                    app.current_board = None;
                    app.columns.clear();
                    app.selected_column_idx = 0;
                    app.selected_task_idx = 0;
                    app.task_scroll_offset = 0;
                }
            }
            View::Help => app.current_view = View::Projects,
        }
    }
}

async fn handle_refresh(app: &mut App) -> io::Result<()> {
    log::info!("User requested refresh");
    match app.current_view {
        View::Projects => app.load_projects().await?,
        View::Boards => app.load_boards().await?,
        View::Tasks | View::TaskDetail => app.load_columns_with_tasks().await?,
        _ => {}
    }
    Ok(())
}
