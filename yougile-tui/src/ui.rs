use crate::app::{App, View};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

// Fixed width for each column in characters
const COLUMN_WIDTH: u16 = 30;

pub fn draw(f: &mut Frame, app: &App) {
    match app.current_view {
        View::Projects => draw_projects_view(f, app),
        View::Boards => draw_boards_view(f, app),
        View::Tasks => draw_kanban_view(f, app),
        View::TaskDetail => draw_task_detail_view(f, app),
        View::Help => draw_help_view(f, app),
    }
}

fn draw_projects_view(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.area());

    // Header
    let header = Paragraph::new("YouGile TUI - Projects")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Projects list
    let projects_block = Block::default()
        .title(" Projects ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded);

    let items: Vec<ListItem> = app
        .projects
        .iter()
        .enumerate()
        .map(|(idx, project)| {
            let name = project.title.clone();
            let content = if idx == app.selected_project_idx {
                Line::from(vec![Span::styled(
                    format!("▶ {}", name),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )])
            } else {
                Line::from(vec![Span::raw(format!("  {}", name))])
            };
            ListItem::new(content)
        })
        .collect();

    let projects_list = List::new(items).block(projects_block);
    f.render_widget(projects_list, chunks[1]);

    // Footer with instructions
    let footer_text = "↵: open | ?: help | ↑/↓ or j/k: navigate | r: refresh | q: quit";
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);

    // Loading indicator
    if app.loading {
        draw_loading_popup(f);
    }

    // Error message
    if let Some(ref error) = app.error {
        draw_error_popup(f, error);
    }
}

fn draw_boards_view(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.area());

    // Header
    let header_text = if let Some(ref project) = app.current_project {
        format!("YouGile TUI - {} / Boards", project.title)
    } else {
        "YouGile TUI - Boards".to_string()
    };
    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Boards list
    let boards_block = Block::default()
        .title(" Boards ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded);

    let items: Vec<ListItem> = app
        .boards
        .iter()
        .enumerate()
        .map(|(idx, board)| {
            let name = board.title.clone();
            let content = if idx == app.selected_board_idx {
                Line::from(vec![Span::styled(
                    format!("▶ {}", name),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )])
            } else {
                Line::from(vec![Span::raw(format!("  {}", name))])
            };
            ListItem::new(content)
        })
        .collect();

    let boards_list = List::new(items).block(boards_block);
    f.render_widget(boards_list, chunks[1]);

    // Footer with instructions
    let footer_text = "↵: open | ?: help | ↑/↓ or j/k: navigate | r: refresh | Esc: back | q: quit";
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);

    // Loading indicator
    if app.loading {
        draw_loading_popup(f);
    }

    // Error message
    if let Some(ref error) = app.error {
        draw_error_popup(f, error);
    }
}

fn draw_kanban_view(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.area());

    // Header
    let header_text = if let Some(ref board) = app.current_board {
        format!("YouGile TUI - {} (Kanban)", board.title)
    } else {
        "YouGile TUI - Kanban Board".to_string()
    };
    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Kanban board - columns with fixed width and horizontal scrolling
    if app.columns.is_empty() {
        let empty_msg = Paragraph::new("No columns found")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(empty_msg, chunks[1]);
    } else {
        // Calculate how many columns fit on screen
        let available_width = chunks[1].width;
        let columns_on_screen = (available_width / COLUMN_WIDTH).max(1) as usize;
        
        // Calculate scroll offset based on selected column
        let scroll_offset = if app.selected_column_idx >= columns_on_screen {
            app.selected_column_idx - columns_on_screen + 1
        } else {
            0
        };
        
        // Get visible columns
        let visible_columns: Vec<_> = app.columns
            .iter()
            .skip(scroll_offset)
            .take(columns_on_screen)
            .enumerate()
            .collect();

        // Create fixed width constraints for visible columns
        let constraints: Vec<Constraint> = vec![Constraint::Length(COLUMN_WIDTH); visible_columns.len()];
        
        let column_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(chunks[1]);

        // Draw each visible column
        for (visible_idx, (col_idx, column_with_tasks)) in visible_columns.iter().enumerate() {
            let actual_col_idx = scroll_offset + col_idx;
            let is_selected = actual_col_idx == app.selected_column_idx;
            
            let border_style = if is_selected {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };

            let title = format!(" {} ({}) ", column_with_tasks.column.title, column_with_tasks.tasks.len());
            let block = Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_style);

            // Create task card items with borders and padding
            let items: Vec<ListItem> = column_with_tasks
                .tasks
                .iter()
                .enumerate()
                .map(|(task_idx, task)| {
                    let is_task_selected = is_selected && task_idx == app.selected_task_idx;
                    
                    // Truncate task name to fit in card width (accounting for borders and padding)
                    let max_width = (COLUMN_WIDTH - 4) as usize; // -4 for borders and padding
                    let task_name = if task.title.len() > max_width {
                        format!("{}...", &task.title[..max_width - 3])
                    } else {
                        task.title.clone()
                    };
                    
                    // Create card lines
                    let mut lines = vec![];
                    
                    // Top border of card
                    lines.push(Line::from("┌".to_string() + &"─".repeat(max_width) + "┐"));
                    
                    // Task title with padding
                    let padded_title = format!(" {}", task_name);
                    let padding_right = max_width.saturating_sub(padded_title.len());
                    let card_line = format!("│{}{} │", padded_title, " ".repeat(padding_right));
                    
                    if is_task_selected {
                        lines.push(Line::from(Span::styled(
                            card_line,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )));
                    } else {
                        lines.push(Line::from(Span::styled(
                            card_line,
                            Style::default().fg(Color::White),
                        )));
                    }
                    
                    // Bottom border of card
                    lines.push(Line::from("└".to_string() + &"─".repeat(max_width) + "┘"));
                    
                    // Empty line for spacing between cards
                    lines.push(Line::from(""));
                    
                    ListItem::new(lines)
                })
                .collect();

            let tasks_list = List::new(items).block(block);
            f.render_widget(tasks_list, column_chunks[visible_idx]);
        }
    }

    // Footer with instructions
    let scroll_indicator = if app.columns.len() > 1 {
        format!(" ({}/{})", app.selected_column_idx + 1, app.columns.len())
    } else {
        String::new()
    };
    let footer_text = format!(
        "↵: open{} | Tab/←/→/h/l: columns | ↑/↓/j/k: tasks | r: refresh | Esc: back | q: quit",
        scroll_indicator
    );
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);

    // Loading indicator
    if app.loading {
        draw_loading_popup(f);
    }

    // Error message
    if let Some(ref error) = app.error {
        draw_error_popup(f, error);
    }
}

fn draw_task_detail_view(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.area());

    if let Some(ref task) = app.current_task {
        // Header with task title
        let header = Paragraph::new(format!("Task: {}", task.title))
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
        f.render_widget(header, chunks[0]);

        // Task details
        let mut details = vec![];
        
        // ID
        if let Some(ref id_common) = task.id_task_common {
            details.push(Line::from(vec![
                Span::styled("ID: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(id_common.clone()),
            ]));
        }
        
        details.push(Line::from(""));

        // Status
        let status = if task.completed.unwrap_or(false) {
            "✓ Completed"
        } else if task.archived.unwrap_or(false) {
            "📦 Archived"
        } else {
            "⏳ In Progress"
        };
        details.push(Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(status),
        ]));

        // Assigned users - show names if available, otherwise count
        if let Some(ref assigned) = task.assigned {
            if !assigned.is_empty() {
                // Try to get names
                let assignee_names: Vec<String> = assigned
                    .iter()
                    .map(|user_id| app.get_user_name(user_id))
                    .collect();
                
                // Check if we got actual names or just IDs
                let has_real_names = assignee_names.iter().any(|name| !name.starts_with("User("));
                
                let display_text = if has_real_names {
                    assignee_names.join(", ")
                } else {
                    format!("{} user(s)", assigned.len())
                };
                
                details.push(Line::from(vec![
                    Span::styled("Assigned: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::raw(display_text),
                ]));
            }
        }

        // Subtasks
        if let Some(ref subtasks) = task.subtasks {
            if !subtasks.is_empty() {
                details.push(Line::from(vec![
                    Span::styled("Subtasks: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::raw(format!("{}", subtasks.len())),
                ]));
            }
        }

        details.push(Line::from(""));
        details.push(Line::from(vec![
            Span::styled("Description:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]));
        details.push(Line::from(""));

        // Description
        if let Some(ref description) = task.description {
            if !description.is_empty() {
                details.push(Line::from(description.clone()));
            } else {
                details.push(Line::from(Span::styled(
                    "No description",
                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
                )));
            }
        } else {
            details.push(Line::from(Span::styled(
                "No description",
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
            )));
        }

        let block = Block::default()
            .title(" Task Details ")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded);

        let details_widget = Paragraph::new(details)
            .block(block)
            .wrap(Wrap { trim: true });
        f.render_widget(details_widget, chunks[1]);

        // Footer
        let footer_text = "Esc: back to kanban | q: quit";
        let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::DarkGray));
        f.render_widget(footer, chunks[2]);
    } else {
        let error_msg = Paragraph::new("No task selected")
            .style(Style::default().fg(Color::Red));
        f.render_widget(error_msg, chunks[1]);
    }

    // Error message
    if let Some(ref error) = app.error {
        draw_error_popup(f, error);
    }
}

fn draw_help_view(f: &mut Frame, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)])
        .split(f.area());

    let help_text = vec![
        Line::from(vec![Span::styled(
            "YouGile TUI - Help",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  j/↓   Move down in current column/list"),
        Line::from("  k/↑   Move up in current column/list"),
        Line::from("  Tab   Switch to next column (Kanban view)"),
        Line::from("  h/←   Previous column (Kanban view)"),
        Line::from("  l/→   Next column (Kanban view)"),
        Line::from(""),
        Line::from("Actions:"),
        Line::from("  ↵     Open selected item (project/board/task)"),
        Line::from("  Esc   Back to previous view / Close error"),
        Line::from("  r     Refresh current view"),
        Line::from(""),
        Line::from("Views:"),
        Line::from("  p     Projects view"),
        Line::from("  ?     Help view"),
        Line::from(""),
        Line::from("General:"),
        Line::from("  q     Quit"),
        Line::from(""),
        Line::from("Note: Logs are written to ~/.cache/yougile-tui/yougile-tui.log"),
    ];

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded);

    let help_widget = Paragraph::new(help_text).block(block);
    f.render_widget(help_widget, chunks[0]);
}

fn draw_loading_popup(f: &mut Frame) {
    let popup_width = 40;
    let popup_height = 5;
    let screen = f.area();

    let x = (screen.width.saturating_sub(popup_width)) / 2;
    let y = (screen.height.saturating_sub(popup_height)) / 2;

    let popup_rect = Rect {
        x,
        y,
        width: popup_width,
        height: popup_height,
    };

    let block = Block::default()
        .title(" Loading ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded);

    let content = Paragraph::new("Loading data...").block(block);
    f.render_widget(content, popup_rect);
}

fn draw_error_popup(f: &mut Frame, error: &str) {
    let popup_width = 60;
    let popup_height = 7;
    let screen = f.area();

    let x = (screen.width.saturating_sub(popup_width)) / 2;
    let y = (screen.height.saturating_sub(popup_height)) / 2;

    let popup_rect = Rect {
        x,
        y,
        width: popup_width,
        height: popup_height,
    };

    let block = Block::default()
        .title(" Error (Press Esc to close) ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Red));

    let content = Paragraph::new(error).block(block);
    f.render_widget(content, popup_rect);
}
