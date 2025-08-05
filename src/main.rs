use reqwest::blocking::Client;
use serde::Serialize;

use dotenvy::dotenv;
use std::env;

fn get_suumo_chintai(url: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    let document = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("p > span.fs13").unwrap();
    let elements = document.select(&selector);
    for e in elements {
        if let Some(text) = e.text().next() {
            let digits_only: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
            if let Ok(count) = digits_only.parse::<usize>() {
                return Ok(count);
            }
        }
    }

    Err("件数の取得に失敗しました".into())
}

fn get_homes_chintai(url: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    let document = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("p.text-sm.mt-2 > span").unwrap();
    let elements = document.select(&selector);
    for e in elements {
        if let Some(text) = e.text().next() {
            let digits_only: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
            if let Ok(count) = digits_only.parse::<usize>() {
                return Ok(count);
            }
        }
    }

    Err("件数の取得に失敗しました".into())
}

#[derive(Serialize)]
struct DiscordWebhook<'a> {
    content: &'a str,
}

fn send_discord_notification(webhook_url: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let payload = DiscordWebhook { content: message };

    let res = client
        .post(webhook_url)
        .json(&payload)
        .send()?;

    if res.status().is_success() {
        println!("通知を送信しました．")
    } else {
        println!("通知に失敗しました． status: {}", res.status());
    }

    Ok(())
}

fn get_env_data() -> String {
    dotenv().ok();
    env::var("DISCORD_WEBHOOK_URL").expect("webhook URL not found")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let urls = vec![
    //     "https://www.homes.co.jp/archive/b-41726737/",
    //     "https://suumo.jp/library/tf_13/sc_13117/to_1002638814/",
    //     "https://suumo.jp/library/tf_13/sc_13117/to_1001612572/"
    // ];
    
    let mut counts: Vec<usize> = Vec::new();
    counts.push(get_homes_chintai("https://www.homes.co.jp/archive/b-41726737/").unwrap());
    counts.push(get_suumo_chintai("https://suumo.jp/library/tf_13/sc_13117/to_1002638814/").unwrap());
    counts.push(get_suumo_chintai("https://suumo.jp/library/tf_13/sc_13117/to_1001612572/").unwrap());
    println!("{:?}", counts);

    let webhook_url = get_env_data();
    let message = "賃貸情報テストメッセージ";
    send_discord_notification(&webhook_url, message)?;

    Ok(())
}
