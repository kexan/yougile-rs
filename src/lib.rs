#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;

// Initialize logging
#[cfg(feature = "logging")]
pub fn init_logging() {
    use std::env;
    env_logger::init_from_env(env_logger::Env::default().filter_or("YOUGILE_LOG_LEVEL", "info"));
}

#[cfg(not(feature = "logging"))]
pub fn init_logging() {
    // Logging disabled when feature is not enabled
}

pub mod apis;
pub mod client;
pub mod error;
pub mod models;

// Re-export API functions for convenience
pub use apis::auth::*;
pub use apis::boards::*;
pub use apis::chats::*;
pub use apis::columns::*;
pub use apis::departments::*;
pub use apis::files::*;
pub use apis::group_chats::*;
pub use apis::projects::*;
pub use apis::stickers::*;
pub use apis::tasks::*;
pub use apis::users::*;
pub use apis::webhooks::*;

// Re-export the client and error types
pub use client::YouGileClient;
pub use error::YougileError;

pub use models::stickers::*;
