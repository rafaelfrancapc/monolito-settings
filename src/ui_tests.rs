//! Run with a real window manager: dbus-run-session -- xvfb-run cargo test ui_workflow -- --ignored
use gtk::prelude::*;
use std::{
    thread,
    time::{Duration, Instant},
};
fn settle() {
    let until = Instant::now() + Duration::from_millis(350);
    while Instant::now() < until {
        while gtk::glib::MainContext::default().iteration(false) {}
        thread::sleep(Duration::from_millis(10));
    }
}
fn descendants(w: &gtk::Widget) -> Vec<gtk::Widget> {
    let mut result = vec![w.clone()];
    let mut child = w.first_child();
    while let Some(c) = child {
        result.extend(descendants(&c));
        child = c.next_sibling();
    }
    result
}
#[test]
#[ignore = "requires display, session bus and window manager"]
fn ui_workflow() {
    gtk::init().unwrap();
    gtk::gio::resources_register_include!("monolito.gresource").unwrap();
    assert!(
        gtk::gdk_pixbuf::Pixbuf::from_resource("/org/monolito/Settings/icons/home-symbolic.svg")
            .is_ok(),
        "SVG loader required"
    );
    let app = gtk::Application::builder()
        .application_id("org.monolito.Settings.Test")
        .build();
    app.register(None::<&gtk::gio::Cancellable>).unwrap();
    let w = crate::window::build(&app);
    w.present();
    settle();
    let widgets = descendants(w.upcast_ref());
    assert_eq!(w.title().as_deref(), Some("Configurações"));
    for text in widgets
        .iter()
        .filter_map(|x| x.clone().downcast::<gtk::Label>().ok())
    {
        assert!(
            !text.text().to_lowercase().contains("monolito settings"),
            "Visible brand: {}",
            text.text()
        );
        assert!(!text.text().contains("Em desenvolvimento"));
    }
    let stack = widgets
        .iter()
        .find_map(|x| x.clone().downcast::<gtk::Stack>().ok())
        .unwrap();
    let list = widgets
        .iter()
        .find_map(|x| x.clone().downcast::<gtk::ListBox>().ok())
        .unwrap();
    for (i, c) in crate::navigation::CATEGORIES.iter().enumerate() {
        list.select_row(list.row_at_index(i as i32).as_ref());
        settle();
        assert_eq!(stack.visible_child_name().as_deref(), Some(c.id));
    }
    let search = widgets
        .iter()
        .find_map(|x| x.clone().downcast::<gtk::SearchEntry>().ok())
        .unwrap();
    search.set_text("personalizacao");
    settle();
    assert!(list.row_at_index(4).unwrap().is_visible());
    assert!(!list.row_at_index(0).unwrap().is_visible());
    search.set_text("sem-resultados");
    settle();
    assert!((0..11).all(|i| !list.row_at_index(i).unwrap().is_visible()));
    search.set_text("Bluetooth");
    settle();
    assert!(list.row_at_index(2).unwrap().is_visible());
    search.set_text("");
    settle();
    assert!((0..11).all(|i| list.row_at_index(i).unwrap().is_visible()));
    stack.set_visible_child_name("personalization");
    settle();
    if crate::backends::DesktopBackend::detect() == crate::backends::DesktopBackend::Cinnamon {
        let mouse = crate::backends::desktop_settings::setting(
            "org.cinnamon.desktop.peripherals.mouse",
            "left-handed",
            "b",
        )
        .unwrap();
        let switch = widgets
            .iter()
            .filter_map(|w| w.clone().downcast::<gtk::Switch>().ok())
            .find(|w| w.widget_name() == "org.cinnamon.desktop.peripherals.mouse/left-handed")
            .unwrap();
        mouse.set_boolean("left-handed", true).unwrap();
        settle();
        assert!(switch.is_active());
        switch.set_active(false);
        settle();
        assert!(!mouse.boolean("left-handed"));
        let settings = crate::backends::desktop_settings::setting(
            "org.cinnamon.desktop.interface",
            "font-name",
            "s",
        )
        .expect("Cinnamon test schemas");
        settings.set_string("font-name", "Sans 11").unwrap();
        let violet = widgets
            .iter()
            .filter_map(|w| w.clone().downcast::<gtk::ToggleButton>().ok())
            .find(|button| button.has_css_class("violet"))
            .unwrap();
        violet.set_active(true);
        settle();
        let accent =
            crate::backends::desktop_settings::setting("org.x.apps.portal", "accent-rgb", "s")
                .unwrap();
        assert_eq!(accent.string("accent-rgb"), "#7040B0");
        assert_eq!(
            crate::backends::desktop_settings::setting(
                "org.cinnamon.desktop.interface",
                "gtk-theme",
                "s"
            )
            .unwrap()
            .string("gtk-theme"),
            "Adwaita-Violet"
        );
        let icons = crate::backends::desktop_settings::setting(
            "org.cinnamon.desktop.interface",
            "icon-theme",
            "s",
        )
        .unwrap();
        icons.set_string("icon-theme", "External-Icons").unwrap();
        let portal =
            crate::backends::desktop_settings::setting("org.x.apps.portal", "accent-rgb", "s")
                .unwrap();
        portal.set_string("accent-rgb", "#217346").unwrap();
        settle();
        assert!(
            widgets
                .iter()
                .filter_map(|w| w.clone().downcast::<gtk::FontButton>().ok())
                .any(|f| f.font().as_deref() == Some("Sans 11")),
            "External changes must update the native control"
        );
        assert!(widgets
            .iter()
            .filter_map(|w| w.clone().downcast::<gtk::Entry>().ok())
            .any(|entry| entry.widget_name() == "system-theme-icons"
                && entry.text() == "External-Icons"));
        assert!(widgets
            .iter()
            .filter_map(|w| w.clone().downcast::<gtk::ToggleButton>().ok())
            .any(|button| button.has_css_class("green") && button.is_active()));
        assert!(crate::backends::desktop_settings::setting(
            "org.cinnamon.desktop.interface",
            "missing-key",
            "b"
        )
        .is_none());
        assert!(crate::backends::desktop_settings::setting(
            "org.cinnamon.desktop.interface",
            "font-name",
            "b"
        )
        .is_none());
    }
    for text in ["Escuro", "Claro", "Automático"] {
        let button = widgets
            .iter()
            .filter_map(|x| x.clone().downcast::<gtk::ToggleButton>().ok())
            .find(|b| b.label().as_deref() == Some(text))
            .unwrap();
        button.set_active(true);
        settle();
        assert!(button.is_active());
        if crate::backends::DesktopBackend::detect() == crate::backends::DesktopBackend::Cinnamon {
            let portal = crate::backends::desktop_settings::setting(
                "org.x.apps.portal",
                "color-scheme",
                "s",
            )
            .unwrap();
            let expected = match text {
                "Escuro" => "prefer-dark",
                "Claro" => "prefer-light",
                _ => "default",
            };
            assert_eq!(portal.string("color-scheme"), expected);
        }
    }
    if let Ok(directory) = std::env::var("MONOLITO_SCREENSHOT_DIR") {
        std::fs::create_dir_all(&directory).unwrap();
        for page in ["personalization", "accounts", "devices"] {
            stack.set_visible_child_name(page);
            settle();
            let paintable = gtk::WidgetPaintable::new(Some(&w));
            let snapshot = gtk::Snapshot::new();
            paintable.snapshot(&snapshot, w.width() as f64, w.height() as f64);
            let node = snapshot.to_node().unwrap();
            let renderer = gtk::gsk::Renderer::for_surface(&w.surface().unwrap()).unwrap();
            renderer
                .render_texture(&node, None)
                .save_to_png(std::path::Path::new(&directory).join(format!("{page}.png")))
                .unwrap();
            renderer.unrealize();
        }
    }
    let control = |class: &str| {
        widgets
            .iter()
            .filter_map(|x| x.clone().downcast::<gtk::Button>().ok())
            .find(|b| b.has_css_class(class))
            .unwrap()
    };
    control("maximize").emit_clicked();
    settle();
    assert!(w.is_maximized());
    control("maximize").emit_clicked();
    settle();
    assert!(!w.is_maximized());
    control("minimize").emit_clicked();
    settle();
    let top = w
        .surface()
        .unwrap()
        .downcast::<gtk::gdk::Toplevel>()
        .unwrap();
    assert!(top.state().contains(gtk::gdk::ToplevelState::MINIMIZED));
    w.present();
    settle();
    assert!(!top.state().contains(gtk::gdk::ToplevelState::MINIMIZED));
    w.set_default_size(800, 600);
    settle();
    assert!(
        w.width() <= 800,
        "minimum width unexpectedly grew: {}",
        w.width()
    );
    control("close").emit_clicked();
    settle();
    assert!(!w.is_visible());
}
