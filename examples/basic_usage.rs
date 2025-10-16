use yougile_client::{YouGileClient, YougileError};

// This example requires tokio runtime
// Add this to your Cargo.toml:
// [dependencies]
// tokio = { version = "1.0", features = ["full"] }

#[tokio::main]
async fn main() -> Result<(), YougileError> {
    // Initialize logging
    env_logger::init();
    
    // Create a new client with a bearer token
    let config = yougile_client::apis::configuration::Configuration::new(
        "your-api-token-here".to_string()
    )
    .with_base_path("https://yougile.com");

    let client = YouGileClient::new(config);

    // Example: Get company information
    log::info!("Getting company information...");
    match client.get_company().await {
        Ok(company) => {
            log::info!("Company: {}", company.title);
        }
        Err(e) => {
            log::error!("Error getting company: {:?}", e);
        }
    }

    // Example: Search for projects
    log::info!("Searching for projects...");
    match client
        .search_projects(None, Some(10.0), Some(0.0), None)
        .await
    {
        Ok(projects) => {
            log::info!("Found {} projects", projects.content.len());
            for project in projects.content.iter().take(3) {
                log::info!("  - {}: {}", project.id, project.title);
            }
        }
        Err(e) => {
            log::error!("Error searching projects: {:?}", e);
        }
    }

    // Example: Search for users
    log::info!("Searching for users...");
    match client.search_users(Some(10.0), Some(0.0), None, None).await {
        Ok(users) => {
            log::info!("Found {} users", users.content.len());
            for user in users.content.iter().take(3) {
                log::info!("  - {}: {} ({})", user.id, user.real_name, user.email);
            }
        }
        Err(e) => {
            log::error!("Error searching users: {:?}", e);
        }
    }

    Ok(())
}