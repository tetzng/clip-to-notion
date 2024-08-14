mod cli;
mod config;
mod notion;
mod scraper;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use config::load_config;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Init => {
            config::init_config()?;
        }
        Command::Run { url, tags } => {
            let cfg = load_config()?;
            notion::post_to_notion(cfg, url, tags).await?;
        }
    }

    Ok(())
}
