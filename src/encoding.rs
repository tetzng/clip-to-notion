use scraper::{Html, Selector};

pub fn detect_charset(document: &Html) -> Option<String> {
    document
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
        .next()
}
