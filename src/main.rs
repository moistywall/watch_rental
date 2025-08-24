mod url_store;
mod crawler;
mod notificator;
use notificator::Notifier;

use crate::url_store::SiteType;

fn main() {
    let suumo_notificator = Notifier::new(SiteType::Suumo);
    let homes_notificator = Notifier::new(SiteType::Homes);
    suumo_notificator.send_notifications();
    homes_notificator.send_notifications();
}
