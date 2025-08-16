use crate::url_store::SiteType;
use std::fmt;

#[derive(Debug)]
pub enum ScraperError {
    InvalidUrl(String),
    RequestFailed(String, reqwest::Error),
    ParseError(String),
}

impl fmt::Display for ScraperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScraperError::InvalidUrl(url) => write!(f, "無効なURLです: {}", url),
            ScraperError::RequestFailed(url, e) => write!(f, "リクエスト失敗 ({}): {}", url, e),
            ScraperError::ParseError(msg) => write!(f, "スクレイピング失敗: {}", msg),
        }
    }
}

/// 1件のスクレイピング結果
#[derive(Debug)]
pub struct CrawlResult {
    url: String,                    // スクレイプ対象 URL
    room_count: Option<usize>,      // Some(>0): 部屋数あり，Some(0) or None: 空き部屋なし
    property_name: Option<String>,  // 拡張用
    city: Option<String>,           // 拡張用
}

struct Scraper {
    site: SiteType,
    selector_str: &'static str,
}

impl Scraper {
    pub fn new(site: SiteType) -> Self {
        let selector_str = match site {
            SiteType::Homes => "p.text-sm.mt-2 > span",
            SiteType::Suumo => "p > span.fs13",
        };

        Self {
            site,
            selector_str,
        }
    }

    pub fn scrape_url(&self, url: &str) -> Result<CrawlResult, ScraperError> {
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

        let mut room_count = None;

        for e in document.select(&selector) {
            if let Some(text) = e.text().next() {
                let digit_only: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
                if let Ok(count) = digit_only.parse::<usize>() {
                    room_count = Some(count);
                    break;
                }
            }
        }

        Ok(CrawlResult {
            url: url.to_string(),
            room_count,
            property_name: None,
            city: None,
        })
    }
 }
