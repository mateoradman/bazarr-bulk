use std::fmt::Debug;

use reqwest::{Client, Method, Response, Url};
use serde::de::DeserializeOwned;
use serde_json::Number;

use crate::data_types::{
    request::{ActionPayload, MediaType},
    response::{Episode, Movie, PaginatedResponse, TVShow},
};

pub struct ActionDetail {
    pub client: Client,
    pub media_type: MediaType,
    pub base_url: Url,
}

async fn get_all<T>(
    client: &Client,
    url: Url,
) -> Result<PaginatedResponse<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
    T: Debug,
{
    let req = client.get(url);
    let res = req.send().await?;
    let body: PaginatedResponse<T> = res.json().await?;
    Ok(body)
}

pub async fn get_movie_ids(
    base_url: &Url,
    client: &Client,
) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let mut url = base_url.clone();
    url.path_segments_mut().unwrap().push("movies");
    let response = get_all::<Movie>(client, url).await?;
    Ok(response.data.into_iter().map(|m| m.radarr_id).collect())
}

pub async fn get_episode_ids(
    base_url: &Url,
    client: &Client,
) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let mut url = base_url.clone();
    url.path_segments_mut().unwrap().push("series");
    let response = get_all::<TVShow>(client, url.clone()).await?;
    let series_ids: Vec<u32> = response
        .data
        .into_iter()
        .map(|s| s.sonarr_series_id)
        .collect();
    url.path_segments_mut().unwrap().pop().push("episodes");
    let ids_as_string = serde_json::to_string(&series_ids)?;
    let query_param = format!("seriesid[]={}", ids_as_string);
    url.set_query(Some(&query_param));
    let response = get_all::<Episode>(client, url).await?;
    Ok(response
        .data
        .into_iter()
        .map(|e| e.sonarr_episode_id)
        .collect())
}

pub async fn perform_action(
    base_url: &Url,
    client: &Client,
    payload: ActionPayload,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut url = base_url.clone();
    url.path_segments_mut().unwrap().push("subtitles");
    let response = client.post(url).json(&payload).send().await?;
    Ok(response.text().await?)
}
