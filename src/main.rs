mod actions;
mod cli;
mod data_types;

use clap::Parser;
use cli::Cli;
use data_types::app_config::AppConfig;
use std::io::IsTerminal;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    human_panic::setup_panic!();
    let mut color = ColorChoice::Auto;
    if !std::io::stdin().is_terminal() {
        color = ColorChoice::Never;
    }
    let mut stdout = StandardStream::stdout(color);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
    let cli = Cli::parse();
    let config = AppConfig::new(cli.config.to_str().unwrap())?;
    cli.run(config).await?;
    Ok(())
}
