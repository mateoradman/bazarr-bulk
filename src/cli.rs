use clap::{Parser, Subcommand, ValueEnum};
use reqwest::{header, Client, Url};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

use crate::{actions::Action, data_types::app_config::AppConfig};

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
    pub async fn run(self, config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
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
    pub async fn run(self, config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-API-KEY",
            header::HeaderValue::from_str(&config.api_key).unwrap(),
        );
        let client = Client::builder().default_headers(headers).build()?;
        let base_url = format!("{}://{}:{}/api", config.protocol, config.host, config.port);
        let url = Url::from_str(&base_url)?;
        let mut action = Action::new(client, url);
        match self {
            Commands::Movies { subcommand } => {
                action.action = subcommand;
                action.movies().await
            }
            Commands::TVShows { subcommand } => {
                action.action = subcommand;
                action.tv_shows().await
            }
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
