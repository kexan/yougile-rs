use yougile_client::{YouGileClient, YougileError};

#[tokio::main]
async fn main() -> Result<(), YougileError> {
    // Create a new client with a bearer token
    let config =
        yougile_client::apis::configuration::Configuration::new("your-api-token-here".to_string())
            .with_base_path("https://yougile.kexan.ru");

    let client = YouGileClient::new(config);

    println!("Getting company information...");
    match client.get_company().await {
        Ok(company) => {
            println!("Company: {} (ID: {})", company.title, company.id);
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

    println!("\nSearching for boards...");
    match client
        .search_boards(None, Some(10.0), Some(0.0), None, None)
        .await
    {
        Ok(boards) => {
            println!("Found {} boards", boards.content.len());
            for board in boards.content.iter().take(3) {
                println!("  - {}: {}", board.id, board.title);
            }
        }
        Err(e) => {
            eprintln!("Error searching boards: {:?}", e);
        }
    }

    println!("\nSearching for tasks...");
    match client
        .search_tasks(None, Some(10.0), Some(0.0), None, None, None, None, None)
        .await
    {
        Ok(tasks) => {
            println!("Found {} tasks", tasks.content.len());
            for task in tasks.content.iter().take(3) {
                println!("  - {}: {}", task.id, task.title);
            }
        }
        Err(e) => {
            eprintln!("Error searching tasks: {:?}", e);
        }
    }

    // Example of creating a task (uncomment if you want to test)
    /*
    println!("\nCreating a test task...");
    let create_task = yougile_client::models::CreateTask::new("Test Task".to_string());
    match client.create_task(create_task).await {
        Ok(task_id) => {
            println!("Created task with ID: {}", task_id.id);
        }
        Err(e) => {
            eprintln!("Error creating task: {:?}", e);
        }
    }
    */

    Ok(())
}

