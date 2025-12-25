use crate::app::{App, ColumnWithTasks};
use crate::ui::{
    get_column_color,
    widgets::{
        build_task_card_lines, calculate_card_height, draw_error_popup, draw_loading_popup,
        draw_task_detail_panel,
    },
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::rc::Rc;
use yougile_api_client::models::Task;

const COLUMN_WIDTH: u16 = 40;
const COLUMN_PADDING: u16 = 1;
const MAX_COLUMNS_VISIBLE: usize = 4;
const TASK_DETAIL_MIN_WIDTH: u16 = 50;
const PANEL_PADDING: u16 = 2;

pub fn draw_kanban_view(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(f.area());

    let header_text = if let Some(ref board) = app.current_board {
        format!("YouGile TUI - {} (Kanban)", board.title)
    } else {
        "YouGile TUI - Kanban Board".to_string()
    };
    let header = Paragraph::new(header_text).style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(ratatui::style::Modifier::BOLD),
    );
    f.render_widget(header, chunks[0]);

    let main_chunks: Rc<[Rect]> = if app.current_task.is_some() {
        let visible_columns = app.columns.len().min(MAX_COLUMNS_VISIBLE);
        let columns_width = if visible_columns > 0 {
            (visible_columns as u16 * COLUMN_WIDTH)
                + ((visible_columns.saturating_sub(1)) as u16 * COLUMN_PADDING)
        } else {
            COLUMN_WIDTH
        };

        let available_width = chunks[1].width;
        let task_detail_width = available_width
            .saturating_sub(columns_width)
            .saturating_sub(PANEL_PADDING)
            .max(TASK_DETAIL_MIN_WIDTH);

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
        draw_columns(f, app, columns_area);
    }

    if app.current_task.is_some() && main_chunks.len() > 2 {
        draw_task_detail_panel(f, app, main_chunks[2]);
    }

    let scroll_indicator = if app.columns.len() > 1 {
        format!(" ({}/{})", app.selected_column_idx + 1, app.columns.len())
    } else {
        String::new()
    };
    let footer_text = format!(
        "↵: open task{} | ←/→/h/l: columns | ↑/↓/j/k: tasks | r: refresh | Esc: {} | q: quit",
        scroll_indicator,
        if app.current_task.is_some() {
            "close task"
        } else {
            "back"
        }
    );
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);

    if app.loading {
        draw_loading_popup(f);
    }

    if let Some(ref error) = app.error {
        draw_error_popup(f, error);
    }
}

fn draw_columns(f: &mut Frame, app: &App, area: Rect) {
    let available_width = area.width;
    let column_with_padding = COLUMN_WIDTH + COLUMN_PADDING;
    let columns_that_fit =
        ((available_width + COLUMN_PADDING) / column_with_padding).max(1) as usize;
    let columns_on_screen = columns_that_fit.min(MAX_COLUMNS_VISIBLE);

    let scroll_offset = if app.selected_column_idx >= columns_on_screen {
        app.selected_column_idx - columns_on_screen + 1
    } else {
        0
    };

    let has_left_columns = scroll_offset > 0;
    let has_right_columns = scroll_offset + columns_on_screen < app.columns.len();

    let visible_columns: Vec<(usize, &ColumnWithTasks)> = app
        .columns
        .iter()
        .enumerate()
        .skip(scroll_offset)
        .take(columns_on_screen)
        .collect();

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
        .split(area);

    for (visible_idx, (actual_col_idx, column_with_tasks)) in visible_columns.iter().enumerate() {
        let chunk_idx = visible_idx * 2;
        draw_single_column(
            f,
            app,
            column_chunks[chunk_idx],
            column_with_tasks,
            *actual_col_idx,
            visible_idx,
            visible_columns.len(),
            has_left_columns,
            has_right_columns,
        );
    }
}

fn draw_single_column(
    f: &mut Frame,
    app: &App,
    area: Rect,
    column_with_tasks: &ColumnWithTasks,
    actual_col_idx: usize,
    visible_idx: usize,
    visible_count: usize,
    has_left: bool,
    has_right: bool,
) {
    let is_selected = actual_col_idx == app.selected_column_idx;
    let column_color = get_column_color(column_with_tasks.column.color);

    let border_style = if is_selected {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(column_color)
    };

    let active_tasks_count = column_with_tasks
        .tasks
        .iter()
        .filter(|t| !t.archived.unwrap_or(false))
        .count();

    let left_indicator = if visible_idx == 0 && has_left {
        "◀ "
    } else {
        ""
    };
    let right_indicator = if visible_idx == visible_count - 1 && has_right {
        " ▶"
    } else {
        ""
    };
    let title = format!(
        "{}{}{} ({}) {}",
        left_indicator, column_with_tasks.column.title, "", active_tasks_count, right_indicator
    );

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(border_style);

    let available_height = area.height.saturating_sub(2) as usize;
    let max_width = (COLUMN_WIDTH - 6) as usize;

    let mut sorted_tasks: Vec<(usize, &Task)> =
        column_with_tasks.tasks.iter().enumerate().collect();
    sorted_tasks.sort_by_key(|(_, task)| task.archived.unwrap_or(false));

    let mut cumulative_height = 0;
    let mut visible_task_start = 0;

    if is_selected && app.selected_task_idx > 0 {
        for (idx, (_, task)) in sorted_tasks.iter().enumerate() {
            let card_height = calculate_card_height(app, task, max_width);
            if idx < app.selected_task_idx {
                cumulative_height += card_height;
                if cumulative_height > available_height.saturating_sub(card_height) {
                    visible_task_start = idx + 1;
                }
            } else {
                break;
            }
        }
    }

    let has_tasks_above = visible_task_start > 0;
    let mut has_tasks_below = false;
    let mut current_height = 0;
    for (_, task) in sorted_tasks.iter().skip(visible_task_start) {
        let card_height = calculate_card_height(app, task, max_width);
        current_height += card_height;
        if current_height > available_height {
            has_tasks_below = true;
            break;
        }
    }

    let items: Vec<ListItem> = sorted_tasks
        .iter()
        .skip(visible_task_start)
        .enumerate()
        .map(|(display_idx, (_original_task_idx, task))| {
            let actual_task_idx = display_idx + visible_task_start;
            let is_task_selected = is_selected && actual_task_idx == app.selected_task_idx;
            let is_archived = task.archived.unwrap_or(false);
            let is_first_visible = display_idx == 0;

            let lines = build_task_card_lines(
                app,
                task,
                is_task_selected,
                is_archived,
                max_width,
                is_first_visible,
                has_tasks_above,
            );

            ListItem::new(lines)
        })
        .collect();

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

    f.render_widget(tasks_list, area);
}
