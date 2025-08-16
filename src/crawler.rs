use crate::url_store::SiteType;
use std::fmt;

#[derive(Debug)]
enum ScraperError {
    InvalidUrl(String),
    RequestFailed(String, reqwest::Error),
    ParseError(String),
}

impl fmt::Display for ScraperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScraperError::InvalidUrl(url) => write!(f, "無効なURLです {}", url),
            ScraperError::RequestFailed(url, e) => write!(f, "リクエストに失敗しました ({}): {}", url, e),
            ScraperError::ParseError(msg) => write!(f, "スクレイピングに失敗しました: {}", msg),
        }
    }
}

type RoomInfo = Vec<(String, Option<usize>)>;

struct SearchTarget {
    site: SiteType,
    selector_str: &'static str,
}

impl SearchTarget {
    fn new(site: SiteType) -> Self {
        let selector_str = match site {
            SiteType::Homes => "p.text-sm.mt-2 > span",
            SiteType::Suumo => "p > span.fs13",
        };

        Self {
            site,
            selector_str,
        }
    }

    fn get_number_of_rent(&self, url: &str) -> Result<Option<usize>, ScraperError> {
        if url.is_empty() || !url.starts_with("http") {
            return Err(ScraperError::InvalidUrl(url.to_string()));
        }

        let body = reqwest::blocking::get(url)
            .map_err(|e| ScraperError::RequestFailed(url.to_string(), e))?
            .text()
            .map_err(|e| ScraperError::RequestFailed(url.to_string(), e))?;

        let document = scraper::Html::parse_document(&body);
        let selector = scraper::Selector::parse(self.selector_str)
            .map_err(|_| ScraperError::ParseError("セレクタが不正".to_string()))?;

        let elements = document.select(&selector);
        for e in elements {
            if let Some(text) = e.text().next() {
                let digit_only: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
                if let Ok(count) = digit_only.parse::<usize>() {
                    return Ok(Some(count));
                }
            }
        }
        Ok(None)
    }
 }
