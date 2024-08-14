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

#[cfg(test)]
mod tests {
    use super::*;
    use scraper::Html;

    #[tokio::test]
    async fn test_fetch_title_and_ogp() {
        let mut server = mockito::Server::new_async().await;

        let _mock = server
            .mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body(
                r#"
                <html>
                    <head>
                        <title>Example Title</title>
                        <meta property="og:description" content="This is a description.">
                        <meta property="og:image" content="https://example.com/image.png">
                    </head>
                </html>"#,
            )
            .create_async()
            .await;

        let result = fetch_title_and_ogp(&server.url().as_str()).await;
        assert!(result.is_ok());
        let (title, ogp_data) = result.unwrap();
        assert_eq!(title, "Example Title");
        assert_eq!(
            ogp_data.get("og:description").unwrap(),
            "This is a description."
        );
    }

    #[test]
    fn test_extract_title() {
        let html = r#"<html><head><title>Test Title</title></head></html>"#;
        let document = Html::parse_document(html);
        let title = extract_title(&document).unwrap();
        assert_eq!(title, "Test Title");
    }

    #[test]
    fn test_extract_title_empty() {
        let html = r#"<html><head><title></title></head></html>"#;
        let document = Html::parse_document(html);
        let title = extract_title(&document).unwrap();
        assert_eq!(title, "");
    }

    #[test]
    fn test_extract_title_missing() {
        let html = r#"<html><head></head></html>"#;
        let document = Html::parse_document(html);
        let title = extract_title(&document).unwrap();
        assert_eq!(title, "");
    }

    #[test]
    fn test_extract_ogp_data() {
        let html = r#"
            <meta property="og:title" content="OGP Title">
            <meta property="og:description" content="OGP Description">
        "#;
        let document = Html::parse_document(html);
        let ogp_data = extract_ogp_data(&document).unwrap();
        assert_eq!(ogp_data.get("og:title").unwrap(), "OGP Title");
        assert_eq!(ogp_data.get("og:description").unwrap(), "OGP Description");
    }

    #[test]
    fn test_extract_ogp_data_missing() {
        let html = r#"<html><head></head></html>"#;
        let document = Html::parse_document(html);
        let ogp_data = extract_ogp_data(&document).unwrap();
        assert!(ogp_data.is_empty());
    }
}
