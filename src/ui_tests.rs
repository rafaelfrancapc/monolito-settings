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
    search.set_text("");
    settle();
    assert!((0..11).all(|i| list.row_at_index(i).unwrap().is_visible()));
    stack.set_visible_child_name("personalization");
    settle();
    for text in ["Escuro", "Claro", "Automático"] {
        let button = widgets
            .iter()
            .filter_map(|x| x.clone().downcast::<gtk::ToggleButton>().ok())
            .find(|b| b.label().as_deref() == Some(text))
            .unwrap();
        button.set_active(true);
        settle();
        assert!(button.is_active());
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
