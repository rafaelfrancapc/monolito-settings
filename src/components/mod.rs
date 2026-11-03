pub mod window_controls;
use crate::i18n::tr;
use gtk::prelude::*;
pub fn icon(name: &str, size: i32) -> gtk::Image {
    let file = gtk::gio::File::for_uri(&format!(
        "resource:///org/monolito/Settings/icons/{name}-symbolic.svg"
    ));
    let paintable = gtk::IconPaintable::for_file(&file, size, 1);
    let image = gtk::Image::from_paintable(Some(&paintable));
    image.set_pixel_size(size);
    image.set_halign(gtk::Align::Center);
    image.set_valign(gtk::Align::Center);
    image
}
pub fn label(text: &str, class: &str) -> gtk::Label {
    let l = gtk::Label::new(Some(&tr(text)));
    l.set_xalign(0.0);
    l.set_wrap(true);
    l.add_css_class(class);
    l
}
pub fn card() -> gtk::Box {
    let b = gtk::Box::new(gtk::Orientation::Vertical, 0);
    b.add_css_class("settings-card");
    b
}
pub fn row(title: &str, subtitle: &str, image: &str) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 16);
    row.add_css_class("settings-row");
    row.append(&icon(image, 24));
    let text = gtk::Box::new(gtk::Orientation::Vertical, 4);
    text.set_hexpand(true);
    text.append(&label(title, "row-title"));
    if !subtitle.is_empty() {
        text.append(&label(subtitle, "secondary"));
    }
    row.append(&text);
    row
}
pub fn system_row(title: &str, subtitle: &str, icon_name: &str) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 16);
    row.add_css_class("settings-row");
    row.append(&system_icon(icon_name, 24));
    let text = gtk::Box::new(gtk::Orientation::Vertical, 4);
    text.set_hexpand(true);
    text.append(&label(title, "row-title"));
    if !subtitle.is_empty() {
        text.append(&label(subtitle, "secondary"));
    }
    row.append(&text);
    row
}

pub fn system_icon(icon_name: &str, size: i32) -> gtk::Image {
    let fallback = match icon_name {
        "input-mouse-symbolic" => Some("mouse"),
        "input-keyboard-symbolic" => Some("keyboard"),
        "printer-symbolic" => Some("printer"),
        "drive-harddisk-symbolic" => Some("disk"),
        "applications-engineering-symbolic" => Some("hardware"),
        "preferences-color-symbolic" => Some("color"),
        "thunderbolt-symbolic" => Some("thunderbolt"),
        "bluetooth-symbolic" => Some("devices"),
        "video-display-symbolic" | "preferences-system-symbolic" => Some("system"),
        "network-wireless-symbolic" => Some("network"),
        "preferences-desktop-theme-symbolic" => Some("personalization"),
        "system-users-symbolic" => Some("accounts"),
        "preferences-desktop-locale-symbolic" | "x-office-calendar-symbolic" => Some("datetime"),
        "security-high-symbolic" => Some("security"),
        "system-software-update-symbolic" => Some("updates"),
        "application-x-executable-symbolic" => Some("applications"),
        "preferences-desktop-accessibility-symbolic" => Some("accessibility"),
        "preferences-system-notifications-symbolic" => Some("system"),
        _ => None,
    };
    if let (Some(display), Some(fallback)) = (gtk::gdk::Display::default(), fallback) {
        if !gtk::IconTheme::for_display(&display).has_icon(icon_name) {
            let image = icon(fallback, size);
            image.set_widget_name(icon_name);
            return image;
        }
    }
    let image = gtk::Image::from_icon_name(icon_name);
    image.set_widget_name(icon_name);
    image.set_pixel_size(size);
    image.set_halign(gtk::Align::Center);
    image.set_valign(gtk::Align::Center);
    image
}
pub fn page(title: &str, description: &str) -> (gtk::ScrolledWindow, gtk::Box) {
    let body = gtk::Box::new(gtk::Orientation::Vertical, 20);
    body.add_css_class("page");
    body.append(&label(title, "page-title"));
    body.append(&label(description, "secondary"));
    let scroll = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .hexpand(true)
        .vexpand(true)
        .child(&body)
        .build();
    (scroll, body)
}
pub fn accessible(w: &impl IsA<gtk::Accessible>, text: &str) {
    w.update_property(&[gtk::accessible::Property::Label(&tr(text))]);
}
pub fn button(text: &str) -> gtk::Button {
    let b = gtk::Button::with_label(&tr(text));
    b.set_valign(gtk::Align::Center);
    b
}
