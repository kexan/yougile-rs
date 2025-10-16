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

    // Example: Authentication and company operations
    log::info!("Getting company information...");
    match client.auth().get_company().await {
        Ok(company) => {
            log::info!("Company: {}", company.title);
        }
        Err(e) => {
            log::error!("Error getting company: {:?}", e);
        }
    }

    // Example: Working with tasks
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

    // Example: Working with projects
    log::info!("Searching for projects...");
    match client.projects().search().title("important").limit(5.0).execute().await {
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

    // Example: Working with users
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

    // Example: Working with boards
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

    // Example: Working with columns
    log::info!("Searching for columns...");
    match client.columns().search(None, Some(10.0), Some(0.0), None, None).await {
        Ok(columns) => {
            log::info!("Found {} columns", columns.content.len());
            for column in columns.content.iter().take(3) {
                log::info!("  - {}: {}", column.id, column.title);
            }
        }
        Err(e) => {
            log::error!("Error searching columns: {:?}", e);
        }
    }

    // Example: Working with group chats
    log::info!("Searching for group chats...");
    match client.group_chats().search(None, Some(10.0), Some(0.0), None).await {
        Ok(group_chats) => {
            log::info!("Found {} group chats", group_chats.content.len());
            for chat in group_chats.content.iter().take(3) {
                log::info!("  - {}: {}", chat.id, chat.title);
            }
        }
        Err(e) => {
            log::error!("Error searching group chats: {:?}", e);
        }
    }

    // Example: Working with stickers (sprint stickers)
    log::info!("Searching for sprint stickers...");
    match client.stickers().search_sprint_stickers(None, Some(10.0), Some(0.0), None, None).await {
        Ok(stickers) => {
            log::info!("Found {} sprint stickers", stickers.content.len());
            for sticker in stickers.content.iter().take(3) {
                log::info!("  - {}: {}", sticker.id, sticker.name);
            }
        }
        Err(e) => {
            log::error!("Error searching sprint stickers: {:?}", e);
        }
    }

    // Example: Working with webhooks
    log::info!("Searching for webhooks...");
    match client.webhooks().search(None).await {
        Ok(_webhooks) => {
            log::info!("Found webhooks");
        }
        Err(e) => {
            log::error!("Error searching webhooks: {:?}", e);
        }
    }

    log::info!("All API examples completed successfully!");
    Ok(())
}