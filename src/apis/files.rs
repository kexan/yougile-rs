use crate::{
    apis::{configuration::Configuration, parse_response, RequestBuilderExt},
    models::FileUpload,
    YougileError,
};

const UPLOAD_FILE_PATH: &str = "/api-v2/upload-file";

/// Загружает файл на сервер и возвращает его URL.
/// Если вы хотите использовать curl из командной строки,
/// не указывайте явно boundary в Content-Type — curl сам выставит нужный заголовок.
pub async fn upload_file(
    configuration: &Configuration,
    file_data: Vec<u8>,
    file_name: &str,
) -> Result<FileUpload, YougileError> {
    let url = format!("{}{}", configuration.base_path, UPLOAD_FILE_PATH);

    let form = reqwest::multipart::Form::new().part(
        "file",
        reqwest::multipart::Part::bytes(file_data).file_name(file_name.to_string()),
    );

    let resp = configuration
        .client
        .post(&url)
        .multipart(form)
        .with_auth_headers(configuration)
        .send()
        .await?;

    parse_response(resp).await
}
