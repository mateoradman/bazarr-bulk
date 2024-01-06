use std::fmt::Debug;

use reqwest::{Client, Url};
use serde::de::DeserializeOwned;

use crate::{
    cli::ActionCommands,
    data_types::{
        request::ActionPayload,
        response::{Episode, Movie, PaginatedResponse, TVShow},
    },
};

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

pub async fn movie_action(
    base_url: &Url,
    client: &Client,
    subcommand: ActionCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut url = base_url.clone();
    url.path_segments_mut().unwrap().push("movies");
    let response = get_all::<Movie>(client, url).await?;
    println!("Found {} movies", response.data.len());
    for movie in response.data {
        for subtitle in movie.subtitles {
            if subtitle.path.is_none() {
                println!(
                    "Skipping subtitle with a language {} because it has no file path",
                    subtitle.audio_language_item.name
                );
                continue;
            }
            println!(
                "Performing action {} on subtitle {} of movie {}",
                subcommand.to_string(),
                subtitle.audio_language_item.name,
                movie.common_attributes.title
            );
            let payload = ActionPayload {
                id: movie.radarr_id,
                media_type: String::from("movie"),
                language: subtitle.audio_language_item.code2,
                path: subtitle.path.unwrap(),
                action: subcommand.clone(),
            };
            perform_action(base_url, client, payload).await?;
        }
    }
    Ok(())
}

pub async fn series_action(
    base_url: &Url,
    client: &Client,
    subcommand: ActionCommands,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut url = base_url.clone();
    url.path_segments_mut().unwrap().push("series");
    let response = get_all::<TVShow>(client, url.clone()).await?;
    println!("Found {} series", response.data.len());
    url.path_segments_mut().unwrap().pop().push("episodes");
    for series in response.data {
        println!("Processing tv show {}", series.common_attributes.title);
        let query_param = format!("seriesid[]={}", series.sonarr_series_id);
        let mut new_url = url.clone();
        new_url.set_query(Some(&query_param));
        let response = get_all::<Episode>(client, new_url).await?;
        println!("Retrieved {} episodes in total", { response.data.len() });
        for episode in response.data {
            for subtitle in episode.subtitles {
                if subtitle.path.is_none() {
                    println!(
                        "Skipping subtitle with a language {} because it has no file path",
                        subtitle.audio_language_item.name
                    );
                    continue;
                }
                println!(
                    "Performing action {} on subtitle {} of episode {}",
                    subcommand.to_string(),
                    subtitle.audio_language_item.name,
                    episode.title
                );
                let payload = ActionPayload {
                    id: episode.sonarr_episode_id,
                    media_type: String::from("episode"),
                    language: subtitle.audio_language_item.code2,
                    path: subtitle.path.unwrap(),
                    action: subcommand.clone(),
                };
                perform_action(base_url, client, payload).await?;
            }
        }
    }
    Ok(())
}

pub async fn perform_action(
    base_url: &Url,
    client: &Client,
    payload: ActionPayload,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut url = base_url.clone();
    url.path_segments_mut().unwrap().push("subtitles");
    let response = client.patch(url).json(&payload).send().await?;
    let res_body = response.text().await?;
    println!("{}", res_body);
    Ok(res_body)
}
