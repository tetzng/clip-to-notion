mod cli;
mod config;
mod encoding;
mod notion;
mod scraper;
mod utils;

use clap::Parser;
use cli::{Cli, Command, DbCommand};
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
        Command::Db(db_args) => {
            let cmd = &db_args.command;
            match cmd {
                DbCommand::Create => {
                    let cfg = load_config().unwrap();
                    notion::create_database(cfg).await.unwrap();
                }
            }
        }
    }
}
