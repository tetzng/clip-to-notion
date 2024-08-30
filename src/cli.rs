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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_init() {
        let args = vec!["prog", "init"];
        let cli = Cli::parse_from(args);
        assert!(matches!(cli.command, Command::Init));
    }

    #[test]
    fn test_cli_parse_run() {
        let args = vec![
            "prog",
            "run",
            "https://example.com",
            "--tags",
            "test,example",
        ];
        let cli = Cli::parse_from(args);
        if let Command::Run { url, tags } = cli.command {
            assert_eq!(url, "https://example.com");
            assert_eq!(tags, vec!["test", "example"]);
        } else {
            panic!("Expected Command::Run");
        }
    }
}
