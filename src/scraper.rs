use anyhow::{Context, Result};
use reqwest;
use scraper::{Html, Selector};
use std::collections::HashMap;

pub async fn fetch_title_and_ogp(url: &str) -> Result<(String, HashMap<String, String>)> {
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
