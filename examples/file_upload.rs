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

    // Example: Upload a file
    // Create some sample file data (in real usage, you'd read from a file)
    let file_data = b"Hello, this is a test file content!".to_vec();
    let file_name = "test_file.txt";

    log::info!("Uploading file: {}...", file_name);
    match client.upload_file(file_data, file_name).await {
        Ok(file_upload) => {
            log::info!("File uploaded successfully!");
            log::info!("  Result: {}", file_upload.result);
            log::info!("  URL: {}", file_upload.url);
            log::info!("  Full URL: {}", file_upload.full_url);
        }
        Err(e) => {
            log::error!("Error uploading file: {:?}", e);
        }
    }

    // Example: Working with sprint stickers
    log::info!("Working with sprint stickers...");
    match client.search_sprint_stickers(None, Some(10.0), Some(0.0), None, None).await {
        Ok(stickers) => {
            log::info!("Found {} sprint stickers", stickers.content.len());
        }
        Err(e) => {
            log::error!("Error searching sprint stickers: {:?}", e);
        }
    }

    // Example: Working with webhooks
    log::info!("Working with webhooks...");
    match client.search_webhooks(None).await {
        Ok(webhook) => {
            log::info!("Found webhook: {} (ID: {})", webhook.url, webhook.id);
        }
        Err(e) => {
            log::error!("Error searching webhooks: {:?}", e);
        }
    }

    Ok(())
}