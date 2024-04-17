use clap::{Parser, Subcommand, ValueEnum};
use reqwest::{header, Client, Url};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr, time::Duration};

use crate::{actions::Action, connection::check_health, data_types::app_config::AppConfig};

#[derive(Parser)]
#[command(name = "Bazarr Bulk Actions CLI")]
#[command(author = "Mateo Radman <radmanmateo@gmail.com>")]
#[command(about = "Performs bulk operations on subtitles of movies and tv shows using Bazarr's API", long_about = None)]
pub struct Cli {
    /// Path to the JSON configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: PathBuf,

    /// Number of times to retry the request in case of lost connection
    #[arg(short, long, default_value_t = 3)]
    pub max_retries: u32,

    /// Duration of the retry interval (seconds)
    #[arg(short, long, default_value_t = 10)]
    pub retry_interval: u64,

    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub async fn run(self, config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
        println!("Bazarr Bulk CLI v{}", env!("CARGO_PKG_VERSION"));
        self.command
            .run(config, self.max_retries, self.retry_interval)
            .await
    }
}

#[derive(clap::Args)]
pub struct CommonArgs {
    /// Skip N records
    #[arg(long, default_value_t = 0)]
    offset: u32,
    /// Limit to N records [default: unlimited]
    #[arg(long)]
    limit: Option<u32>,
    /// List available actions
    #[command(subcommand)]
    subcommand: ActionCommands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Perform operations on movies
    Movies(CommonArgs),
    /// Perform operations on tv shows
    TVShows(CommonArgs),
}

impl Commands {
    pub async fn run(
        self,
        config: AppConfig,
        max_retries: u32,
        retry_interval: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-API-KEY",
            header::HeaderValue::from_str(&config.api_key).unwrap(),
        );
        let min_retry_interval = Duration::new(retry_interval, 0);
        let max_retry_interval = Duration::new(retry_interval + 1, 0);
        let retry_policy = ExponentialBackoff::builder()
            .retry_bounds(min_retry_interval, max_retry_interval)
            .build_with_max_retries(max_retries);
        let reqwest_client = Client::builder().default_headers(headers).build()?;
        let client = ClientBuilder::new(reqwest_client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();
        let base_url = format!("{}://{}:{}/api", config.protocol, config.host, config.port);
        let url = Url::from_str(&base_url)?;
        check_health(&client, &url).await;

        let mut action = Action::new(client, url);
        match self {
            Commands::Movies(c) => {
                action.action = c.subcommand;
                action.limit = c.limit;
                action.offset = c.offset;
                action.movies().await
            }
            Commands::TVShows(c) => {
                action.action = c.subcommand;
                action.limit = c.limit;
                action.offset = c.offset;
                action.tv_shows().await
            }
        }
    }
}

#[derive(Subcommand, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, ValueEnum)]
pub enum ActionCommands {
    /// Sync all
    Sync,
    /// Perform OCR fixes
    OCRFixes,
    /// Perform common fixes
    CommonFixes,
    /// Remove hearing impaired tags from subtitles
    RemoveHearingImpaired,
    /// Remove style tags from subtitles
    RemoveStyleTags,
    /// Fix uppercase subtitles
    FixUppercase,
    /// Reverse RTL directioned subtitles
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
