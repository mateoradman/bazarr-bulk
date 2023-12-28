use std::fmt::Debug;

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::data_types::{
    app_config::AppConfig,
    request::MediaType,
    response::{Movie, PaginatedResponse},
};

pub struct ActionConfig {
    pub config: AppConfig,
    pub media_type: MediaType,
    pub start: Option<u8>,
}

async fn get_all<T>(
    action_config: &ActionConfig,
) -> Result<PaginatedResponse<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
    T: Debug,
{
    let endpoint = match action_config.media_type {
        MediaType::TVShow => "series",
        MediaType::Movie => "movies",
    };
    let client = Client::new();
    let url = get_endpoint_url(action_config, endpoint).await;
    let body: PaginatedResponse<T> = client
        .get(&url)
        .header("X-API-KEY", &action_config.config.api_key)
        .send()
        .await?
        .json()
        .await?;
    Ok(body)
}

pub async fn list_records(
    action_config: &ActionConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = get_all::<Movie>(action_config).await?;
    let record_titles = response
        .data
        .iter()
        .map(|record| record.common_attributes.title.clone())
        .collect::<Vec<String>>();
    Ok(format!("{:?}", record_titles))
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
        "{}://{}:{}/api/{}",
        action_config.config.protocol,
        action_config.config.host,
        action_config.config.port,
        endpoint
    )
}
