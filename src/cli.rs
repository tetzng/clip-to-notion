use clap::{Args, Parser, Subcommand};

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
    Db(DbArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct DbArgs {
    #[command(subcommand)]
    pub command: DbCommand,
}

#[derive(Subcommand, Debug)]
pub enum DbCommand {
    Create,
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

    #[test]
    fn test_cli_parse_db_create() {
        let args = vec!["prog", "db", "create"];
        let cli = Cli::parse_from(args);
        if let Command::Db(DbArgs { command }) = cli.command {
            assert!(matches!(command, DbCommand::Create));
        } else {
            panic!("Expected Command::Db");
        }
    }
}
