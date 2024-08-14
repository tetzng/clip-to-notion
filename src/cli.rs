use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init,
    Run {
        url: String,

        #[arg(short, long, use_value_delimiter = true)]
        tags: Vec<String>,
    },
}
