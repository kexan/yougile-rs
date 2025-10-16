use yougile_client::{YouGileClient, YougileError};

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

    log::info!("Getting company information...");
    match client.get_company().await {
        Ok(company) => {
            log::info!("Company: {} (ID: {})", company.title, company.id);
        }
        Err(e) => {
            log::error!("Error getting company: {:?}", e);
        }
    }

    log::info!("Searching for projects...");
    match client.search_projects(None, Some(10.0), Some(0.0), None).await {
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

    log::info!("Searching for boards...");
    match client.search_boards(None, Some(10.0), Some(0.0), None, None).await {
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

    log::info!("Searching for tasks...");
    match client.search_tasks(None, Some(10.0), Some(0.0), None, None, None, None, None).await {
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

    // Example of creating a task (uncomment if you want to test)
    /*
    log::info!("Creating a test task...");
    let create_task = yougile_client::models::CreateTask::new("Test Task".to_string());
    match client.create_task(create_task).await {
        Ok(task_id) => {
            log::info!("Created task with ID: {}", task_id.id);
        }
        Err(e) => {
            log::error!("Error creating task: {:?}", e);
        }
    }
    */

    Ok(())
}