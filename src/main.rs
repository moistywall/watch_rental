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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut counts: Vec<usize> = Vec::new();
    counts.push(get_homes_chintai("https://www.homes.co.jp/archive/b-41726737/").unwrap());
    counts.push(get_suumo_chintai("https://suumo.jp/library/tf_13/sc_13117/to_1002638814/").unwrap());
    counts.push(get_suumo_chintai("https://suumo.jp/library/tf_13/sc_13117/to_1001612572/").unwrap());
    println!("{:?}", counts);

    Ok(())
}
