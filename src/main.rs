mod actions;
mod cli;
mod data_types;

use clap::Parser;
use cli::Cli;
use data_types::app_config::AppConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    human_panic::setup_panic!();
    let cli = Cli::parse();
    let config = AppConfig::new(cli.config.to_str().unwrap())?;
    cli.run(config).await?;
    Ok(())
}
