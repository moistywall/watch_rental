mod url_store;
mod crawler;
mod notificator;
use notificator::NotificationInfo;

use crate::url_store::SiteType;

fn main() {
    let suumo_notificator = NotificationInfo::new(SiteType::Suumo);
    let homes_notificator = NotificationInfo::new(SiteType::Homes);
    suumo_notificator.run_notificate();
    homes_notificator.run_notificate();
}
