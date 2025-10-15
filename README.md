# YouGile Client

A Rust client for the YouGile API v2.0. Provides a more idiomatic Rust interface for interacting with YouGile services.

## Features

- **Idiomatic Rust API**: Uses Rust conventions like `Result<T, E>` and async/await
- **Type-safe**: Strong typing throughout the API
- **Easy configuration**: Simple client setup with bearer token authentication
- **Comprehensive coverage**: Includes all YouGile API endpoints

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
yougile-client = { path = "path/to/yougile-client" }  # Replace with actual path or version
```

## Usage

```rust
use yougile_client::{YouGileClient, YouGileError};

#[tokio::main]
async fn main() -> Result<(), YouGileError> {
    // Create a new client with a bearer token
    let config = yougile_client::apis::configuration::Configuration::new()
        .with_base_path("https://yougile.com")
        .with_bearer_token("your-api-token-here");
    
    let client = YouGileClient::new(config);

    // Example: Get company information
    match client.get_company().await {
        Ok(company) => {
            println!("Company: {}", company.title);
        }
        Err(e) => {
            eprintln!("Error getting company: {:?}", e);
        }
    }

    // Example: Search for projects
    match client.search_projects(None, Some(10.0), Some(0.0), None).await {
        Ok(projects) => {
            println!("Found {} projects", projects.content.len());
        }
        Err(e) => {
            eprintln!("Error searching projects: {:?}", e);
        }
    }

    Ok(())
}
```

## API Coverage

The client includes methods for:

- Authentication (API keys, company management)
- Boards and columns
- Tasks and task management
- Users and user management
- Projects and project roles
- Chats and messages
- Departments
- Stickers (string and sprint)
- Group chats
- Webhooks
- File uploads

## License

This project is licensed under the Unlicense license.