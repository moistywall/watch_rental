use std::collections::HashMap;
use crate::url_store::{SiteType, UrlStore};

type RoomInfo = Vec<(String, Option<usize>)>;

struct SearchTarget {
    room_info: RoomInfo,
}

impl SearchTarget {
    fn new(site: &SiteType) -> Self {
        let url_stroe = UrlStore::new();
        let urls = url_stroe.get_urls(&site).unwrap().clone();
        Self { room_info: urls.iter().map(|url| (url.clone(), None)).collect::<RoomInfo>() }
    }
}