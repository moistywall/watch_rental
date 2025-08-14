mod url_store;
mod crawler;
use crate::url_store::{SiteType, UrlStore};

fn main() {
    let url_store = UrlStore::new();
    println!("{:#?}", url_store.get_urls(&SiteType::Suumo));
    println!("{:#?}", url_store.get_urls(&SiteType::Homes));
}
