use crate::encoding::detect_charset;
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

    let charset_meta = detect_charset(&document);

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

    let title = extract_title(&document)?;
    let ogp_data = extract_ogp_data(&document)?;

    Ok((title, ogp_data))
}

fn extract_title(document: &Html) -> Result<String> {
    let title_selector = Selector::parse("title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .map(|el| el.inner_html())
        .unwrap_or_default();
    Ok(title)
}

fn extract_ogp_data(document: &Html) -> Result<HashMap<String, String>> {
    let meta_selector = Selector::parse(r#"meta[property^="og:"]"#).unwrap();
    let mut ogp_data = HashMap::new();
    for meta in document.select(&meta_selector) {
        if let Some(property) = meta.value().attr("property") {
            if let Some(content) = meta.value().attr("content") {
                ogp_data.insert(property.to_string(), content.to_string());
            }
        }
    }
    Ok(ogp_data)
}
