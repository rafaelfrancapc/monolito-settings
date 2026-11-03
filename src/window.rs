use crate::{
    backends::{appearance::SystemAppearance, DesktopBackend},
    components::*,
    navigation::CATEGORIES,
    pages,
    state::Preferences,
    theme,
};
use gtk::prelude::*;
use std::{cell::RefCell, rc::Rc, sync::mpsc};
pub fn build(app: &gtk::Application) -> gtk::ApplicationWindow {
    let appearance = (DesktopBackend::detect() == DesktopBackend::Cinnamon)
        .then(SystemAppearance::detect)
        .flatten();
    let mut preferences = Preferences::load();
    preferences.theme = "auto".into();
    if let Some(system) = &appearance {
        system.sync_preferences(&mut preferences);
    }
    let state = Rc::new(RefCell::new(preferences));
    let provider = theme::install(&state.borrow());
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title(crate::i18n::tr("Configurações"))
        .default_width(1100)
        .default_height(720)
        .build();
    window.add_css_class("settings-app");
    window.set_size_request(760, 520);
    let titlebar = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    titlebar.add_css_class("titlebar");
    let handle = gtk::WindowHandle::new();
    handle.set_hexpand(true);
    handle.set_child(Some(&gtk::Box::new(gtk::Orientation::Horizontal, 0)));
    titlebar.append(&handle);
    titlebar.append(&crate::components::window_controls::build(&window));
    window.set_titlebar(Some(&titlebar));
    let shell = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let sidebar = gtk::Box::new(gtk::Orientation::Vertical, 0);
    sidebar.add_css_class("sidebar");
    sidebar.set_size_request(250, -1);
    sidebar.append(&label("Configurações", "sidebar-title"));
    let search = gtk::SearchEntry::new();
    search.set_placeholder_text(Some(&crate::i18n::tr("Pesquisar configurações")));
    search.add_css_class("search");
    accessible(&search, "Pesquisar configurações");
    sidebar.append(&search);
    let list = gtk::ListBox::new();
    list.add_css_class("nav-list");
    let stack = gtk::Stack::new();
    stack.set_hhomogeneous(false);
    stack.set_vhomogeneous(false);
    stack.set_hexpand(true);
    stack.set_vexpand(true);
    stack.set_transition_duration(200);
    let settings = gtk::Settings::default().unwrap();
    let set_motion = |s: &gtk::Settings, stack: &gtk::Stack| {
        stack.set_transition_type(if s.is_gtk_enable_animations() {
            gtk::StackTransitionType::Crossfade
        } else {
            gtk::StackTransitionType::None
        })
    };
    set_motion(&settings, &stack);
    let weak = stack.downgrade();
    settings.connect_gtk_enable_animations_notify(move |s| {
        if let Some(stack) = weak.upgrade() {
            set_motion(s, &stack)
        }
    });
    let mut rows = Vec::new();
    for c in CATEGORIES {
        let row = gtk::ListBoxRow::new();
        row.add_css_class("nav-item");
        let b = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        b.append(&icon(c.icon, 20));
        b.append(&label(c.title, "nav-label"));
        row.set_child(Some(&b));
        accessible(&row, c.title);
        list.append(&row);
        rows.push(row);
    }
    let navscroll = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .child(&list)
        .build();
    sidebar.append(&navscroll);
    let empty = label("Nenhuma categoria encontrada", "secondary");
    empty.set_visible(false);
    sidebar.append(&empty);
    let (home, host, summary) = pages::home::build(&stack);
    stack.add_named(&home, Some("home"));
    let (system, system_body) = pages::system::build();
    stack.add_named(&system, Some("system"));
    stack.add_named(
        &pages::personalization::build(state.clone(), provider.clone(), appearance),
        Some("personalization"),
    );
    stack.add_named(&pages::accounts::build(), Some("accounts"));
    for c in CATEGORIES {
        if !["home", "system", "personalization", "accounts"].contains(&c.id) {
            stack.add_named(&pages::settings_page(c), Some(c.id));
        }
    }
    let weak = stack.downgrade();
    list.connect_row_selected(move |_, row| {
        if let (Some(s), Some(row)) = (weak.upgrade(), row) {
            s.set_visible_child_name(CATEGORIES[row.index() as usize].id);
        }
    });
    let weak = list.downgrade();
    stack.connect_visible_child_name_notify(move |s| {
        if let (Some(list), Some(name)) = (weak.upgrade(), s.visible_child_name()) {
            if let Some(i) = CATEGORIES.iter().position(|c| c.id == name) {
                list.select_row(list.row_at_index(i as i32).as_ref());
            }
        }
    });
    let settings_catalog = crate::backends::catalog::catalog();
    search.connect_search_changed(move |s| {
        let mut count = 0;
        for (c, r) in CATEGORIES.iter().zip(&rows) {
            let found = crate::navigation::matches(c, &s.text())
                || settings_catalog.iter().any(|setting| {
                    setting.category == c.id
                        && crate::navigation::normalize(&crate::i18n::tr(&setting.title))
                            .contains(&crate::navigation::normalize(&s.text()))
                });
            r.set_visible(found);
            if found {
                count += 1;
            }
        }
        empty.set_visible(count == 0);
    });
    list.select_row(list.row_at_index(0).as_ref());
    shell.append(&sidebar);
    shell.append(&stack);
    window.set_child(Some(&shell));
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(DesktopBackend::detect().read_system());
    });
    gtk::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        match rx.try_recv() {
            Ok(info) => {
                host.set_text(&if info.hostname.is_empty() {
                    crate::i18n::tr("Este computador")
                } else {
                    info.hostname.clone()
                });
                summary.set_text(&format!(
                    "{}\n{} · {}",
                    info.distribution, info.desktop, info.session
                ));
                pages::system::populate(&system_body, &info);
                gtk::glib::ControlFlow::Break
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                summary.set_text(&crate::i18n::tr("Não foi possível consultar o sistema."));
                gtk::glib::ControlFlow::Break
            }
            _ => gtk::glib::ControlFlow::Continue,
        }
    });
    for property in ["gtk-theme-name", "gtk-application-prefer-dark-theme"] {
        let (s, p) = (state.clone(), provider.clone());
        settings.connect_notify_local(Some(property), move |_, _| {
            if s.borrow().theme == "auto" {
                theme::apply(&p, &s.borrow())
            }
        });
    }
    window
}
