use crate::app::App;
use crate::ui::widgets::{draw_error_popup, draw_loading_popup};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw_boards_view(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(f.area());

    let header_text = if let Some(ref project) = app.current_project {
        format!("YouGile TUI - {} / Boards", project.title)
    } else {
        "YouGile TUI - Boards".to_string()
    };
    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

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

    let footer_text = "↵: open | ?: help | ↑/↓ or j/k: navigate | r: refresh | Esc: back | q: quit";
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);

    if app.loading {
        draw_loading_popup(f);
    }

    if let Some(ref error) = app.error {
        draw_error_popup(f, error);
    }
}
