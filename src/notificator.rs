use crate::{
    crawler::Scraper,
    url_store::{SiteType, UrlStore}
};
use dotenvy::dotenv;
use reqwest::blocking::Client;
use serde::Serialize;
use std::{thread, time, env};

const ERROR_HOOK: &str = "DISCORD_WEBHOOK_ERROR_URL";
const SUUMO_CRAWLER_HOOK: &str = "DISCORD_WEBHOOK_SUUMO_URL";
const HOMES_CRAWLER_HOOK: &str = "DISCORD_WEBHOOK_HOMES_URL";

#[derive(Serialize)]
struct DiscordWebhook<'a> {
    content: &'a str,
}

pub struct NotificationInfo {
    site: SiteType
}

impl NotificationInfo {
    pub fn new(site: SiteType) -> Self {
        Self { site }
    }

    pub fn run_notificate(&self) {
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

        let _ = self.notificate(crawl_results.join("\n"));
    }

    fn notificate(&self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("{:?}", message);
        let hook_url = self.get_env_data();
        let client = Client::new();
        let payload = DiscordWebhook { content: &message };

        println!("{:?}", hook_url);

        let res = client
            .post(hook_url)
            .json(&payload)
            .send()?;

        if res.status().is_success() {
            println!("通知を送信しました．")
        } else {
            println!("通知に失敗しました． status: {}", res.status());
        }

        Ok(())
    }

    fn get_env_data(&self) -> String {
        dotenv().ok();
        match self.site {
            SiteType::Homes => env::var(HOMES_CRAWLER_HOOK).expect("webhook URL not found"),
            SiteType::Suumo => env::var(SUUMO_CRAWLER_HOOK).expect("webhook URL not found"),
        }
    }
}
