use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw_help_view(f: &mut Frame, _app: &App) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
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
        Line::from(
            "Columns: Color-coded by API (1-16 hex colors) • Count shows non-archived tasks only",
        ),
        Line::from(
            "Tasks: Card borders colored by task-* • Stickers shown in cyan boxes • Archived dimmed",
        ),
        Line::from("Stickers: [name:state] for state-based • [name:text] for free-text values"),
        Line::from(
            "Card indicators: ✓ (top-left, green=done, gray=pending) • [XX] (bottom-right, assignee initials)",
        ),
    ];

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded);

    let help_widget = Paragraph::new(help_text).block(block);
    f.render_widget(help_widget, chunks[0]);
}
