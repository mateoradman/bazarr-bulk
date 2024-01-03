use clap::{Parser, Subcommand, ValueEnum};
use reqwest::{header, Client, Url};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

use crate::{
    actions::{self, get_episode_ids, get_movie_ids, perform_action, ActionDetail},
    data_types::{
        app_config::AppConfig,
        request::{ActionPayload, MediaType},
    },
};

#[derive(Parser)]
#[command(name = "Bazarr Bulk Actions CLI")]
#[command(author = "Mateo Radman <radmanmateo@gmail.com>")]
#[command(about = "Performs bulk operations on subtitles of movies and tv shows using Bazarr's API", long_about = None)]
pub struct Cli {
    // Path to the JSON configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub async fn run(self, config: AppConfig) -> Result<String, Box<dyn std::error::Error>> {
        self.command.run(config).await
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// perform operations on movies
    Movies {
        /// list available actions
        #[command(subcommand)]
        subcommand: ActionCommands,
    },
    /// perform operations on tv shows
    TVShows {
        /// list available actions
        #[command(subcommand)]
        subcommand: ActionCommands,
    },
}

impl Commands {
    pub async fn run(self, config: AppConfig) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Commands::Movies { subcommand } => subcommand.run(MediaType::Movie, config).await,
            Commands::TVShows { subcommand } => subcommand.run(MediaType::TVShow, config).await,
        }
    }
}

#[derive(Subcommand, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, ValueEnum)]
pub enum ActionCommands {
    /// sync all
    Sync,
    /// perform OCR fixes on all
    OCRFixes,
    /// perform common fixes on all
    CommonFixes,
    /// remove hearing impaired tags from subtitles
    RemoveHearingImpaired,
    /// remove style tags from subtitles
    RemoveStyleTags,
    /// fix uppercase subtitles
    FixUppercase,
    /// reverse RTL directioned subtitles
    ReverseRTL,
}

impl ActionCommands {
    pub async fn run(
        self,
        media_type: MediaType,
        config: AppConfig,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-API-KEY",
            header::HeaderValue::from_str(&config.api_key).unwrap(),
        );
        let client = Client::builder().default_headers(headers).build()?;
        let base_url = format!("{}://{}:{}/api", config.protocol, config.host, config.port);
        let url = Url::from_str(&base_url)?;
        let ids: Vec<u32> = match media_type {
            MediaType::Movie => get_movie_ids(&url, &client).await?,
            MediaType::TVShow => get_episode_ids(&url, &client).await?,
        };
        println!("{:?}", ids);
        Ok(String::from("aaa"))
    }
}

impl ToString for ActionCommands {
    fn to_string(&self) -> String {
        match self {
            ActionCommands::Sync => "sync".to_string(),
            ActionCommands::OCRFixes => "OCR_fixes".to_string(),
            ActionCommands::CommonFixes => "common".to_string(),
            ActionCommands::RemoveHearingImpaired => "remove_HI".to_string(),
            ActionCommands::RemoveStyleTags => "remove_tags".to_string(),
            ActionCommands::FixUppercase => "fix_uppercase".to_string(),
            ActionCommands::ReverseRTL => "reverse_rtl".to_string(),
        }
    }
}
