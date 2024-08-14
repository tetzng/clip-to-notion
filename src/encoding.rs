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

#[cfg(test)]
mod tests {
    use super::*;
    use scraper::Html;

    #[test]
    fn test_detect_charset_with_meta_charset() {
        let html = r#"<meta charset="shift_jis">"#;
        let document = Html::parse_document(html);
        let charset = detect_charset(&document);
        assert_eq!(charset, Some("shift-jis".to_string()));
    }

    #[test]
    fn test_detect_charset_with_http_equiv() {
        let html = r#"<meta http-equiv="content-type" content="text/html; charset=shift_jis">"#;
        let document = Html::parse_document(html);
        let charset = detect_charset(&document);
        assert_eq!(charset, Some("shift-jis".to_string()));
    }

    #[test]
    fn test_detect_charset_none() {
        let html = r#"<meta charset="utf-8">"#;
        let document = Html::parse_document(html);
        let charset = detect_charset(&document);
        assert_eq!(charset, None);
    }
}
