use std::collections::HashMap;
use std::fs;

const SUUMO_URLS_FILE: &str = "suumo_watcher.txt";
const HOMES_URLS_FILE: &str = "homes_watcher.txt";

#[derive(Debug, Eq, PartialEq, Hash)]
enum SiteType {
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
struct UrlStore {
    urls: HashMap<SiteType, UrlList>,
}

impl UrlStore {
    fn new() -> Self {
        let kinds = [SiteType::Suumo, SiteType::Homes];

        let urls = kinds
            .into_iter()
            .map(|kind| {
                let list = Self::load_urls_from_file(kind.file_name());
                (kind, list)
            })
            .collect();

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
}

fn main() {
    let url_store = UrlStore::new();
    println!("{:#?}", url_store);
}
