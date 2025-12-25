use crate::app::App;
use crate::ui::{has_stickers, truncate_str};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw_task_detail_panel(f: &mut Frame, app: &App, area: Rect) {
    if let Some(ref task) = app.current_task {
        let mut details = vec![];
        
        details.push(Line::from(vec![Span::styled(
            &task.title,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )]));
        details.push(Line::from(""));
        
        if let Some(ref id_common) = task.id_task_common {
            details.push(Line::from(vec![
                Span::styled("ID: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(id_common.clone()),
            ]));
        }
        
        details.push(Line::from(""));

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

        if let Some(ref stickers) = task.stickers {
            if has_stickers(Some(stickers)) {
                details.push(Line::from(""));
                details.push(Line::from(vec![
                    Span::styled("Stickers:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]));
                
                if let Some(obj) = stickers.as_object() {
                    for (sticker_id, state_value) in obj.iter() {
                        let sticker_meta = app.stickers.get(sticker_id);
                        
                        let (display_title, value_str) = if let Some(meta) = sticker_meta {
                            let value = match state_value {
                                serde_json::Value::String(state_id_or_text) => {
                                    meta.states
                                        .get(state_id_or_text)
                                        .cloned()
                                        .unwrap_or_else(|| state_id_or_text.clone())
                                }
                                serde_json::Value::Number(n) => n.to_string(),
                                serde_json::Value::Bool(b) => b.to_string(),
                                _ => "[unknown]".to_string(),
                            };
                            (meta.title.clone(), value)
                        } else {
                            let value = match state_value {
                                serde_json::Value::String(s) => s.clone(),
                                serde_json::Value::Number(n) => n.to_string(),
                                serde_json::Value::Bool(b) => b.to_string(),
                                _ => "[unknown]".to_string(),
                            };
                            (format!("Unknown ({})", truncate_str(sticker_id, 8)), value)
                        };
                        
                        details.push(Line::from(vec![
                            Span::raw("  • "),
                            Span::styled(format!("{}: ", display_title), Style::default().fg(Color::Cyan)),
                            Span::raw(value_str),
                        ]));
                    }
                }
            }
        }

        if let Some(ref assigned) = task.assigned {
            if !assigned.is_empty() {
                details.push(Line::from(""));
                let assignee_names: Vec<String> = assigned
                    .iter()
                    .map(|user_id| app.get_user_name(user_id))
                    .collect();
                
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
