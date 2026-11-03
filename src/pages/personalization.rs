use crate::{
    backends::{appearance::SystemAppearance, catalog},
    components::*,
    i18n::tr,
    state::Preferences,
    theme,
};
use gtk::prelude::*;
use std::{cell::RefCell, rc::Rc};

type State = Rc<RefCell<Preferences>>;

pub fn build(
    state: State,
    provider: gtk::CssProvider,
    system: Option<SystemAppearance>,
) -> gtk::ScrolledWindow {
    let (scroll, body) = page("Personalização", "Aparência do sistema e da interface.");
    let preview = gtk::Box::new(gtk::Orientation::Horizontal, 16);
    preview.add_css_class("preview");
    let side = gtk::Box::new(gtk::Orientation::Vertical, 12);
    side.add_css_class("preview-sidebar");
    side.append(&icon("personalization", 30));
    let content = gtk::Box::new(gtk::Orientation::Vertical, 12);
    content.add_css_class("preview-content");
    content.set_hexpand(true);
    content.append(&label("Aparência sincronizada", "hero-title"));
    content.append(&label(
        "A interface acompanha automaticamente as cores e o modo definidos no Mint.",
        "secondary",
    ));
    let accent = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    accent.add_css_class("preview-accent");
    content.append(&accent);
    preview.append(&side);
    preview.append(&content);
    body.append(&preview);

    let status = label(
        if system.is_some() {
            "Sincronizado com as configurações do sistema."
        } else {
            "O painel de temas do Cinnamon não está disponível nesta sessão."
        },
        "secondary",
    );
    body.append(&label("Tema do sistema", "section-title"));
    let themes = card();
    themes.append(&super::integrations::launch_card(
        "Cores, modo e temas",
        "Defina cor, modo misto, claro ou escuro no painel do Mint",
        "preferences-desktop-theme-symbolic",
        catalog::Target::Module("themes".into()),
        &status,
    ));
    body.append(&themes);

    let current = card();
    let mode = value_row("Modo atual", "personalization");
    let gtk_theme = value_row("Tema GTK", "personalization");
    let icons = value_row("Tema de ícones", "personalization");
    let cursor = value_row("Tema do ponteiro", "personalization");
    let font = value_row("Fonte da interface", "personalization");
    for (_, row) in [&mode, &gtk_theme, &icons, &cursor, &font] {
        current.append(row);
    }
    body.append(&label("Aparência atual", "section-title"));
    body.append(&current);
    body.append(&status);

    let sync = {
        let state = state.clone();
        let provider = provider.clone();
        let system = system.clone();
        let mode = mode.0.clone();
        let gtk_theme = gtk_theme.0.clone();
        let icons = icons.0.clone();
        let cursor = cursor.0.clone();
        let font = font.0.clone();
        Rc::new(move || {
            if let Some(system) = &system {
                system.sync_preferences(&mut state.borrow_mut());
                mode.set_text(&tr(match state.borrow().theme.as_str() {
                    "dark" => "Escuro",
                    "light" => "Claro",
                    _ => "Automático / misto",
                }));
                gtk_theme.set_text(&system.gtk_theme());
                icons.set_text(&system.icon_theme());
                cursor.set_text(&system.cursor_theme());
                font.set_text(&system.font());
            } else {
                mode.set_text(&tr("Automático"));
                gtk_theme.set_text(&tr("Conforme a sessão GTK"));
                icons.set_text(&state.borrow().icons);
                cursor.set_text(&tr("Conforme a sessão GTK"));
                font.set_text(&state.borrow().font);
            }
            theme::apply(&provider, &state.borrow());
            let _ = state.borrow().save();
        })
    };
    sync();
    if let Some(system) = &system {
        let sync = sync.clone();
        system.connect_changed(move || sync());
    }

    super::integrations::append(&body, "personalization");
    scroll
}

fn value_row(title: &str, image: &str) -> (gtk::Label, gtk::Box) {
    let row = row(title, "", image);
    let value = label("—", "secondary");
    value.set_halign(gtk::Align::End);
    value.set_selectable(true);
    row.append(&value);
    (value, row)
}
