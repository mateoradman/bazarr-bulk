use clap::{Parser, Subcommand};
use reqwest::{header, Client};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::Duration};

use crate::{
    actions::Action, connection::check_health, data_types::app_config::AppConfig, db::init_db,
};

#[derive(Parser)]
#[command(name = "Bazarr Bulk Actions CLI")]
#[command(author = "Mateo Radman <radmanmateo@gmail.com>")]
#[command(about = "Performs bulk operations on subtitles of movies and tv shows using Bazarr's API", long_about = None)]
pub struct Cli {
    /// Path to the JSON configuration file
    #[arg(required = true, short, long, value_name = "FILE")]
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
    /// Filter records by Sonarr/Radarr ID (comma-separated)
    #[arg(long, required = false, value_delimiter = ',')]
    ids: Vec<u32>,
    /// Skip N records (ignored if ids are specified) [default: skip none]
    #[arg(long, default_value_t = 0)]
    offset: u32,
    /// Limit to N records (ignored if ids are specified) [default: unlimited]
    #[arg(long)]
    limit: Option<u32>,
    /// Skip already processed as queried from the db.
    /// Must have all subtitles processed to be skipped.
    #[arg(long, default_value_t = false, required = false)]
    skip_processed: bool,
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
        let url = config.construct_url();
        check_health(&client, &url).await;
        let db_conn = init_db().await?;
        let mut action = Action::new(client, url, db_conn);
        match self {
            Commands::Movies(c) => {
                action.action = c.subcommand;
                action.ids = c.ids;
                action.limit = c.limit;
                action.offset = c.offset;
                action.skip_processed = c.skip_processed;
                action.movies().await
            }
            Commands::TVShows(c) => {
                action.action = c.subcommand;
                action.ids = c.ids;
                action.limit = c.limit;
                action.offset = c.offset;
                action.skip_processed = c.skip_processed;
                action.tv_shows().await
            }
        }
    }
}

#[derive(Subcommand, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionCommands {
    /// Sync all
    Sync(SyncOptions),
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

#[allow(clippy::to_string_trait_impl)]
impl ToString for ActionCommands {
    fn to_string(&self) -> String {
        match self {
            ActionCommands::Sync(_) => "sync".to_string(),
            ActionCommands::OCRFixes => "OCR_fixes".to_string(),
            ActionCommands::CommonFixes => "common".to_string(),
            ActionCommands::RemoveHearingImpaired => "remove_HI".to_string(),
            ActionCommands::RemoveStyleTags => "remove_tags".to_string(),
            ActionCommands::FixUppercase => "fix_uppercase".to_string(),
            ActionCommands::ReverseRTL => "reverse_rtl".to_string(),
        }
    }
}

#[derive(clap::Args, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SyncOptions {
    /// Reference for sync from video file track number (a:0), subtitle (s:0), or some subtitles file path
    #[arg(short)]
    pub reference: Option<String>,
    /// Seconds of offset allowed when syncing [default: null]
    #[arg(short, value_name = "MAX OFFSET")]
    pub max_offset_seconds: Option<u32>,
    /// Do not attempt to fix framerate [default: false]
    #[arg(short, default_value_t = false)]
    pub no_fix_framerate: bool,
    /// Use Golden-Section search [default: false]
    #[arg(short, default_value_t = false)]
    pub gss: bool,
}
