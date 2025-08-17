use crate::crawler;


struct NotificationInfo<'a> {
    discord_webhook_url: &'a str,
    crawl_result: CrawlResult,
}

