use reqwest::Client;

use crate::data_types::{app_config::AppConfig, request::MediaType};

pub struct ActionConfig {
    pub config: AppConfig,
    pub media_type: MediaType,
    pub start: Option<u8>,
}

pub async fn list_records(
    action_config: &ActionConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let endpoint = match action_config.media_type {
        MediaType::TVShow => "series",
        MediaType::Movie => "movies",
    };
    let client = Client::new();
    let url = get_endpoint_url(action_config, endpoint).await;
    let body = client
        .get(&url)
        .header("X-API-KEY", &action_config.config.bazarr_api_key)
        .send()
        .await?;
    Ok(body.text().await?)
}

pub async fn sync_subtitles(
    action_config: &ActionConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::from("aaa"))
}
pub async fn ocr_fixes(action_config: &ActionConfig) -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::from("aaa"))
}
pub async fn common_fixes(
    action_config: &ActionConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::from("aaa"))
}

async fn get_endpoint_url(action_config: &ActionConfig, endpoint: &str) -> String {
    format!(
        "http://{}:{}/api/{}",
        action_config.config.bazarr_host, action_config.config.bazarr_port, endpoint
    )
}
