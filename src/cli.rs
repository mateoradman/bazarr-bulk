use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::{
    actions::{self, ActionConfig},
    data_types::{app_config::AppConfig, request::MediaType},
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
        subcommand: MediaCommands,
    },
    /// perform operations on tv shows
    TVShows {
        /// list available actions
        #[command(subcommand)]
        subcommand: MediaCommands,
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

#[derive(Subcommand)]
pub enum MediaCommands {
    /// list all
    List {
        /// lists test values
        #[arg(default_value = "0")]
        start: u8,
    },
    /// sync all
    Sync,
    /// perform OCR fixes on all
    OCRFixes,
    /// perform common fixes on all
    CommonFixes,
}

impl MediaCommands {
    pub async fn run(
        self,
        media_type: MediaType,
        config: AppConfig,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut action_config = ActionConfig {
            media_type,
            config,
            start: None,
        };
        match self {
            MediaCommands::List { start } => {
                action_config.start = Some(start);
                actions::list_records(&action_config).await
            }
            MediaCommands::Sync => actions::sync_subtitles(&action_config).await,
            MediaCommands::OCRFixes => actions::ocr_fixes(&action_config).await,
            MediaCommands::CommonFixes => actions::common_fixes(&action_config).await,
        }
    }
}
