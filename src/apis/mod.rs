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

use crate::YougileError;
use crate::apis::configuration::Configuration;

use log::{debug, error, trace};
use std::fmt::Debug;

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

pub async fn parse_response<T: DeserializeOwned + Debug>(
    resp: Response,
) -> Result<T, YougileError> {
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
        debug!(
            "Received JSON response with length: {} bytes",
            content.len()
        );
        // Log actual JSON content at DEBUG level for easier debugging
        debug!("JSON response: {}", content);
        trace!("JSON response content: {}", content);
    }

    if status.is_success() {
        if content_type.contains("application/json") {
            match serde_json::from_str::<T>(&content) {
                Ok(parsed) => {
                    debug!("Successfully parsed JSON response");
                    trace!("Parsed response: {:?}", parsed);
                    Ok(parsed)
                }
                Err(e) => {
                    error!("Failed to parse JSON: {}", e);
                    error!("JSON content: {}", content);
                    Err(YougileError::Serde(e))
                }
            }
        } else {
            Err(YougileError::UnsupportedContentType(content_type))
        }
    } else {
        error!(
            "API request failed with status: {} and content: {}",
            status, content
        );
        Err(YougileError::ApiError { status, content })
    }
}
