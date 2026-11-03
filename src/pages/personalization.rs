use crate::{
    backends::appearance::SystemAppearance, components::*, i18n::tr, state::Preferences, theme,
};
use gtk::prelude::*;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

type State = Rc<RefCell<Preferences>>;

fn persist(state: &State, provider: &gtk::CssProvider) -> Result<(), String> {
    theme::apply(provider, &state.borrow());
    state.borrow().save()
}

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
    content.append(&label("Prévia da interface", "hero-title"));
    content.append(&label(
        "As alterações acompanham as preferências da sessão.",
        "secondary",
    ));
    let accent_preview = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    accent_preview.add_css_class("preview-accent");
    content.append(&accent_preview);
    preview.append(&side);
    preview.append(&content);
    body.append(&preview);
    body.append(&label("Aparência", "section-title"));

    let status = label(
        if system.is_some() {
            "Sincronizado com as configurações do sistema."
        } else {
            "Preferências aplicadas somente a este aplicativo."
        },
        "secondary",
    );
    let appearance_card = card();
    let mode_row = row(
        "Modo de cor",
        "Claro, escuro ou conforme a preferência do sistema",
        "personalization",
    );
    let mode_group = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let mut mode_first: Option<gtk::ToggleButton> = None;
    let mut mode_buttons = Vec::new();
    for (id, text) in [
        ("light", "Claro"),
        ("dark", "Escuro"),
        ("auto", "Automático"),
    ] {
        let button = gtk::ToggleButton::with_label(&tr(text));
        button.add_css_class("choice");
        if let Some(first) = &mode_first {
            button.set_group(Some(first));
        } else {
            mode_first = Some(button.clone());
        }
        button.set_active(state.borrow().theme == id);
        mode_group.append(&button);
        mode_buttons.push((id, button));
    }
    let mode_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
    mode_box.append(&mode_row);
    mode_group.set_margin_start(20);
    mode_group.set_margin_bottom(16);
    mode_box.append(&mode_group);
    appearance_card.append(&mode_box);

    let accent_row = row(
        "Cor de destaque",
        "Cor coordenada com a família de temas do Cinnamon",
        "personalization",
    );
    let colors = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let mut accent_first: Option<gtk::ToggleButton> = None;
    let mut accent_buttons = Vec::new();
    for (id, text) in [
        ("blue", "Azul"),
        ("violet", "Violeta"),
        ("green", "Verde"),
        ("terracotta", "Terracota"),
    ] {
        let button = gtk::ToggleButton::new();
        button.add_css_class("swatch");
        button.add_css_class(id);
        accessible(&button, text);
        button.set_tooltip_text(Some(&tr(text)));
        if let Some(first) = &accent_first {
            button.set_group(Some(first));
        } else {
            accent_first = Some(button.clone());
        }
        button.set_active(state.borrow().accent == id);
        if let Some(system) = &system {
            button.set_sensitive(system.accent_available());
        }
        colors.append(&button);
        accent_buttons.push((id, button));
    }
    accent_row.append(&colors);
    appearance_card.append(&accent_row);
    body.append(&appearance_card);

    let details = card();
    let font_row = row(
        "Fonte da interface",
        "Fonte utilizada pelo desktop e pelos aplicativos GTK",
        "personalization",
    );
    let font = gtk::FontButton::with_font(&state.borrow().font);
    font.set_use_font(false);
    font.set_valign(gtk::Align::Center);
    font_row.append(&font);
    details.append(&font_row);

    let mut system_entries = Vec::new();
    if let Some(system) = &system {
        for (key, title, description, value) in [
            (
                "gtk",
                "Tema dos aplicativos",
                "Tema GTK usado pela sessão",
                system.gtk_theme(),
            ),
            (
                "icons",
                "Tema de ícones",
                "Ícones usados pelo desktop e pelos aplicativos",
                system.icon_theme(),
            ),
            (
                "cursor",
                "Tema do ponteiro",
                "Cursor usado pela sessão",
                system.cursor_theme(),
            ),
        ] {
            let setting_row = row(title, description, "personalization");
            let entry = gtk::Entry::builder()
                .text(value)
                .width_chars(10)
                .max_width_chars(14)
                .valign(gtk::Align::Center)
                .build();
            entry.set_widget_name(&format!("system-theme-{key}"));
            accessible(&entry, title);
            let apply = button("Aplicar");
            setting_row.append(&entry);
            setting_row.append(&apply);
            details.append(&setting_row);
            system_entries.push((key, entry, apply));
        }
    }
    body.append(&details);
    body.append(&status);

    let refreshing = Rc::new(Cell::new(false));
    let sync_ui: Rc<dyn Fn()> = {
        let state = state.clone();
        let provider = provider.clone();
        let system = system.clone();
        let mode_buttons = mode_buttons.clone();
        let accent_buttons = accent_buttons.clone();
        let font = font.clone();
        let entries: Vec<_> = system_entries
            .iter()
            .map(|(key, entry, _)| (*key, entry.clone()))
            .collect();
        let refreshing = refreshing.clone();
        Rc::new(move || {
            if refreshing.replace(true) {
                return;
            }
            if let Some(system) = &system {
                system.sync_preferences(&mut state.borrow_mut());
                font.set_font(&system.font());
                for (key, entry) in &entries {
                    entry.set_text(&match *key {
                        "gtk" => system.gtk_theme(),
                        "icons" => system.icon_theme(),
                        _ => system.cursor_theme(),
                    });
                }
            }
            for (id, button) in &mode_buttons {
                button.set_active(state.borrow().theme == *id);
            }
            for (id, button) in &accent_buttons {
                button.set_active(state.borrow().accent == *id);
            }
            let _ = persist(&state, &provider);
            refreshing.set(false);
        })
    };

    for (id, button) in &mode_buttons {
        let id = *id;
        let state = state.clone();
        let provider = provider.clone();
        let status = status.clone();
        let system = system.clone();
        let sync_ui = sync_ui.clone();
        let refreshing = refreshing.clone();
        button.connect_toggled(move |button| {
            if !button.is_active() || refreshing.get() {
                return;
            }
            state.borrow_mut().theme = id.into();
            let result = system
                .as_ref()
                .map_or(Ok(()), |settings| settings.set_mode(id));
            match result {
                Ok(()) => {
                    let _ = persist(&state, &provider);
                    sync_ui();
                    status.set_text(&tr("Modo de cor atualizado."));
                }
                Err(error) => {
                    let _ = persist(&state, &provider);
                    status.set_text(&tr(&error));
                }
            }
        });
    }
    for (id, button) in &accent_buttons {
        let id = *id;
        let state = state.clone();
        let provider = provider.clone();
        let status = status.clone();
        let system = system.clone();
        let sync_ui = sync_ui.clone();
        let refreshing = refreshing.clone();
        button.connect_toggled(move |button| {
            if !button.is_active() || refreshing.get() {
                return;
            }
            state.borrow_mut().accent = id.into();
            let result = system
                .as_ref()
                .map_or(Ok(()), |settings| settings.set_accent(id));
            match result {
                Ok(()) => {
                    let _ = persist(&state, &provider);
                    sync_ui();
                    status.set_text(&tr("Cor de destaque atualizada."));
                }
                Err(error) => status.set_text(&tr(&error)),
            }
        });
    }

    {
        let state = state.clone();
        let provider = provider.clone();
        let status = status.clone();
        let system = system.clone();
        let sync_ui = sync_ui.clone();
        let refreshing = refreshing.clone();
        font.connect_font_set(move |button| {
            if refreshing.get() {
                return;
            }
            let value = button.font().unwrap_or_default().to_string();
            let result = system
                .as_ref()
                .map_or(Ok(()), |settings| settings.set_font(&value));
            if let Err(error) = result {
                status.set_text(&tr(&error));
                return;
            }
            state.borrow_mut().font = value;
            let _ = persist(&state, &provider);
            sync_ui();
            status.set_text(&tr("Fonte atualizada."));
        });
    }

    for (key, entry, apply) in &system_entries {
        let key = *key;
        let entry = entry.clone();
        let status = status.clone();
        let system = system.clone().expect("entries require system appearance");
        let sync_ui = sync_ui.clone();
        apply.connect_clicked(move |_| {
            let value = entry.text();
            let result = match key {
                "gtk" => system.set_gtk_theme(&value),
                "icons" => system.set_icon_theme(&value),
                _ => system.set_cursor_theme(&value),
            };
            match result {
                Ok(()) => {
                    sync_ui();
                    status.set_text(&tr("Tema atualizado."));
                }
                Err(error) => status.set_text(&tr(&error)),
            }
        });
    }

    if let Some(system) = &system {
        let sync_ui = sync_ui.clone();
        system.connect_changed(move || sync_ui());
    }
    super::integrations::append(&body, "personalization");
    scroll
}
