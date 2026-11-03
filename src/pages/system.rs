use crate::{backends::SystemInfo, components::*, i18n::tr};
use gtk::prelude::*;
pub fn build() -> (gtk::ScrolledWindow, gtk::Box) {
    let (s, b) = page(
        "Sistema",
        "Informações do dispositivo, sistema operacional e sessão gráfica.",
    );
    let info = gtk::Box::new(gtk::Orientation::Vertical, 20);
    info.append(&label("Lendo informações do sistema…", "secondary"));
    b.append(&info);
    super::integrations::append(&b, "system");
    (s, info)
}
pub fn populate(body: &gtk::Box, info: &SystemInfo) {
    if let Some(last) = body.last_child() {
        body.remove(&last);
    }
    let card = card();
    for (title, value) in [
        ("Nome do dispositivo", &info.hostname),
        ("Distribuição", &info.distribution),
        ("Kernel", &info.kernel),
        ("Processador", &info.cpu),
        ("Memória total", &info.memory),
        ("Sessão gráfica", &info.session),
        ("Desktop", &info.desktop),
    ] {
        let value = if value.is_empty() {
            tr("Não disponível")
        } else {
            value.clone()
        };
        card.append(&row(title, &value, "system"));
    }
    body.append(&card);
    body.append(&label(&info.integration_title, "section-title"));
    body.append(&label(&info.integration, "secondary"));
    let card = crate::components::card();
    for (key, value) in &info.settings {
        card.append(&row(key, value, "personalization"));
    }
    body.append(&card);
}
