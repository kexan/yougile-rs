/// Truncate string to max_chars characters (not bytes!)
pub fn truncate_str(s: &str, max_chars: usize) -> String {
    s.chars().take(max_chars).collect()
}

/// Extract initials from a name: "Сергей Никитин" -> "СН", "Admin" -> "AD"
pub fn get_initials(name: &str) -> String {
    let words: Vec<&str> = name.split_whitespace().collect();

    if words.len() >= 2 {
        let first = words[0].chars().next().unwrap_or(' ');
        let second = words[1].chars().next().unwrap_or(' ');
        format!("{}{}", first, second).to_uppercase()
    } else if words.len() == 1 {
        let chars: Vec<char> = words[0].chars().collect();
        if chars.len() >= 2 {
            format!("{}{}", chars[0], chars[1]).to_uppercase()
        } else if chars.len() == 1 {
            format!("{} ", chars[0]).to_uppercase()
        } else {
            "??".to_string()
        }
    } else {
        "??".to_string()
    }
}

/// Wrap text into lines of max_width characters
pub fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        let word_len = word.chars().count();
        let current_len = current_line.chars().count();

        if current_len == 0 {
            if word_len > max_width {
                let chars: Vec<char> = word.chars().collect();
                for chunk in chars.chunks(max_width) {
                    lines.push(chunk.iter().collect());
                }
            } else {
                current_line = word.to_string();
            }
        } else if current_len + 1 + word_len <= max_width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
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

use std::collections::HashMap;
use yougile_api_client::models::StickerValue;

/// Check if stickers value is non-empty
pub fn has_stickers(stickers: Option<&HashMap<String, StickerValue>>) -> bool {
    stickers
        .is_some_and(|map| !map.is_empty())
}
