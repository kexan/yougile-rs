pub mod auth;
pub mod boards;
pub mod chats;
pub mod columns;
pub mod configuration;
pub mod departments;
pub mod files;
pub mod group_chats;
pub mod projects;
pub mod stickers;
pub mod tasks;
pub mod users;
pub mod webhooks;

use std::error;
use std::fmt;

use reqwest::RequestBuilder;
use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::apis::configuration::Configuration;
use crate::YougileError;

pub trait RequestBuilderExt {
    fn with_auth_headers(self, cfg: &Configuration) -> Self;
}

impl RequestBuilderExt for RequestBuilder {
    fn with_auth_headers(self, cfg: &Configuration) -> Self {
        self.bearer_auth(&cfg.token)
            .header(reqwest::header::USER_AGENT, &cfg.user_agent)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub async fn parse_response<T: DeserializeOwned>(resp: Response) -> Result<T, YougileError> {
    let status = resp.status();

    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let content = resp.text().await?;

    // Debug logging for JSON responses
    if status.is_success() && content_type.contains("application/json") {
        // Limit debug output to prevent huge logs
        let debug_content = if content.len() > 1000 {
            format!("{}...", &content[..1000])
        } else {
            content.clone()
        };
        eprintln!("DEBUG: Received JSON response (first 1000 chars): {}", debug_content);
    }

    if status.is_success() {
        if content_type.contains("application/json") {
            match serde_json::from_str::<T>(&content) {
                Ok(parsed) => Ok(parsed),
                Err(e) => {
                    eprintln!("DEBUG: Failed to parse JSON: {}", e);
                    eprintln!("DEBUG: JSON content: {}", content);
                    Err(YougileError::Serde(e))
                }
            }
        } else {
            Err(YougileError::UnsupportedContentType(content_type))
        }
    } else {
        Err(YougileError::ApiError { status, content })
    }
}
