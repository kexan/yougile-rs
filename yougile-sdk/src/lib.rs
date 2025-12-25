//! High-level SDK for YouGile API with builder pattern and convenience methods
//!
//! This crate provides a more ergonomic interface to the YouGile API compared to the low-level client.
//! It includes a builder pattern for client configuration, resource-based access patterns,
//! and convenience methods for common operations.
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust,no_run
//! use yougile_sdk::YouGileSDK;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = YouGileSDK::builder()
//!     .token("your-api-token")
//!     .base_url("https://yougile.com")
//!     .build()?;
//!
//! // Get company information
//! let company = client.get_company().await?;
//! log::info!("Company: {}", company.title);
//!
//! // Search for tasks using fluent API
//! let tasks = client.tasks()
//!     .search()
//!     .title("important")
//!     .limit(10.0)
//!     .execute()
//!     .await?;
//!
//! // List projects
//! let projects = client.projects().list().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Working with Resources
//!
//! The SDK provides resource-based access to different parts of the YouGile API:
//!
//! - `client.tasks()` - Access task-related operations
//! - `client.projects()` - Access project-related operations  
//! - `client.users()` - Access user-related operations
//! - `client.boards()` - Access board-related operations
//! - `client.chats()` - Access chat-related operations
//! - `client.group_chats()` - Access group chat-related operations
//! - `client.stickers()` - Access sticker-related operations
//! - `client.webhooks()` - Access webhook-related operations
//! - `client.files()` - Access file-related operations

mod builder;
mod error;
mod resources;

pub use builder::YouGileSDK;
pub use error::SDKError;
pub use yougile_api_client::{YouGileClient, models::*};
