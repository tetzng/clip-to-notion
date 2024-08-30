use crate::config::Config;
use crate::scraper::fetch_title_and_ogp;
use anyhow::{Context, Result};
use serde_json::json;
use std::collections::HashMap;

fn build_headers(notion_api_key: &str) -> Result<reqwest::header::HeaderMap> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Notion-Version", "2022-06-28".parse()?);
    headers.insert("Content-Type", "application/json".parse()?);
    headers.insert(
        "Authorization",
        format!("Bearer {}", notion_api_key).parse()?,
    );
    Ok(headers)
}

fn build_notion_properties(
    title: &str,
    url: &str,
    tags: &[String],
    database_id: &str,
    ogp_data: &HashMap<String, String>,
) -> serde_json::Value {
    let tags_json: Vec<_> = tags.iter().map(|tag| json!({ "name": tag })).collect();
    let default_description = "".to_string();
    let default_url = url.to_string();
    let description = ogp_data
        .get("og:description")
        .unwrap_or(&default_description);

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
                "url": ogp_data.get("og:url").unwrap_or(&default_url)
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
    properties
}

pub async fn post_to_notion(cfg: Config, url: &str, tags: &[String]) -> Result<()> {
    let (title, ogp_data) = fetch_title_and_ogp(url)
        .await
        .context("Failed to fetch title and OGP data")?;
    println!("Title: {}", title);
    println!("OGP data: {:?}", ogp_data);

    let client = reqwest::Client::builder().build()?;
    let headers = build_headers(&cfg.notion_api_key)?;

    let database_id = cfg.database_id;
    let properties = build_notion_properties(&title, url, tags, &database_id, &ogp_data);

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

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
    use std::collections::HashMap;

    #[test]
    fn test_build_headers() {
        let notion_api_key = "test_api_key";
        let headers = build_headers(notion_api_key).unwrap();

        assert_eq!(headers.get("Notion-Version").unwrap(), "2022-06-28");
        assert_eq!(headers.get(CONTENT_TYPE).unwrap(), "application/json");
        assert_eq!(
            headers.get(AUTHORIZATION).unwrap(),
            &HeaderValue::from_str("Bearer test_api_key").unwrap()
        );
    }

    #[test]
    fn test_build_notion_properties() {
        let title = "Test Title";
        let url = "https://example.com";
        let tags = vec!["tag1".to_string(), "tag2".to_string()];
        let database_id = "test_db_id";
        let mut ogp_data = HashMap::new();
        ogp_data.insert("og:description".to_string(), "Test Description".to_string());
        ogp_data.insert(
            "og:image".to_string(),
            "https://example.com/image.png".to_string(),
        );

        let properties = build_notion_properties(title, url, &tags, database_id, &ogp_data);

        assert_eq!(
            properties["properties"]["Name"]["title"][0]["text"]["content"],
            title
        );

        assert_eq!(properties["properties"]["URL"]["url"], url);

        assert_eq!(
            properties["properties"]["Tags"]["multi_select"],
            json!([{ "name": "tag1" }, { "name": "tag2" }])
        );

        let description = "Test Description";
        assert_eq!(
            properties["properties"]["Description"]["rich_text"][0]["text"]["content"].as_str(),
            Some(description)
        );

        assert_eq!(properties["cover"]["external"]["url"], ogp_data["og:image"]);
    }
}
