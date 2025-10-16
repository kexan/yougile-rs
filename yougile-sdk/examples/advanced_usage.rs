use yougile_sdk::{YouGileSDK, SDKError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Create a new client using the builder pattern
    let client = YouGileSDK::builder()
        .token("your-api-token-here") // Replace with your actual token
        .base_url("https://yougile.com")
        .build()?;

    // Example: Complex task search with multiple filters
    println!("Searching for important tasks assigned to a specific user...");
    let tasks = client.tasks()
        .search()
        .title("important")
        .assigned_to("user-id-here")
        .limit(20.0)
        .execute()
        .await?;
    
    println!("Found {} tasks matching criteria", tasks.content.len());
    for task in tasks.content.iter().take(5) {
        println!("  - {}: {}", task.id, task.title);
        if let Some(description) = &task.description {
            println!("    Description: {}", description);
        }
    }

    // Example: Creating a new project
    println!("\nCreating a new project...");
    let create_project = yougile_client::models::CreateProject::new("New Project".to_string());
    match client.projects().create(create_project).await {
        Ok(project_id) => {
            println!("Created project with ID: {}", project_id.id);
            
            // Now create a task in that project
            println!("Creating a task in the new project...");
            let create_task = yougile_client::models::CreateTask::new("Initial Task".to_string());
            match client.tasks().create(create_task).await {
                Ok(task_id) => {
                    println!("Created task with ID: {}", task_id.id);
                }
                Err(e) => {
                    eprintln!("Error creating task: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error creating project: {:?}", e);
        }
    }

    // Example: Searching for users by email
    println!("\nSearching for users by email...");
    match client.users().search().email("example@domain.com").execute().await {
        Ok(users) => {
            println!("Found {} users with that email", users.content.len());
            for user in users.content.iter() {
                println!("  - {}: {} ({})", user.id, user.real_name, user.email);
            }
        }
        Err(e) => {
            eprintln!("Error searching users by email: {:?}", e);
        }
    }

    // Example: Searching for boards in a specific project
    println!("\nSearching for boards in a specific project...");
    match client.boards().search().project_id("project-id-here").execute().await {
        Ok(boards) => {
            println!("Found {} boards in project", boards.content.len());
            for board in boards.content.iter() {
                println!("  - {}: {}", board.id, board.title);
            }
        }
        Err(e) => {
            eprintln!("Error searching boards by project: {:?}", e);
        }
    }

    // Example: Error handling
    println!("\nDemonstrating error handling...");
    match client.tasks().get("non-existent-task-id").await {
        Ok(task) => {
            println!("Found task: {}", task.title);
        }
        Err(SDKError::ClientError(yougile_client::YougileError::ApiError { status, content })) => {
            eprintln!("API Error - Status: {}, Content: {}", status, content);
        }
        Err(e) => {
            eprintln!("Other error occurred: {:?}", e);
        }
    }

    Ok(())
}