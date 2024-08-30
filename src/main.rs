mod cli;
mod config;
mod encoding;
mod notion;
mod scraper;

use clap::Parser;
use cli::{Cli, Command};
use config::load_config;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Init => {
            config::init_config().unwrap();
        }
        Command::Run { url, tags } => {
            let cfg = load_config().unwrap();
            notion::post_to_notion(cfg, url, tags).await.unwrap();
        }
    }
}
