use crate::{components::*, i18n::tr};
use gtk::prelude::*;
pub fn build(stack: &gtk::Stack) -> (gtk::ScrolledWindow, gtk::Label, gtk::Label) {
    let (scroll, body) = page(
        "Início",
        "Visão geral do dispositivo e acesso às configurações.",
    );
    let hero = gtk::Box::new(gtk::Orientation::Horizontal, 24);
    hero.add_css_class("hero");
    hero.append(&icon("system", 56));
    let text = gtk::Box::new(gtk::Orientation::Vertical, 8);
    let host = label("Carregando informações…", "hero-title");
    let summary = label("Consultando o sistema", "secondary");
    text.append(&host);
    text.append(&summary);
    hero.append(&text);
    body.append(&hero);
    body.append(&label("Configurações principais", "section-title"));
    let card = card();
    for (title, desc, image, id) in [
        (
            "Aparência da interface",
            "Tema, cores e tipografia",
            "personalization",
            "personalization",
        ),
        (
            "Acesso à organização",
            "Contas e ingresso no Active Directory",
            "accounts",
            "accounts",
        ),
        (
            "Informações do dispositivo",
            "Hardware, sistema e sessão gráfica",
            "system",
            "system",
        ),
    ] {
        let b = gtk::Button::new();
        b.set_child(Some(&row(title, desc, image)));
        b.set_tooltip_text(Some(&tr(title)));
        let weak = stack.downgrade();
        b.connect_clicked(move |_| {
            if let Some(s) = weak.upgrade() {
                s.set_visible_child_name(id)
            }
        });
        card.append(&b);
    }
    body.append(&card);
    (scroll, host, summary)
}
