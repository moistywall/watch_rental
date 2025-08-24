use std::fs;

const SUUMO_URLS_FILE: &str = "suumo_watcher.txt";
const HOMES_URLS_FILE: &str = "homes_watcher.txt";

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SiteType {
    Suumo,
    Homes,
}

impl SiteType {
    fn file_name(&self) -> &'static str {
        match self {
            SiteType::Suumo => SUUMO_URLS_FILE,
            SiteType::Homes => HOMES_URLS_FILE,
        }
    }
}

type UrlList = Vec<String>;

#[derive(Debug)]
pub struct UrlStore {
    urls: UrlList,
}

impl UrlStore {
    pub fn new(site: SiteType) -> Self {
        let urls = Self::load_urls_from_file(site.file_name());
        Self { urls }
    }

    fn load_urls_from_file(file_name: &str) -> UrlList {
        fs::read_to_string(file_name)
            .expect("ファイル読み込み中に問題が発生しました．")
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    pub fn get_urls(&self) -> UrlList {
        self.urls.clone()
    }
}