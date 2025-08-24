use crate::{
    crawler::Scraper,
    url_store::{SiteType, UrlStore}
};
use serde::Serialize;
use std::{thread, time};

// const ERROR_HOOK: &str = "DISCORD_WEBHOOK_ERROR_URL"; // 後で、エラー時の送信メッセージを作成する
const SUUMO_CRAWLER_HOOK: &str = "DISCORD_WEBHOOK_SUUMO_URL";
const HOMES_CRAWLER_HOOK: &str = "DISCORD_WEBHOOK_HOMES_URL";

#[derive(Serialize)]
struct DiscordWebhook<'a> {
    content: &'a str,
}

pub struct Notifier {
    site: SiteType,
    client: reqwest::blocking::Client,
    webhook: String,
}

impl Notifier {
    pub fn new(site: SiteType) -> Self {
        dotenvy::dotenv().ok();
        let webhook = match site {
            SiteType::Homes => std::env::var(HOMES_CRAWLER_HOOK).expect("..."),
            SiteType::Suumo => std::env::var(SUUMO_CRAWLER_HOOK).expect("..."),
        };
        let client = reqwest::blocking::Client::new();
        Self {
            site,
            client,
            webhook,
        }
    }

    pub fn send_notifications(&self) {
        let url_store = UrlStore::new(self.site.clone());
        let urls = url_store.get_urls();
        let scraper = Scraper::new(self.site.clone());
        let mut crawl_results: Vec<String> = Vec::new();
        let five_sec = time::Duration::from_secs(5);
        for url in urls {
            match scraper.scrape_url(&url) {
                Ok(result) => {
                    match result.format_info() {
                        Some(res) => {
                            crawl_results.push(res);
                        }
                        None => {
                            
                        }
                    }
                }
                Err(e) => {
                    crawl_results.push(e.to_string());
                }
            }
            thread::sleep(five_sec);
        }

        let _ = self.send_messages(&crawl_results);
    }

    fn send_messages(&self, lines: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        const LIMIT: usize = 2000;
        let text = lines.join("\n");
        for chunk in Self::split_discord(&text, LIMIT) {
            let payload = DiscordWebhook { content: &chunk };
            let res = self.client.post(&self.webhook).json(&payload).send()?;
            if !res.status().is_success() {
                eprintln!("Discord送信失敗: {}", res.status());
            }
        }

        Ok(())
    }

    fn split_discord(s: &str, limit: usize) -> Vec<String> {
        s.chars().collect::<Vec<_>>()
            .chunks(limit).map(|c| c.iter().collect()).collect()
    }
}
