use crate::app::{App, View};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use std::rc::Rc;

// Fixed width for each column in characters
const COLUMN_WIDTH: u16 = 40;
// Padding between columns
const COLUMN_PADDING: u16 = 1;
// Maximum columns to display at once (to reserve space for task detail panel)
const MAX_COLUMNS_VISIBLE: usize = 4;
// Minimum width for task detail panel
const TASK_DETAIL_MIN_WIDTH: u16 = 50;
// Padding between columns area and task detail panel
const PANEL_PADDING: u16 = 2;

pub fn draw(f: &mut Frame, app: &App) {
    match app.current_view {
        View::Projects => draw_projects_view(f, app),
        View::Boards => draw_boards_view(f, app),
        View::Tasks => draw_kanban_view(f, app),
        View::TaskDetail => draw_kanban_view(f, app), // Same view, task detail shown on the right
        View::Help => draw_help_view(f, app),
    }
}

/// Map column color index (0-15) to ratatui Color
fn get_column_color(color_index: Option<u8>) -> Color {
    match color_index {
        Some(0) => Color::Gray,
        Some(1) => Color::Red,
        Some(2) => Color::LightRed,
        Some(3) => Color::Green,
        Some(4) => Color::LightGreen,
        Some(5) => Color::Yellow,
        Some(6) => Color::LightYellow,
        Some(7) => Color::Blue,
        Some(8) => Color::LightBlue,
        Some(9) => Color::Magenta,
        Some(10) => Color::LightMagenta,
        Some(11) => Color::Cyan,
        Some(12) => Color::LightCyan,
        Some(13) => Color::White,
        Some(14) => Color::DarkGray,
        Some(15) => Color::Rgb(138, 43, 226), // BlueViolet
        _ => Color::White, // Default
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

/// Wrap text into lines of max_width characters
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for word in text.split_whitespace() {
        let word_len = word.chars().count();
        let current_len = current_line.chars().count();
        
        if current_len == 0 {
            // First word in line
            if word_len > max_width {
                // Word too long, split it
                let chars: Vec<char> = word.chars().collect();
                for chunk in chars.chunks(max_width) {
                    lines.push(chunk.iter().collect());
                }
            } else {
                current_line = word.to_string();
            }
        } else if current_len + 1 + word_len <= max_width {
            // Word fits in current line
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            // Word doesn't fit, start new line
            lines.push(current_line);
            current_line = word.to_string();
        }
    }
    
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    if lines.is_empty() {
        lines.push(String::new());
    }
    
    lines
}

/// Calculate card height (number of lines) for a task
fn calculate_card_height(task_title: &str, max_width: usize) -> usize {
    let wrapped = wrap_text(task_title, max_width);
    3 + wrapped.len() // top border + content lines + bottom border
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

    // Calculate how many columns can fit and their total width
    let available_width = chunks[1].width;
    let column_with_padding = COLUMN_WIDTH + COLUMN_PADDING;
    
    // Split main area into columns and task detail if task is open
    let main_chunks: Rc<[Rect]> = if app.current_task.is_some() {
        // Calculate columns area width:
        // We want to show up to MAX_COLUMNS_VISIBLE columns
        let visible_columns = app.columns.len().min(MAX_COLUMNS_VISIBLE);
        let columns_width = if visible_columns > 0 {
            (visible_columns as u16 * COLUMN_WIDTH) + ((visible_columns.saturating_sub(1)) as u16 * COLUMN_PADDING)
        } else {
            COLUMN_WIDTH
        };
        
        // Task detail panel gets remaining space (but at least minimum width)
        // Total width = columns_width + padding + task_detail_width
        // So: task_detail_width = available_width - columns_width - padding
        let task_detail_width = available_width.saturating_sub(columns_width).saturating_sub(PANEL_PADDING).max(TASK_DETAIL_MIN_WIDTH);
        
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(columns_width),
                Constraint::Length(PANEL_PADDING),
                Constraint::Min(task_detail_width),
            ])
            .split(chunks[1])
    } else {
        Rc::from(vec![chunks[1]].as_slice())
    };

    let columns_area = main_chunks[0];

    // Kanban board - columns with fixed width and horizontal scrolling
    if app.columns.is_empty() {
        let empty_msg = Paragraph::new("No columns found")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(empty_msg, columns_area);
    } else {
        // Calculate how many columns fit on screen (with padding and max limit)
        let available_width = columns_area.width;
        let column_with_padding = COLUMN_WIDTH + COLUMN_PADDING;
        let columns_that_fit = ((available_width + COLUMN_PADDING) / column_with_padding).max(1) as usize;
        let columns_on_screen = columns_that_fit.min(MAX_COLUMNS_VISIBLE);
        
        // Calculate scroll offset based on selected column
        let scroll_offset = if app.selected_column_idx >= columns_on_screen {
            app.selected_column_idx - columns_on_screen + 1
        } else {
            0
        };
        
        // Check if there are hidden columns
        let has_left_columns = scroll_offset > 0;
        let has_right_columns = scroll_offset + columns_on_screen < app.columns.len();
        
        // Get visible columns with their original indices
        let visible_columns: Vec<(usize, &crate::app::ColumnWithTasks)> = app.columns
            .iter()
            .enumerate()
            .skip(scroll_offset)
            .take(columns_on_screen)
            .collect();

        // Create constraints with padding
        let mut constraints = Vec::new();
        for i in 0..visible_columns.len() {
            constraints.push(Constraint::Length(COLUMN_WIDTH));
            if i < visible_columns.len() - 1 {
                constraints.push(Constraint::Length(COLUMN_PADDING));
            }
        }
        
        let column_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(columns_area);

        // Draw each visible column
        for (visible_idx, (actual_col_idx, column_with_tasks)) in visible_columns.iter().enumerate() {
            let is_selected = *actual_col_idx == app.selected_column_idx;
            
            // Get column color from API
            let column_color = get_column_color(column_with_tasks.column.color);
            
            let border_style = if is_selected {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(column_color)
            };

            // Calculate which chunk to use (accounting for padding)
            let chunk_idx = visible_idx * 2;

            // Count non-archived tasks only
            let active_tasks_count = column_with_tasks.tasks.iter()
                .filter(|t| !t.archived.unwrap_or(false))
                .count();

            // Add scroll indicators to title
            let left_indicator = if visible_idx == 0 && has_left_columns { "◀ " } else { "" };
            let right_indicator = if visible_idx == visible_columns.len() - 1 && has_right_columns { " ▶" } else { "" };
            let title = format!("{}{}{}({}) {}", 
                left_indicator,
                column_with_tasks.column.title, 
                " ",
                active_tasks_count,  // Only non-archived count
                right_indicator
            );
            
            let block = Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(border_style);

            // Calculate available height for tasks
            let available_height = column_chunks[chunk_idx].height.saturating_sub(2) as usize; // -2 for borders
            
            // Sort tasks: non-archived first, then archived
            let mut sorted_tasks: Vec<(usize, &yougile_client::models::Task)> = column_with_tasks.tasks
                .iter()
                .enumerate()
                .collect();
            sorted_tasks.sort_by_key(|(_, task)| task.archived.unwrap_or(false));
            
            // Calculate which tasks to show based on scroll offset
            let max_width = (COLUMN_WIDTH - 6) as usize;
            let mut task_scroll_offset = 0;
            let mut cumulative_height = 0;
            let mut visible_task_start = 0;
            
            // If this is the selected column, calculate scroll
            if is_selected && app.selected_task_idx > 0 {
                for (idx, (_, task)) in sorted_tasks.iter().enumerate() {
                    let card_height = calculate_card_height(&task.title, max_width);
                    if idx < app.selected_task_idx {
                        cumulative_height += card_height;
                        // Scroll if selected task would be below visible area
                        if cumulative_height > available_height.saturating_sub(card_height) {
                            visible_task_start = idx + 1;
                            task_scroll_offset = cumulative_height;
                        }
                    } else {
                        break;
                    }
                }
            }
            
            // Check if there are tasks above/below visible area
            let has_tasks_above = visible_task_start > 0;
            let mut has_tasks_below = false;
            let mut current_height = 0;
            for (idx, (_, task)) in sorted_tasks.iter().enumerate().skip(visible_task_start) {
                let card_height = calculate_card_height(&task.title, max_width);
                current_height += card_height;
                if current_height > available_height {
                    has_tasks_below = true;
                    break;
                }
            }

            // Create task card items with borders and text wrapping
            let items: Vec<ListItem> = sorted_tasks
                .iter()
                .skip(visible_task_start)
                .enumerate()
                .map(|(display_idx, (original_task_idx, task))| {
                    let actual_task_idx = display_idx + visible_task_start;
                    let is_task_selected = is_selected && actual_task_idx == app.selected_task_idx;
                    let is_archived = task.archived.unwrap_or(false);
                    
                    // Wrap task name to fit in card width
                    let wrapped_lines = wrap_text(&task.title, max_width);
                    
                    // Create card lines
                    let mut lines = vec![];
                    
                    // Top border of card with scroll indicator
                    let top_border = if actual_task_idx == 0 && has_tasks_above {
                        // Add up arrow in the middle
                        let half_width = max_width / 2;
                        let left_part = "─".repeat(half_width);
                        let right_part = "─".repeat(max_width - half_width);
                        format!("┌{}▲{}┐", left_part, right_part)
                    } else {
                        format!("┌{}┐", "─".repeat(max_width + 2))
                    };
                    lines.push(Line::from(top_border));
                    
                    // Task title lines with padding
                    for line_text in wrapped_lines {
                        let padded = format!(" {} ", line_text);
                        let padding_right = (max_width + 2).saturating_sub(padded.chars().count());
                        let card_line = format!("│{}{}│", padded, " ".repeat(padding_right));
                        
                        let style = if is_archived {
                            // Archived tasks are dimmed
                            Style::default().fg(Color::DarkGray)
                        } else if is_task_selected {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        };
                        
                        lines.push(Line::from(Span::styled(card_line, style)));
                    }
                    
                    // Bottom border of card
                    lines.push(Line::from(format!("└{}┘", "─".repeat(max_width + 2))));
                    
                    ListItem::new(lines)
                })
                .collect();

            // Add scroll indicator at bottom if needed
            let tasks_list = if has_tasks_below {
                let mut items_with_indicator = items;
                items_with_indicator.push(ListItem::new(Line::from(Span::styled(
                    format!("{:^width$}", "▼", width = max_width + 4),
                    Style::default().fg(Color::DarkGray),
                ))));
                List::new(items_with_indicator).block(block)
            } else {
                List::new(items).block(block)
            };

            f.render_widget(tasks_list, column_chunks[chunk_idx]);
        }
    }

    // Draw task detail panel on the right if task is open (skip padding area at index 1)
    if app.current_task.is_some() && main_chunks.len() > 2 {
        draw_task_detail_panel(f, app, main_chunks[2]);
    }

    // Footer with instructions
    let scroll_indicator = if app.columns.len() > 1 {
        format!(" ({}/{})", app.selected_column_idx + 1, app.columns.len())
    } else {
        String::new()
    };
    let footer_text = format!(
        "↵: open task{} | ←/→/h/l: columns | ↑/↓/j/k: tasks | r: refresh | Esc: {} | q: quit",
        scroll_indicator,
        if app.current_task.is_some() { "close task" } else { "back" }
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

fn draw_task_detail_panel(f: &mut Frame, app: &App, area: Rect) {
    if let Some(ref task) = app.current_task {
        // Task details
        let mut details = vec![];
        
        // Title
        details.push(Line::from(vec![Span::styled(
            &task.title,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )]));
        details.push(Line::from(""));
        
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
        f.render_widget(details_widget, area);
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
        Line::from("  h/←   Previous column (Kanban view)"),
        Line::from("  l/→   Next column (Kanban view)"),
        Line::from(""),
        Line::from("Actions:"),
        Line::from("  ↵     Open task details (replaces current if open)"),
        Line::from("  Esc   Close task / Back to previous view / Close error"),
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
        Line::from(""),
        Line::from("Columns: Color-coded by API • Count shows non-archived tasks only"),
        Line::from("Tasks: Archived tasks are dimmed and sorted to bottom"),
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
