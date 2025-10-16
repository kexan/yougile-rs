use yougile_client::{YouGileClient, YougileError};

#[tokio::main]
async fn main() -> Result<(), YougileError> {
    // Create a new client with a bearer token
    let config =
        yougile_client::apis::configuration::Configuration::new("your-api-token-here".to_string())
            .with_base_path("https://yougile.com");

    let client = YouGileClient::new(config);

    // Example: Upload a file
    // Create some sample file data (in real usage, you'd read from a file)
    let file_data = b"Hello, this is a test file content!".to_vec();
    let file_name = "test_file.txt";

    println!("Uploading file: {}...", file_name);
    match client.upload_file(file_data, file_name).await {
        Ok(file_upload) => {
            println!("File uploaded successfully!");
            println!("  Result: {}", file_upload.result);
            println!("  URL: {}", file_upload.url);
            println!("  Full URL: {}", file_upload.full_url);
        }
        Err(e) => {
            eprintln!("Error uploading file: {:?}", e);
        }
    }

    // Example: Working with stickers
    println!("\nWorking with sprint stickers...");
    match client
        .search_sprint_stickers(None, Some(10.0), Some(0.0), None, None)
        .await
    {
        Ok(stickers) => {
            println!("Found {} sprint stickers", stickers.content.len());
        }
        Err(e) => {
            eprintln!("Error searching sprint stickers: {:?}", e);
        }
    }

    // Example: Working with webhooks
    println!("\nWorking with webhooks...");
    match client.search_webhooks(None).await {
        Ok(webhook) => {
            println!("Found webhook: {} (ID: {})", webhook.url, webhook.id);
        }
        Err(e) => {
            eprintln!("Error searching webhooks: {:?}", e);
        }
    }

    Ok(())
}

