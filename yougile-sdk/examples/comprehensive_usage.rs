use log::{error, info};
use yougile_sdk::YouGileSDK;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Create a new client using the builder pattern
    let client = YouGileSDK::builder()
        .token("JI0NR9RZJg64alRGKciFVcpItkY2RHtzdoJd2qPQdMdu8hARd0pFzX2bL+Cgz7Xn") // Replace with your actual token
        .base_url("https://ru.yougile.com")
        .build()?;

    // Example: Authentication and company operations
    info!("Getting company information...");
    match client.auth().get_company().await {
        Ok(company) => {
            info!("Full company details: {:#?}", company);
        }
        Err(e) => {
            error!("Error getting company: {:?}", e);
        }
    }

    // Example: Working with tasks
    info!("Searching for tasks...");
    match client.tasks().search().limit(10.0).execute().await {
        Ok(tasks) => {
            info!("Found {} tasks", tasks.content.len());
            for task in tasks.content.iter().take(3) {
                info!("Full task details:\n{:#?}", task);
            }
        }
        Err(e) => {
            error!("Error searching tasks: {:?}", e);
        }
    }
    // Example: Working with projects
    info!("Searching for projects...");
    match client
        .projects()
        .search()
        .title("important")
        .limit(5.0)
        .execute()
        .await
    {
        Ok(projects) => {
            info!("Found {} projects", projects.content.len());
            for project in projects.content.iter().take(3) {
                info!("Full project details: \n {:#?}", project);
            }
        }
        Err(e) => {
            error!("Error searching projects: {:?}", e);
        }
    }

    // Example: Working with users
    info!("Searching for users...");
    match client.users().search().limit(10.0).execute().await {
        Ok(users) => {
            info!("Found {} users", users.content.len());
            for user in users.content.iter().take(3) {
                info!("Full user details: {:#?}", user);
            }
        }
        Err(e) => {
            error!("Error searching users: {:?}", e);
        }
    }

    // Example: Working with boards
    info!("Searching for boards...");
    match client.boards().search().limit(10.0).execute().await {
        Ok(boards) => {
            info!("Found {} boards", boards.content.len());
            for board in boards.content.iter().take(3) {
                info!("Full board details: {:#?}", board);
            }
        }
        Err(e) => {
            error!("Error searching boards: {:?}", e);
        }
    }

    // Example: Working with columns
    info!("Searching for columns...");
    match client
        .columns()
        .search(None, Some(10.0), Some(0.0), None, None)
        .await
    {
        Ok(columns) => {
            info!("Found {} columns", columns.content.len());
            for column in columns.content.iter().take(3) {
                info!("Full column details: {:#?}", column);
            }
        }
        Err(e) => {
            error!("Error searching columns: {:?}", e);
        }
    }
    // Example: Working with group chats
    info!("Searching for group chats...");
    match client
        .group_chats()
        .search(None, Some(10.0), Some(0.0), None)
        .await
    {
        Ok(group_chats) => {
            info!("Found {} group chats", group_chats.content.len());
            for chat in group_chats.content.iter().take(3) {
                info!("Full group chats details: {:#?}", chat);
            }
        }
        Err(e) => {
            error!("Error searching group chats: {:?}", e);
        }
    }

    // Example: Working with stickers (sprint stickers)
    info!("Searching for sprint stickers...");
    match client
        .stickers()
        .search_sprint_stickers(None, Some(10.0), Some(0.0), None, None)
        .await
    {
        Ok(stickers) => {
            info!("Found {} sprint stickers", stickers.content.len());
            for sticker in stickers.content.iter().take(3) {
                info!("Full sprint sticker details: {:#?}", sticker);
            }
        }
        Err(e) => {
            error!("Error searching sprint stickers: {:?}", e);
        }
    }

    // Example: Working with webhooks
    info!("Searching for webhooks...");
    match client.webhooks().search(None).await {
        Ok(webhooks) => {
            info!("Found {} webhook(s)", webhooks.len());
            for (i, webhook) in webhooks.iter().enumerate() {
                info!("Webhook #{}:\n{:#?}", i + 1, webhook);
            }
        }
        Err(e) => {
            error!("Error searching webhooks: {:?}", e);
        }
    }

    info!("All API examples completed successfully!");
    Ok(())
}
