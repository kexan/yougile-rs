use yougile_client::{YouGileClient, YougileError};

#[tokio::main]
async fn main() -> Result<(), YougileError> {
    // Create a new client with a bearer token
    let config = yougile_client::apis::configuration::Configuration::new()
        .with_base_path("https://yougile.com")
        .with_bearer_token("your-api-token-here");

    let client = YouGileClient::new(config);

    println!("Getting company information...");
    match client.get_company().await {
        Ok(company) => {
            println!("Company: {}", company.title);
            println!("ID: {}", company.id);
        }
        Err(e) => {
            eprintln!("Error getting company: {:?}", e);
        }
    }

    println!("\nSearching for projects...");
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

    println!("\nSearching for users...");
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

