use crate::{components::*, i18n::tr, state::Preferences, theme};
use gtk::prelude::*;
use std::{cell::RefCell, rc::Rc};
type State = Rc<RefCell<Preferences>>;
fn update(state: &State, provider: &gtk::CssProvider, status: &gtk::Label) {
    theme::apply(provider, &state.borrow());
    status.set_text(&match state.borrow().save() {
        Ok(_) => tr("Preferências do aplicativo salvas."),
        Err(e) => format!("{}: {e}", tr("Não foi possível salvar")),
    });
}
pub fn build(state: State, provider: gtk::CssProvider) -> gtk::ScrolledWindow {
    let (scroll, body) = page("Personalização", "Tema, cores e tipografia da interface.");
    let preview = gtk::Box::new(gtk::Orientation::Horizontal, 16);
    preview.add_css_class("preview");
    let side = gtk::Box::new(gtk::Orientation::Vertical, 12);
    side.add_css_class("preview-sidebar");
    side.append(&icon("personalization", 30));
    let wordmark = gtk::Label::new(Some("monolito"));
    wordmark.add_css_class("brand");
    side.append(&wordmark);
    let content = gtk::Box::new(gtk::Orientation::Vertical, 12);
    content.add_css_class("preview-content");
    content.set_hexpand(true);
    content.append(&label("Prévia da interface", "hero-title"));
    content.append(&label(
        "Visualização do tema e da cor de destaque selecionados.",
        "secondary",
    ));
    let accent = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    accent.add_css_class("preview-accent");
    content.append(&accent);
    preview.append(&side);
    preview.append(&content);
    body.append(&preview);
    body.append(&label("Aparência", "section-title"));
    let status = label(
        "Estas preferências afetam somente o Monolito Settings.",
        "secondary",
    );
    let card = card();
    let r = row(
        "Tema da interface",
        "Claro, escuro ou conforme a preferência GTK da sessão",
        "personalization",
    );
    let group = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let mut first: Option<gtk::ToggleButton> = None;
    for (id, text) in [
        ("light", "Claro"),
        ("dark", "Escuro"),
        ("auto", "Automático"),
    ] {
        let b = gtk::ToggleButton::with_label(&tr(text));
        b.add_css_class("choice");
        if let Some(ref f) = first {
            b.set_group(Some(f));
        } else {
            first = Some(b.clone());
        }
        b.set_active(state.borrow().theme == id);
        let (s, p, t) = (state.clone(), provider.clone(), status.clone());
        b.connect_toggled(move |b| {
            if b.is_active() {
                s.borrow_mut().theme = id.into();
                update(&s, &p, &t)
            }
        });
        group.append(&b);
    }
    let theme_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
    theme_box.append(&r);
    group.set_margin_start(20);
    group.set_margin_bottom(16);
    theme_box.append(&group);
    card.append(&theme_box);
    let r = row(
        "Cor de destaque",
        "Cor aplicada aos itens selecionados e às ações principais",
        "personalization",
    );
    let colors = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let mut first: Option<gtk::ToggleButton> = None;
    for (id, text) in [
        ("blue", "Azul"),
        ("violet", "Violeta"),
        ("green", "Verde"),
        ("terracotta", "Terracota"),
    ] {
        let b = gtk::ToggleButton::new();
        b.add_css_class("swatch");
        b.add_css_class(id);
        accessible(&b, text);
        b.set_tooltip_text(Some(&tr(text)));
        if let Some(ref f) = first {
            b.set_group(Some(f));
        } else {
            first = Some(b.clone());
        }
        b.set_active(state.borrow().accent == id);
        let (s, p, t) = (state.clone(), provider.clone(), status.clone());
        b.connect_toggled(move |b| {
            if b.is_active() {
                s.borrow_mut().accent = id.into();
                update(&s, &p, &t)
            }
        });
        colors.append(&b);
    }
    r.append(&colors);
    card.append(&r);
    body.append(&card);
    let card = crate::components::card();
    let r = row(
        "Fonte da interface",
        "Fonte utilizada nos textos e controles",
        "personalization",
    );
    let font = gtk::FontButton::with_font(&state.borrow().font);
    font.set_use_font(false);
    font.set_valign(gtk::Align::Center);
    let (s, p, t) = (state.clone(), provider.clone(), status.clone());
    font.connect_font_set(move |b| {
        s.borrow_mut().font = b.font().unwrap_or_default().into();
        update(&s, &p, &t)
    });
    r.append(&font);
    card.append(&r);
    let r = row(
        "Tema de ícones",
        "Os ícones de navegação mantêm a identidade Monolito",
        "devices",
    );
    let entry = gtk::Entry::builder()
        .text(&state.borrow().icons)
        .width_chars(12)
        .valign(gtk::Align::Center)
        .build();
    accessible(&entry, "Nome do tema de ícones");
    let apply = button("Aplicar");
    let (s, p, t) = (state.clone(), provider.clone(), status.clone());
    let e = entry.clone();
    apply.connect_clicked(move |_| {
        let name = e.text();
        let paths = [
            std::path::PathBuf::from("/usr/share/icons"),
            gtk::glib::user_data_dir().join("icons"),
            gtk::glib::home_dir().join(".icons"),
        ];
        if name.is_empty()
            || name.contains('/')
            || !paths
                .iter()
                .any(|p| p.join(name.as_str()).join("index.theme").is_file())
        {
            t.set_text(&tr("Tema de ícones não encontrado nas pastas padrão."));
            return;
        }
        s.borrow_mut().icons = name.into();
        update(&s, &p, &t)
    });
    r.append(&entry);
    r.append(&apply);
    card.append(&r);
    body.append(&card);
    body.append(&status);
    scroll
}
