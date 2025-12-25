use crate::app::{App, View};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    match app.current_view {
        View::Projects => draw_projects_view(f, app),
        View::Tasks => draw_tasks_view(f, app),
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
    let header = Paragraph::new("YouGile TUI - Project Manager")
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
    let footer_text = "q: quit | h: help | ↑/↓ or j/k: navigate | r: refresh | Tab: switch focus";
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

fn draw_tasks_view(f: &mut Frame, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)])
        .split(f.area());

    let block = Block::default()
        .title(" Tasks ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded);

    let content = Paragraph::new("Task list view - Coming soon!")
        .block(block)
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(content, chunks[0]);
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
        Line::from("  j/↓   Move down"),
        Line::from("  k/↑   Move up"),
        Line::from("  Tab   Switch focus between panels"),
        Line::from(""),
        Line::from("Actions:"),
        Line::from("  Enter Select item"),
        Line::from("  r     Refresh data"),
        Line::from(""),
        Line::from("Views:"),
        Line::from("  p     Projects view"),
        Line::from("  h     Help view"),
        Line::from(""),
        Line::from("General:"),
        Line::from("  q     Quit"),
        Line::from("  Esc   Quit"),
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
        .title(" Error ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Red));

    let content = Paragraph::new(error).block(block);
    f.render_widget(content, popup_rect);
}
