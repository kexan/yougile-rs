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
    println!("Getting company information...");
    match client.get_company().await {
        Ok(company) => {
            println!("Company: {}", company.title);
        }
        Err(e) => {
            eprintln!("Error getting company: {:?}", e);
        }
    }

    // Example: Using the tasks API with fluent search
    println!("Searching for tasks...");
    match client.tasks().search().limit(10.0).execute().await {
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

    // Example: Using the projects API with fluent search
    println!("Searching for projects...");
    match client.projects().search().title("important").limit(5.0).execute().await {
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

    // Example: Using the users API with fluent search
    println!("Searching for users...");
    match client.users().search().limit(10.0).execute().await {
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

    // Example: Using the boards API with fluent search
    println!("Searching for boards...");
    match client.boards().search().limit(10.0).execute().await {
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

    Ok(())
}