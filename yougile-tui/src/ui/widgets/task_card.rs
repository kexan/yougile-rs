use crate::app::App;
use crate::ui::{get_initials, get_task_color, truncate_str, wrap_text};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub fn calculate_card_height(
    app: &App,
    task: &yougile_client::models::Task,
    max_width: usize,
) -> usize {
    let wrapped = wrap_text(&task.title, max_width);
    let title_lines = wrapped.len();
    let sticker_lines = count_sticker_lines(app, task.stickers.as_ref());
    3 + title_lines + sticker_lines
}

fn count_sticker_lines(_app: &App, stickers: Option<&serde_json::Value>) -> usize {
    stickers
        .and_then(|v| v.as_object())
        .map(|obj| obj.len())
        .unwrap_or(0)
}

pub fn format_single_sticker(
    app: &App,
    sticker_id: &str,
    state_value: &serde_json::Value,
    max_width: usize,
) -> String {
    let sticker_meta = app.stickers.get(sticker_id);

    let display = if let Some(meta) = sticker_meta {
        match state_value {
            serde_json::Value::String(state_id_or_text) => {
                let value_display = meta
                    .states
                    .get(state_id_or_text)
                    .map(|s| s.as_str())
                    .unwrap_or(state_id_or_text);

                format!(
                    "[{}:{}]",
                    truncate_str(&meta.title, 15),
                    truncate_str(value_display, 15)
                )
            }
            serde_json::Value::Number(n) => {
                format!("[{}:{}]", truncate_str(&meta.title, 15), n)
            }
            serde_json::Value::Bool(b) if *b => {
                format!("[✓{}]", truncate_str(&meta.title, 15))
            }
            _ => return String::new(),
        }
    } else {
        match state_value {
            serde_json::Value::String(s) => format!("[{}]", truncate_str(s, 20)),
            serde_json::Value::Number(n) => format!("[№{}]", n),
            serde_json::Value::Bool(b) if *b => "[✓]".to_string(),
            _ => return String::new(),
        }
    };

    truncate_str(&display, max_width)
}

pub fn build_task_card_lines(
    app: &App,
    task: &yougile_client::models::Task,
    is_selected: bool,
    is_archived: bool,
    max_width: usize,
    is_first_visible: bool,
    has_tasks_above: bool,
) -> Vec<Line<'static>> {
    let task_color = get_task_color(task.color.as_ref());
    let is_completed = task.completed.unwrap_or(false);

    let assignee_initials = if let Some(ref assigned) = task.assigned {
        if !assigned.is_empty() {
            let first_assignee = &assigned[0];
            let name = app.get_user_name(first_assignee);
            get_initials(&name)
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let wrapped_lines = wrap_text(&task.title, max_width);
    let mut lines = vec![];

    // Top border with checkmark
    let completion_icon = "✓";
    let completion_color = if is_completed {
        Color::Green
    } else {
        Color::DarkGray
    };

    let top_border_base = if is_first_visible && has_tasks_above {
        let half_width = max_width / 2;
        let left_part = "─".repeat(half_width.saturating_sub(1));
        let right_part = "─".repeat(max_width - half_width);
        format!("┌{}▲{}┐", left_part, right_part)
    } else {
        let inner_width = max_width + 1;
        format!("┌{}┐", "─".repeat(inner_width))
    };

    if let Some(color) = task_color {
        lines.push(Line::from(vec![
            Span::styled("┌", Style::default().fg(color)),
            Span::styled(completion_icon, Style::default().fg(completion_color)),
            Span::styled(top_border_base[3..].to_string(), Style::default().fg(color)),
        ]));
    } else {
        lines.push(Line::from(vec![
            Span::raw("┌"),
            Span::styled(completion_icon, Style::default().fg(completion_color)),
            Span::raw(top_border_base[3..].to_string()),
        ]));
    }

    // Title lines
    for line_text in wrapped_lines {
        let padded = format!(" {} ", line_text);
        let padding_right = (max_width + 2).saturating_sub(padded.chars().count());

        let content_style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else if is_archived {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default().fg(Color::White)
        };

        if let Some(color) = task_color {
            lines.push(Line::from(vec![
                Span::styled("│", Style::default().fg(color)),
                Span::styled(
                    format!("{}{}", padded, " ".repeat(padding_right)),
                    content_style,
                ),
                Span::styled("│", Style::default().fg(color)),
            ]));
        } else {
            let card_line = format!("│{}{}│", padded, " ".repeat(padding_right));
            lines.push(Line::from(Span::styled(card_line, content_style)));
        }
    }

    // Stickers
    if let Some(obj) = task.stickers.as_ref().and_then(|s| s.as_object()) {
        for (sticker_id, state_value) in obj.iter() {
            let sticker_text = format_single_sticker(app, sticker_id, state_value, max_width);
            if !sticker_text.is_empty() {
                let padded = format!(" {} ", sticker_text);
                let padding_right = (max_width + 2).saturating_sub(padded.chars().count());
                let sticker_style = Style::default().fg(Color::Cyan);

                if let Some(color) = task_color {
                    lines.push(Line::from(vec![
                        Span::styled("│", Style::default().fg(color)),
                        Span::styled(
                            format!("{}{}", padded, " ".repeat(padding_right)),
                            sticker_style,
                        ),
                        Span::styled("│", Style::default().fg(color)),
                    ]));
                } else {
                    lines.push(Line::from(vec![
                        Span::raw("│"),
                        Span::styled(
                            format!("{}{}", padded, " ".repeat(padding_right)),
                            sticker_style,
                        ),
                        Span::raw("│"),
                    ]));
                }
            }
        }
    }

    // Bottom border with assignee
    let bottom_border_plain = "─".repeat(max_width + 2);
    let assignee_box = if !assignee_initials.is_empty() {
        format!("[{}]", assignee_initials)
    } else {
        String::new()
    };

    let assignee_len = assignee_box.chars().count();

    if let Some(color) = task_color {
        if assignee_len > 0 {
            let left_len = (max_width + 2).saturating_sub(assignee_len);
            lines.push(Line::from(vec![
                Span::styled(
                    format!("└{}", "─".repeat(left_len)),
                    Style::default().fg(color),
                ),
                Span::styled(assignee_box.clone(), Style::default().fg(Color::Blue)),
                Span::styled("┘", Style::default().fg(color)),
            ]));
        } else {
            lines.push(Line::from(Span::styled(
                format!("└{}┘", bottom_border_plain),
                Style::default().fg(color),
            )));
        }
    } else if assignee_len > 0 {
        let left_len = (max_width + 2).saturating_sub(assignee_len);
        lines.push(Line::from(vec![
            Span::raw(format!("└{}", "─".repeat(left_len))),
            Span::styled(assignee_box.clone(), Style::default().fg(Color::Blue)),
            Span::raw("┘"),
        ]));
    } else {
        lines.push(Line::from(format!("└{}┘", bottom_border_plain)));
    }

    lines
}
