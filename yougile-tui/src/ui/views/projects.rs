use crate::app::App;
use crate::ui::widgets::{draw_error_popup, draw_loading_popup};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn draw_projects_view(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(f.area());

    let header = Paragraph::new("YouGile TUI - Projects").style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(header, chunks[0]);

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

    let footer_text = "↵: open | ?: help | ↑/↓ or j/k: navigate | r: refresh | q: quit";
    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);

    if app.loading {
        draw_loading_popup(f);
    }

    if let Some(ref error) = app.error {
        draw_error_popup(f, error);
    }
}
