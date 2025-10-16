use yougile_sdk::{SDKError, YouGileSDK};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Create a new client using the builder pattern
    let client = YouGileSDK::builder()
        .token("dIGhpLEQ38CUh-SjUGDNtz0PxkzcrIn-IESOt47jy6EzD4Nt93rHvdwrsLz37oFF") // Replace with your actual token
        .base_url("https://yougile.kexan.ru")
        .build()?;

    // Example: Complex task search with multiple filters
    log::info!("Searching for important tasks assigned to a specific user...");
    let tasks = client
        .tasks()
        .search()
        .title("important")
        .assigned_to("user-id-here")
        .limit(20.0)
        .execute()
        .await?;

    log::info!("Found {} tasks matching criteria", tasks.content.len());
    for task in tasks.content.iter().take(5) {
        log::info!("  - {}: {}", task.id, task.title);
        if let Some(description) = &task.description {
            log::info!("    Description: {}", description);
        }
    }

    // Example: Creating a new project
    log::info!("Creating a new project...");
    let create_project = yougile_client::models::CreateProject::new("New Project".to_string());
    match client.projects().create(create_project).await {
        Ok(project_id) => {
            log::info!("Created project with ID: {}", project_id.id);

            // Now create a task in that project
            log::info!("Creating a task in the new project...");
            let create_task = yougile_client::models::CreateTask::new("Initial Task".to_string());
            match client.tasks().create(create_task).await {
                Ok(task_id) => {
                    log::info!("Created task with ID: {}", task_id.id);
                }
                Err(e) => {
                    log::error!("Error creating task: {:?}", e);
                }
            }
        }
        Err(e) => {
            log::error!("Error creating project: {:?}", e);
        }
    }

    // Example: Searching for users by email
    log::info!("Searching for users by email...");
    match client
        .users()
        .search()
        .email("example@domain.com")
        .execute()
        .await
    {
        Ok(users) => {
            log::info!("Found {} users with that email", users.content.len());
            for user in users.content.iter() {
                log::info!("  - {}: {} ({})", user.id, user.real_name, user.email);
            }
        }
        Err(e) => {
            log::error!("Error searching users by email: {:?}", e);
        }
    }

    // Example: Searching for boards in a specific project
    log::info!("Searching for boards in a specific project...");
    match client
        .boards()
        .search()
        .project_id("project-id-here")
        .execute()
        .await
    {
        Ok(boards) => {
            log::info!("Found {} boards in project", boards.content.len());
            for board in boards.content.iter() {
                log::info!("  - {}: {}", board.id, board.title);
            }
        }
        Err(e) => {
            log::error!("Error searching boards by project: {:?}", e);
        }
    }

    // Example: Error handling
    log::info!("Demonstrating error handling...");
    match client.tasks().get("non-existent-task-id").await {
        Ok(task) => {
            log::info!("Found task: {}", task.title);
        }
        Err(SDKError::ClientError(yougile_client::YougileError::ApiError { status, content })) => {
            log::error!("API Error - Status: {}, Content: {}", status, content);
        }
        Err(e) => {
            log::error!("Other error occurred: {:?}", e);
        }
    }

    Ok(())
}

