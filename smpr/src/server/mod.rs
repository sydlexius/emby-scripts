// Server module methods are consumed by future milestones (detection, rating).
#![allow(dead_code, unused_imports)]

pub mod error;
pub mod types;

#[cfg(test)]
mod tests;

pub use error::MediaServerError;
pub use types::SystemInfoPublic;

use crate::config::ServerType;
use serde_json::Value;
use std::cell::OnceCell;
use std::time::Duration;

/// HTTP client for Emby/Jellyfin media server APIs.
pub struct MediaServerClient {
    base_url: String,
    api_key: String,
    server_type: ServerType,
    agent: ureq::Agent,
    user_id: OnceCell<String>,
}

impl MediaServerClient {
    /// Create a new client. `server_type` must be resolved before construction
    /// (via `detect_server_type` or TOML override).
    pub fn new(base_url: String, api_key: String, server_type: ServerType) -> Self {
        let agent = ureq::Agent::config_builder()
            .timeout_per_call(Some(Duration::from_secs(15)))
            .http_status_as_error(false)
            .build()
            .new_agent();
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
            server_type,
            agent,
            user_id: OnceCell::new(),
        }
    }

    /// Returns the base URL (trailing slash stripped).
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns the (header_name, header_value) pair for authentication.
    pub fn auth_header(&self) -> (&str, &str) {
        match self.server_type {
            ServerType::Emby => ("X-Emby-Token", &self.api_key),
            ServerType::Jellyfin => ("X-MediaBrowser-Token", &self.api_key),
        }
    }

    /// Returns the server type.
    pub fn server_type(&self) -> &ServerType {
        &self.server_type
    }

    /// Authenticated JSON request. Returns `Ok(None)` when response body is empty.
    pub fn request(
        &self,
        method: &str,
        path: &str,
        body: Option<&Value>,
    ) -> Result<Option<Value>, MediaServerError> {
        let url = format!("{}{}", self.base_url, path);
        let (auth_name, auth_value) = self.auth_header();

        let response = match method {
            "GET" => self
                .agent
                .get(&url)
                .header(auth_name, auth_value)
                .header("Accept", "application/json")
                .call()?,
            "POST" => {
                let req = self
                    .agent
                    .post(&url)
                    .header(auth_name, auth_value)
                    .header("Accept", "application/json");
                match body {
                    Some(b) => req.send_json(b)?,
                    None => req
                        .header("Content-Type", "application/json")
                        .send_empty()?,
                }
            }
            _ => {
                return Err(MediaServerError::Protocol(format!(
                    "unsupported method: {method}"
                )));
            }
        };

        let status = response.status().as_u16();
        if status >= 400 {
            let body_snippet = response
                .into_body()
                .read_to_string()
                .unwrap_or_default();
            let snippet = if body_snippet.len() > 1024 {
                format!("{}...", &body_snippet[..1024])
            } else {
                body_snippet
            };
            return Err(MediaServerError::Http {
                status,
                body: snippet,
            });
        }

        // Read body — empty body returns None
        let body_str = response
            .into_body()
            .read_to_string()
            .unwrap_or_default();
        if body_str.trim().is_empty() {
            return Ok(None);
        }
        let value: Value = serde_json::from_str(&body_str).map_err(|e| {
            MediaServerError::Parse(format!(
                "non-JSON response on {method} {path}: {e}"
            ))
        })?;
        Ok(Some(value))
    }

    /// Authenticated plain-text request. Returns raw response body.
    pub fn request_text(
        &self,
        method: &str,
        path: &str,
    ) -> Result<String, MediaServerError> {
        let url = format!("{}{}", self.base_url, path);
        let (auth_name, auth_value) = self.auth_header();

        let response = match method {
            "GET" => self
                .agent
                .get(&url)
                .header(auth_name, auth_value)
                .call()?,
            _ => {
                return Err(MediaServerError::Protocol(format!(
                    "unsupported method for request_text: {method}"
                )));
            }
        };

        let status = response.status().as_u16();
        if status >= 400 {
            let body_snippet = response
                .into_body()
                .read_to_string()
                .unwrap_or_default();
            let snippet = if body_snippet.len() > 1024 {
                format!("{}...", &body_snippet[..1024])
            } else {
                body_snippet
            };
            return Err(MediaServerError::Http {
                status,
                body: snippet,
            });
        }

        response
            .into_body()
            .read_to_string()
            .map_err(|e| MediaServerError::Connection(format!("read error: {e}")))
    }
}
