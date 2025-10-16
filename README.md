//! # YouGile Client Library
//! 
//! A Rust client for the YouGile API v2.0. Provides a more idiomatic Rust interface for 
//! interacting with YouGile services.
//! 
//! ## Quick Start
//! 
//! ```rust,no_run
//! use yougile_client::{YouGileClient, YougileError};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), YougileError> {
//!     // Initialize logging (optional but recommended)
//!     env_logger::init();
//!     
//!     // Create a new client with your API token
//!     let config = yougile_client::apis::configuration::Configuration::new(
//!         "your-api-token-here".to_string()
//!     )
//!     .with_base_path("https://yougile.com");
//! 
//!     let client = YouGileClient::new(config);
//! 
//!     // Get company information
//!     match client.get_company().await {
//!         Ok(company) => {
//!             log::info!("Company: {}", company.title);
//!         }
//!         Err(e) => {
//!             log::error!("Error getting company: {:?}", e);
//!         }
//!     }
//! 
//!     Ok(())
//! }
//! ```
//! 
//! ## Features
//! 
//! ### Logging
//! 
//! The library supports configurable logging through the `log` crate. To enable logging:
//! 
//! 1. Add `env_logger` to your dependencies:
//!    ```toml
//!    [dependencies]
//!    env_logger = "0.11"
//!    ```
//! 
//! 2. Initialize logging in your application:
//!    ```rust
//!    use yougile_client::YouGileClient;
//!    
//!    #[tokio::main]
//!    async fn main() -> Result<(), yougile_client::YougileError> {
//!        // Initialize logging
//!        env_logger::init();
//!        
//!        // Your code here...
//!        Ok(())
//!    }
//!    ```
//! 
//! 3. Control log levels with environment variables:
//!    ```bash
//!    # Show only errors and warnings
//!    RUST_LOG=warn cargo run
//!    
//!    # Show detailed debug information
//!    RUST_LOG=debug cargo run
//!    
//!    # Show only YouGile client logs
//!    RUST_LOG=yougile_client=trace cargo run
//!    ```
//! 
//! ### TLS Backends
//! 
//! The library supports two TLS backends:
//! 
//! - `native-tls` (default) - Uses the system's native TLS implementation
//! - `rustls-tls` - Uses the pure-Rust rustls implementation
//! 
//! To switch TLS backends:
//! 
//! ```toml
//! [dependencies.yougile-client]
//! version = "2.0"
//! default-features = false
//! features = ["rustls-tls"]  # or ["native-tls"]
//! ```
//! 
//! ## Examples
//! 
//! See the `examples/` directory for complete working examples:
//! 
//! - `basic_usage.rs` - Basic API usage
//! - `advanced_usage.rs` - Advanced API usage with multiple endpoints
//! - `file_upload.rs` - File upload functionality
//! 
//! Run examples with:
//! 
//! ```bash
//! cargo run --example basic_usage
//! ```