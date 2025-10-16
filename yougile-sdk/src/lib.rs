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
//! println!("Company: {}", company.title);
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

mod builder;
mod error;
mod resources;
mod convenience;

pub use builder::YouGileSDK;
pub use error::SDKError;

// Re-export commonly used types
pub use yougile_client::{
    models::{
        // Common models
        Company, Id, User, Task, Project, Board, Column,
        // Request models
        CreateTask, UpdateTask, CreateProject, UpdateProject,
        // Response models
        TaskList, ProjectList, UserList, BoardList, ColumnList,
    },
    // Re-export the low-level client in case users need direct access
    YouGileClient,
};