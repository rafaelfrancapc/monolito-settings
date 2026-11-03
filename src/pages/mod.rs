pub mod accounts;
pub mod home;
pub mod integrations;
pub mod personalization;
pub mod system;
use crate::{components::*, navigation::Category};
pub fn settings_page(c: &Category) -> gtk::ScrolledWindow {
    let (scroll, body) = page(c.title, c.description);
    integrations::append(&body, c.id);
    scroll
}
