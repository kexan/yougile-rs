use crate::SDKError;
use std::sync::Arc;

use yougile_api_client::YouGileClient;
use yougile_api_client::models::*;

/// API for working with files
pub struct FilesAPI {
    client: Arc<YouGileClient>,
}

impl FilesAPI {
    pub fn new(client: Arc<YouGileClient>) -> Self {
        Self { client }
    }

    /// Upload a file
    pub async fn upload_file(
        &self,
        file_data: Vec<u8>,
        file_name: &str,
    ) -> Result<FileUpload, SDKError> {
        self.client
            .upload_file(file_data, file_name)
            .await
            .map_err(SDKError::from)
    }
}
