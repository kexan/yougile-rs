use yougile_client::{YouGileClient, YougileError};

// This example requires tokio runtime
// Add this to your Cargo.toml:
// [dependencies]
// tokio = { version = "1.0", features = [\"full\"] }

#[tokio::main]
async fn main() -> Result<(), YougileError> {
    // Create a new client with a bearer token
    let config =
        yougile_client::apis::configuration::Configuration::new("your-api-token-here".to_string())
            .with_base_path("https://yougile.com");

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
    match client
        .search_projects(None, Some(10.0), Some(0.0), None)
        .await
    {
        Ok(projects) => {
            println!("Found {} projects", projects.content.len());
            for project in projects.content.iter().take(3) {
                println!("  - {}: {}", project.id, project.title);
            }
        }
        Err(e) => {
            eprintln!("Error searching projects: {:?}", e);
        }
    }

    // Example: Search for users
    match client.search_users(Some(10.0), Some(0.0), None, None).await {
        Ok(users) => {
            println!("Found {} users", users.content.len());
            for user in users.content.iter().take(3) {
                println!("  - {}: {} ({})", user.id, user.real_name, user.email);
            }
        }
        Err(e) => {
            eprintln!("Error searching users: {:?}", e);
        }
    }

    Ok(())
}

