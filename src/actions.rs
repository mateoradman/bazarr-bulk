use std::fmt::Debug;

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;

use crate::{
    cli::ActionCommands,
    data_types::{
        request::ActionPayload,
        response::{Episode, Movie, PaginatedResponse, TVShow},
    },
};

pub struct Action {
    pub client: Client,
    pub base_url: Url,
    pub action: ActionCommands,
    pub pb: ProgressBar,
}

impl Action {
    pub fn new(client: Client, base_url: Url) -> Self {
        let pb = ProgressBar::new(0);
        pb.set_style(
            ProgressStyle::with_template("[{bar:60.cyan/blue}] {pos:>7}/{len:7}\n{msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        Self {
            client,
            base_url,
            action: ActionCommands::Sync,
            pb,
        }
    }

    async fn get_all<T>(&self, url: Url) -> Result<PaginatedResponse<T>, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
        T: Debug,
    {
        let req = self.client.get(url);
        let res = req.send().await?;
        let body: PaginatedResponse<T> = res.json().await?;
        Ok(body)
    }

    async fn perform(&self, payload: ActionPayload) -> Result<(), Box<dyn std::error::Error>> {
        let mut url = self.base_url.clone();
        url.path_segments_mut().unwrap().push("subtitles");
        let response = self.client.patch(url).json(&payload).send().await?;
        response.error_for_status()?;
        Ok(())
    }

    async fn process_episode_subtitle(&self, series: &TVShow, episode: Episode) {
        for subtitle in episode.subtitles {
            if !subtitle.is_valid() {
                println!("Skipping invalid subtitle: {}", subtitle.audio_language_item.name);
                continue;
            }

            let payload = ActionPayload {
                id: episode.sonarr_episode_id,
                media_type: String::from("episode"),
                language: subtitle.audio_language_item.code2.unwrap(),
                path: subtitle.path.unwrap(),
                action: self.action.clone(),
            };
            match self.perform(payload).await {
                Ok(_) => {
                    self.pb.set_message(format!("Successfully performed action `{}` on {} subtitle of episode `{}` of tv show `{}`", 
                        self.action.to_string(), 
                        subtitle.audio_language_item.name,
                        episode.title, 
                        series.title, 
                    ));
                }
                Err(err) => {
                    self.pb.set_message(format!("Error performing action `{}` on {} subtitle of episode `{}` of tv show `{}` due to error {}", 
                        self.action.to_string(), 
                        subtitle.audio_language_item.name,
                        episode.title, 
                        series.title, 
                        err,
                    ));
                }
            }
        }
    }

    async fn process_movie_subtitle(&self, movie: Movie) {
        for subtitle in movie.subtitles {
            if !subtitle.is_valid() {
                println!("Skipping invalid subtitle: {}", subtitle.audio_language_item.name);
                continue;
            }
            let payload = ActionPayload {
                id: movie.radarr_id,
                media_type: String::from("movie"),
                language: subtitle.audio_language_item.code2.unwrap(),
                path: subtitle.path.unwrap(),
                action: self.action.clone(),
            };
            match self.perform(payload).await {
                Ok(_) => {
                    self.pb.set_message(format!(
                        "Successfully performed action `{}` on {} subtitle of movie `{}`",
                        self.action.to_string(),
                        subtitle.audio_language_item.name,
                        movie.title,
                    ));
                }
                Err(err) => {
                    self.pb.set_message(
                        format!("Error performing action `{}` on {} subtitle of movie `{}` due to error {}", 
                        self.action.to_string(), 
                        subtitle.audio_language_item.name,
                        movie.title, 
                        err,
                    ));
                }
            }
        }
    }

    pub async fn movies(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut url = self.base_url.clone();
        url.path_segments_mut().unwrap().push("movies");
        let response = self.get_all::<Movie>(url).await?;
        self.pb.set_length(response.data.len() as u64);
        for movie in response.data {
            self.process_movie_subtitle(movie).await;
            self.pb.inc(1);
        }
        self.pb.finish_with_message(format!(
            "Finished performing action {} on all movies",
            self.action.to_string(),
        ));
        Ok(())
    }

    pub async fn tv_shows(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut url = self.base_url.clone();
        url.path_segments_mut().unwrap().push("series");
        let response = self.get_all::<TVShow>(url.clone()).await?;
        let num_series = response.data.len();
        self.pb.set_length(num_series as u64);
        url.path_segments_mut().unwrap().pop().push("episodes");
        for series in response.data {
            let query_param = format!("seriesid[]={}", series.sonarr_series_id);
            let mut new_url = url.clone();
            new_url.set_query(Some(&query_param));
            let response = self.get_all::<Episode>(new_url).await?;
            for episode in response.data {
                self.process_episode_subtitle(&series, episode).await;
            }
            self.pb.inc(1);
        }
        self.pb.finish_with_message(format!(
            "Finished performing action {} on all tv shows",
            self.action.to_string(),
        ));
        Ok(())
    }
}
