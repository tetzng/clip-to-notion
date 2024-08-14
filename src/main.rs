use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use directories::UserDirs;
use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, fs, io, io::Write};

#[derive(Serialize, Deserialize, Debug)]
struct MyConfig {
    database_id: String,
    notion_api_key: String,
}

impl Default for MyConfig {
    fn default() -> Self {
        Self {
            database_id: "".into(),
            notion_api_key: "".into(),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init,
    Run {
        url: String,

        #[arg(short, long, use_value_delimiter = true)]
        tags: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Init => {
            init_config()?;
        }
        Command::Run { url, tags } => {
            let cfg = load_config()?;

            let (title, ogp_data) = fetch_title_and_ogp(url)
                .await
                .context("Failed to fetch title and OGP data")?;

            let client = reqwest::Client::builder().build()?;

            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Notion-Version", "2022-06-28".parse()?);
            headers.insert("Content-Type", "application/json".parse()?);
            headers.insert(
                "Authorization",
                format!("Bearer {}", cfg.notion_api_key).parse()?,
            );

            let tags_json: Vec<_> = tags.iter().map(|tag| json!({ "name": tag })).collect();
            let database_id = cfg.database_id;
            let description = ogp_data.get("og:description").unwrap();

            let mut properties = json!({
                "parent": {
                    "database_id": database_id
                },
                "properties": {
                    "Name": {
                        "title": [
                            {
                                "text": {
                                    "content": title
                                }
                            }
                        ]
                    },
                    "URL": {
                        "url": ogp_data.get("og:url").unwrap_or(url)
                    },
                    "Tags": {
                        "multi_select": tags_json
                    },
                    "Description": {
                        "rich_text": [
                            {
                                "type": "text",
                                "text": {
                                    "content": description,
                                    "link": null,
                                }
                            }
                        ]
                    },
                }
            });

            if let Some(image_url) = ogp_data.get("og:image") {
                properties.as_object_mut().unwrap().insert(
                    "cover".to_string(),
                    json!({
                        "external": {
                            "url": image_url
                        }
                    }),
                );
            }

            let response = client
                .post("https://api.notion.com/v1/pages")
                .headers(headers)
                .json(&properties)
                .send()
                .await
                .context("Failed to send request to Notion API")?;

            let body = response
                .text()
                .await
                .context("Failed to read response body")?;
            println!("{}", body);
        }
    }

    Ok(())
}

async fn fetch_title_and_ogp(url: &str) -> Result<(String, HashMap<String, String>)> {
    let response = reqwest::get(url)
        .await
        .context("Failed to perform GET request")?;

    let body = response
        .text()
        .await
        .context("Failed to read response body")?;
    let document = Html::parse_document(&body);

    let charset_meta = document
        .select(&Selector::parse(r#"meta[http-equiv="content-type"], meta[charset]"#).unwrap())
        .filter_map(|meta| {
            if let Some(content) = meta.value().attr("content") {
                if content.to_lowercase().contains("shift_jis")
                    || content.to_lowercase().contains("sjis")
                {
                    return Some("shift-jis".to_string());
                }
            }
            if let Some(charset) = meta.value().attr("charset") {
                if charset.to_lowercase().contains("shift_jis")
                    || charset.to_lowercase().contains("sjis")
                {
                    return Some("shift-jis".to_string());
                }
            }
            None
        })
        .next();

    let body = if let Some(charset) = charset_meta {
        reqwest::get(url)
            .await
            .context("Failed to perform GET request")?
            .text_with_charset(&charset)
            .await
            .context("Failed to read response body with shift-jis charset")?
    } else {
        body
    };

    let document = Html::parse_document(&body);

    let title_selector = Selector::parse("title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .map(|el| el.inner_html())
        .unwrap_or_default();

    let meta_selector = Selector::parse(r#"meta[property^="og:"]"#).unwrap();
    let mut ogp_data = HashMap::new();
    for meta in document.select(&meta_selector) {
        if let Some(property) = meta.value().attr("property") {
            if let Some(content) = meta.value().attr("content") {
                ogp_data.insert(property.to_string(), content.to_string());
            }
        }
    }

    Ok((title, ogp_data))
}

fn load_config() -> Result<MyConfig> {
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

    let config: MyConfig = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse configuration file at {:?}", config_dir))?;

    Ok(config)
}

fn init_config() -> Result<()> {
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

    let config = MyConfig {
        notion_api_key,
        database_id,
    };

    let toml_content = toml::to_string(&config).context("Failed to serialize configuration")?;
    fs::write(&config_path, toml_content).context("Failed to write configuration file")?;

    println!("Configuration file created at: {:?}", config_path);

    Ok(())
}

fn read_input(prompt: &str) -> Result<String> {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("Failed to read input")?;

    Ok(input.trim().to_string())
}
