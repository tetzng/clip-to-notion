use crate::utils::read_input;
use anyhow::{Context, Result};
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub database_id: String,
    pub notion_api_key: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_id: "".into(),
            notion_api_key: "".into(),
        }
    }
}

pub fn load_config() -> Result<Config> {
    let user_dirs = UserDirs::new().context("Could not determine user directories")?;
    let config_dir = user_dirs
        .home_dir()
        .join(".config/clip-to-notion/config.toml");

    if !config_dir.exists() {
        eprintln!("Config file not found. Please run `init` command to create it.");
        std::process::exit(1);
    }

    let config_content = fs::read_to_string(&config_dir)
        .with_context(|| format!("Could not read configuration file at {:?}", config_dir))?;

    let config: Config = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse configuration file at {:?}", config_dir))?;

    Ok(config)
}

pub fn init_config() -> Result<()> {
    let user_dirs = UserDirs::new().context("Could not determine user directories")?;
    let config_dir = user_dirs.home_dir().join(".config/clip-to-notion");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).context("Failed to create configuration directory")?;
    }

    let config_path = config_dir.join("config.toml");

    if config_path.exists() {
        eprintln!("Warning: Config file already exists at: {:?}", config_path);
        println!("Do you want to overwrite it? (y/n): ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .context("Failed to read input")?;
        let input = input.trim().to_lowercase();

        if input != "y" {
            println!("Aborting initialization.");
            return Ok(());
        }
    }

    println!("Enter your Notion API key:");
    let notion_api_key = read_input("API Key")?;

    println!("Enter your Notion Database ID:");
    let database_id = read_input("Database ID")?;

    let config = Config {
        notion_api_key,
        database_id,
    };

    let toml_content = toml::to_string(&config).context("Failed to serialize configuration")?;
    fs::write(&config_path, toml_content).context("Failed to write configuration file")?;

    println!("Configuration file created at: {:?}", config_path);

    Ok(())
}
