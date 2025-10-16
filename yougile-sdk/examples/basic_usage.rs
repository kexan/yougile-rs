use yougile_sdk::YouGileSDK;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Create a new client using the builder pattern
    let client = YouGileSDK::builder()
        .token("your-api-token-here") // Replace with your actual token
        .base_url("https://yougile.com")
        .build()?;

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

    // Example: Using the tasks API with fluent search
    log::info!("Searching for tasks...");
    match client.tasks().search().limit(10.0).execute().await {
        Ok(tasks) => {
            log::info!("Found {} tasks", tasks.content.len());
            for task in tasks.content.iter().take(3) {
                log::info!("  - {}: {}", task.id, task.title);
            }
        }
        Err(e) => {
            log::error!("Error searching tasks: {:?}", e);
        }
    }

    // Example: Using the projects API with fluent search
    log::info!("Searching for projects...");
    match client
        .projects()
        .search()
        .title("important")
        .limit(5.0)
        .execute()
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

    // Example: Using the users API with fluent search
    log::info!("Searching for users...");
    match client.users().search().limit(10.0).execute().await {
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

    // Example: Using the boards API with fluent search
    log::info!("Searching for boards...");
    match client.boards().search().limit(10.0).execute().await {
        Ok(boards) => {
            log::info!("Found {} boards", boards.content.len());
            for board in boards.content.iter().take(3) {
                log::info!("  - {}: {}", board.id, board.title);
            }
        }
        Err(e) => {
            log::error!("Error searching boards: {:?}", e);
        }
    }

    Ok(())
}

