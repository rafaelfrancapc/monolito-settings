use super::{accessible, icon};
use gtk::prelude::*;
pub fn build(window: &gtk::ApplicationWindow) -> gtk::Box {
    let controls = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    for (name, title) in [
        ("minimize", "Minimizar"),
        ("maximize", "Maximizar ou restaurar"),
        ("close", "Fechar"),
    ] {
        let b = gtk::Button::new();
        b.set_child(Some(&icon(name, 16)));
        b.add_css_class("window-control");
        b.add_css_class(name);
        accessible(&b, title);
        b.set_tooltip_text(Some(&crate::i18n::tr(title)));
        let weak = window.downgrade();
        b.connect_clicked(move |_| {
            if let Some(w) = weak.upgrade() {
                match name {
                    "minimize" => w.minimize(),
                    "maximize" => {
                        if w.is_maximized() {
                            w.unmaximize()
                        } else {
                            w.maximize()
                        }
                    }
                    _ => w.close(),
                }
            }
        });
        if name == "maximize" {
            let weak = b.downgrade();
            window.connect_maximized_notify(move |w| {
                if let Some(b) = weak.upgrade() {
                    b.set_child(Some(&icon(
                        if w.is_maximized() {
                            "restore"
                        } else {
                            "maximize"
                        },
                        16,
                    )));
                }
            });
        }
        controls.append(&b);
    }
    controls
}
