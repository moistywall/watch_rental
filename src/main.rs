mod url_store;
mod crawler;
mod notificator;

use notificator::Notifier;
use crate::url_store::SiteType;
use std::thread;

fn main() {
    let suumo_handle = thread::spawn(|| {
        let suumo_notificator = Notifier::new(SiteType::Suumo);
        suumo_notificator.send_notifications();
    });

    let homes_handle = thread::spawn(|| {
        let homes_notificator = Notifier::new(SiteType::Homes);
        homes_notificator.send_notifications();
    });

    suumo_handle.join().expect("Summo thread panicked");
    homes_handle.join().expect("Homes thread panicked");
}
