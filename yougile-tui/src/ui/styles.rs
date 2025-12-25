use ratatui::style::Color;

/// Map column color index (1-16) to ratatui Color using exact hex codes from API docs
pub fn get_column_color(color_index: Option<f64>) -> Color {
    match color_index.map(|c| c as u8) {
        Some(1) => Color::Rgb(0x7B, 0x86, 0x9E),
        Some(2) => Color::Rgb(0xFF, 0x8C, 0x8C),
        Some(3) => Color::Rgb(0xE9, 0xA2, 0x4F),
        Some(4) => Color::Rgb(0xFC, 0xE2, 0x58),
        Some(5) => Color::Rgb(0x7C, 0xAE, 0x5E),
        Some(6) => Color::Rgb(0x49, 0xC5, 0xBC),
        Some(7) => Color::Rgb(0x8C, 0xAC, 0xFF),
        Some(8) => Color::Rgb(0xCC, 0x8C, 0xFF),
        Some(9) => Color::Rgb(0x66, 0x70, 0x85),
        Some(10) => Color::Rgb(0xEB, 0x37, 0x37),
        Some(11) => Color::Rgb(0xF2, 0x73, 0x2B),
        Some(12) => Color::Rgb(0xF5, 0xCC, 0x00),
        Some(13) => Color::Rgb(0x5C, 0xDC, 0x11),
        Some(14) => Color::Rgb(0x08, 0xA7, 0xA9),
        Some(15) => Color::Rgb(0x50, 0x89, 0xF2),
        Some(16) => Color::Rgb(0xE2, 0x5E, 0xF2),
        _ => Color::White,
    }
}

/// Map task color string to ratatui Color
pub fn get_task_color(color: Option<&String>) -> Option<Color> {
    match color.map(|s| s.as_str()) {
        Some("task-primary") => Some(Color::White),
        Some("task-gray") => Some(Color::Gray),
        Some("task-red") => Some(Color::Red),
        Some("task-pink") => Some(Color::Magenta),
        Some("task-yellow") => Some(Color::Yellow),
        Some("task-green") => Some(Color::Green),
        Some("task-turquoise") => Some(Color::Cyan),
        Some("task-blue") => Some(Color::Blue),
        Some("task-violet") => Some(Color::Rgb(138, 43, 226)),
        _ => None,
    }
}
