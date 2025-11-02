use std::{borrow::Borrow, fmt::Debug, io::IsTerminal, process::exit, sync::Arc};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Url;
use reqwest_middleware::ClientWithMiddleware;
use rusqlite::Connection;
use serde::de::DeserializeOwned;
use tokio::sync::Mutex;

use crate::{
    cli::ActionCommands,
    data_types::{
        request::ActionPayload,
        response::{Episode, Movie, PaginatedResponse, TVShow},
    },
    db::{
        filter_unprocessed_episodes, filter_unprocessed_movies, mark_episode_subtitle_processed,
        mark_movie_subtitle_processed,
    },
};

pub struct Action {
    pub client: ClientWithMiddleware,
    pub base_url: Url,
    pub action: ActionCommands,
    pub ids: Vec<u32>,
    pub offset: u32,
    pub limit: Option<u32>,
    pub skip_processed: bool,
    pub language_code: Option<String>,
    pub pb: ProgressBar,
    pub db_conn: Arc<Mutex<Connection>>,
    pub is_tty: bool,
}

impl Action {
    pub fn new(
        client: ClientWithMiddleware,
        base_url: Url,
        db_conn: Arc<Mutex<Connection>>,
    ) -> Self {
        let is_tty = std::io::stdout().is_terminal();
        let pb = if is_tty {
            ProgressBar::new(0)
        } else {
            // For non-TTY environments, hide the progress bar
            ProgressBar::hidden()
        };
        Self {
            client,
            base_url,
            action: ActionCommands::OCRFixes,
            ids: Vec::new(),
            offset: 0,
            skip_processed: false,
            language_code: None,
            limit: None,
            pb,
            db_conn,
            is_tty,
        }
    }

    /// set message on progress bar or print to stdout based on TTY
    fn log_info(&self, pb: &ProgressBar, msg: impl Into<String>) {
        let message = msg.into();
        if self.is_tty {
            pb.set_message(message);
        } else {
            println!("{}", message);
        }
    }

    /// set error message on progress bar or print to stderr based on TTY
    fn log_error(&self, pb: &ProgressBar, msg: impl Into<String>) {
        let message = msg.into();
        if self.is_tty {
            pb.set_message(message);
        } else {
            eprintln!("{}", message);
        }
    }

    /// finish progress bar with message or print to stdout based on TTY
    fn finish(&self, pb: &ProgressBar, msg: impl Into<String>) {
        let message = msg.into();
        if self.is_tty {
            pb.finish_with_message(message);
        } else {
            println!("{}", message);
        }
    }

    /// Check if subtitle matches the language filter (if specified)
    fn matches_language_filter(&self, subtitle_code: Option<&String>) -> bool {
        match (&self.language_code, subtitle_code) {
            (Some(lang_code), Some(subtitle_code)) => lang_code == subtitle_code,
            (Some(_), None) => false,
            (None, _) => true,
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

    async fn limit_records(&self, mut url: Url, query_param: &str) -> Url {
        if !self.ids.is_empty() {
            for id in &self.ids {
                url.query_pairs_mut()
                    .append_pair(query_param, &id.to_string());
            }
        } else if self.limit.is_some() || self.offset > 0 {
            let length = match self.limit {
                Some(val) => val,
                None => u32::MAX,
            };
            url.query_pairs_mut()
                .append_pair("length", &length.to_string())
                .append_pair("start", &self.offset.to_string());
        }
        url
    }

    async fn perform(
        &self,
        mut payload: ActionPayload,
    ) -> Result<reqwest::Response, reqwest_middleware::Error> {
        let mut url = self.base_url.clone();
        url.path_segments_mut().unwrap().push("subtitles");
        let action_string: String = self.action.to_string();
        url.query_pairs_mut().append_pair("action", &action_string);
        if let ActionCommands::Sync(sync_options) = &self.action.borrow() {
            payload.reference = sync_options.reference.clone();
            payload.max_offset_seconds = sync_options.max_offset_seconds;
            payload.no_fix_framerate = Some(sync_options.no_fix_framerate);
            payload.gss = Some(sync_options.gss);
        }
        let body = serde_json::to_vec(&payload).unwrap();
        self.client
            .patch(url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
    }

    async fn process_episode_subtitle(&self, pb: &ProgressBar, episode: Episode) {
        for subtitle in episode.subtitles {
            if !subtitle.is_valid() {
                continue;
            }

            if !self.matches_language_filter(subtitle.audio_language_item.code2.as_ref()) {
                continue;
            }

            let msg = format!(
                "Performing action {} on {} subtitle of episode {}",
                self.action.to_string(),
                subtitle.audio_language_item.name,
                episode.title,
            );
            self.log_info(pb, msg);

            let payload = ActionPayload::new(episode.sonarr_episode_id, "episode", &subtitle);
            match self.perform(payload).await {
                Ok(res) => match res.error_for_status() {
                    Ok(_) => {
                        let msg = format!(
                            "Successfully performed action {} on {} subtitle of episode {}",
                            self.action.to_string(),
                            subtitle.audio_language_item.name,
                            episode.title,
                        );
                        self.log_info(pb, msg);
                        let _ = mark_episode_subtitle_processed(
                            self.db_conn.clone(),
                            episode.sonarr_episode_id,
                            episode.title.clone(),
                            subtitle,
                        )
                        .await;
                    }
                    Err(err) => {
                        let msg = format!(
                            "Error performing action {} on {} subtitle of episode {}: {}",
                            self.action.to_string(),
                            subtitle.audio_language_item.name,
                            episode.title,
                            err,
                        );
                        self.log_error(pb, msg);
                    }
                },
                Err(err) => {
                    let msg = format!("Error connecting to Bazarr: {}", err);
                    self.log_error(&self.pb, msg);
                    exit(1);
                }
            }
        }
    }

    async fn process_movie_subtitle(&self, movie: Movie) {
        for subtitle in movie.subtitles {
            if !subtitle.is_valid() {
                continue;
            }

            if !self.matches_language_filter(subtitle.audio_language_item.code2.as_ref()) {
                continue;
            }

            let msg = format!(
                "Performing action {} on {} subtitle of movie {}",
                self.action.to_string(),
                subtitle.audio_language_item.name,
                movie.title,
            );
            self.log_info(&self.pb, msg);

            let payload = ActionPayload::new(movie.radarr_id, "movie", &subtitle);
            match self.perform(payload).await {
                Ok(res) => match res.error_for_status() {
                    Ok(_) => {
                        let msg = format!(
                            "Successfully performed action {} on {} subtitle of movie {}",
                            self.action.to_string(),
                            subtitle.audio_language_item.name,
                            movie.title,
                        );
                        self.log_info(&self.pb, msg);
                        let _ = mark_movie_subtitle_processed(
                            self.db_conn.clone(),
                            movie.radarr_id,
                            movie.title.clone(),
                            subtitle,
                        )
                        .await;
                    }
                    Err(err) => {
                        let msg = format!(
                            "Error performing action {} on {} subtitle of episode {}: {}",
                            self.action.to_string(),
                            subtitle.audio_language_item.name,
                            movie.title,
                            err,
                        );
                        self.log_error(&self.pb, msg);
                    }
                },
                Err(err) => {
                    let msg = format!("Error connecting to Bazarr: {}", err);
                    self.log_error(&self.pb, msg);
                    exit(1);
                }
            }
        }
    }

    pub async fn movies(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_tty {
            self.pb.set_style(
                ProgressStyle::with_template(
                    "[{bar:60.green/yellow}] {pos:>7}/{len:7} Movies\n{msg}",
                )
                .unwrap()
                .progress_chars("##-"),
            );
        }

        let mut url = self.base_url.clone();
        url.path_segments_mut().unwrap().push("movies");
        url = self.limit_records(url, "radarrid[]").await;
        let response = self.get_all::<Movie>(url).await?;
        let mut movies = response.data;
        if self.skip_processed {
            let initial_len = movies.len();
            movies = filter_unprocessed_movies(self.db_conn.clone(), movies).await?;
            let after_len = movies.len();
            let difference = initial_len - after_len;
            println!("Skipped {difference} already processed movies...");
        }
        let num_movies: u64 = movies.len() as u64;
        if num_movies == 0 {
            self.finish(&self.pb, "No movies found");
            return Ok(());
        }

        if !self.is_tty {
            println!("Processing {} movies...", num_movies);
        }

        self.pb.set_length(num_movies);
        for (idx, movie) in movies.into_iter().enumerate() {
            if !self.is_tty {
                println!("Processing movie {}/{}", idx + 1, num_movies);
            }
            self.process_movie_subtitle(movie).await;
            self.pb.inc(1);
        }

        self.finish(
            &self.pb,
            format!(
                "Finished performing action {} on all movies",
                self.action.to_string(),
            ),
        );
        Ok(())
    }

    pub async fn tv_shows(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mp = MultiProgress::new();
        let pb_main = mp.add(self.pb.clone());

        if self.is_tty {
            pb_main.set_style(
                ProgressStyle::with_template(
                    "[{bar:60.green/yellow}] {pos:>7}/{len:7} TV Shows\n{msg}",
                )
                .unwrap()
                .progress_chars("##-"),
            );
        }

        let mut url = self.base_url.clone();
        url.path_segments_mut().unwrap().push("series");
        url = self.limit_records(url, "seriesid[]").await;
        let response = self.get_all::<TVShow>(url.clone()).await?;
        let num_series: u64 = response.data.len() as u64;
        if num_series == 0 {
            self.finish(&pb_main, "No tv shows found");
            return Ok(());
        }

        if !self.is_tty {
            println!("Processing {} TV shows...", num_series);
        }

        pb_main.set_length(num_series);
        let sub_pb = if self.is_tty {
            let pb = mp.insert_after(&pb_main, ProgressBar::new(0));
            pb.set_style(
                ProgressStyle::with_template(
                    "[{bar:60.cyan/blue}] {pos:>7}/{len:7} Episodes\n{msg}",
                )
                .unwrap()
                .progress_chars("##-"),
            );
            pb
        } else {
            ProgressBar::hidden()
        };

        url.path_segments_mut().unwrap().pop().push("episodes");
        for (series_idx, series) in response.data.into_iter().enumerate() {
            let msg = format!("Processing tv show {}", series.title);
            if self.is_tty {
                pb_main.set_message(msg.clone());
            } else {
                println!("TV Show {}/{}: {}", series_idx + 1, num_series, msg);
            }

            let query_param = format!("seriesid[]={}", series.sonarr_series_id);
            let mut new_url = url.clone();
            new_url.set_query(Some(&query_param));
            let response = self.get_all::<Episode>(new_url).await?;
            let mut episodes = response.data;
            if self.skip_processed {
                println!(
                    "Processing {} episodes, checking for already processed ones...",
                    episodes.len()
                );
                let initial_len = episodes.len();
                episodes = filter_unprocessed_episodes(self.db_conn.clone(), episodes).await?;
                let after_len = episodes.len();
                let difference = initial_len - after_len;
                if difference > 0 {
                    self.log_info(
                        &pb_main,
                        format!("Skipped {difference} already processed episodes..."),
                    );
                } else {
                    self.log_info(&pb_main, "No previously processed episodes");
                }
            }
            let num_episodes: u64 = episodes.len() as u64;
            sub_pb.set_position(0);
            sub_pb.set_length(num_episodes);
            if num_episodes == 0 {
                self.finish(&sub_pb, "No episodes found");
                continue;
            }

            if !self.is_tty {
                println!("  Processing {} episodes...", num_episodes);
            }

            for (ep_idx, episode) in episodes.into_iter().enumerate() {
                if !self.is_tty {
                    println!("    Episode {}/{}", ep_idx + 1, num_episodes);
                }
                self.process_episode_subtitle(&sub_pb, episode).await;
                sub_pb.inc(1);
            }
            pb_main.inc(1);

            self.log_info(
                &pb_main,
                format!("Finished processing tv show {}", series.title),
            );
        }

        self.finish(
            &pb_main,
            format!(
                "Finished performing action {} on all tv shows",
                self.action.to_string(),
            ),
        );
        Ok(())
    }
}
