use gtk::prelude::*;
pub fn run() -> gtk::glib::ExitCode {
    gtk::gio::resources_register_include!("monolito.gresource").expect("resources");
    let app = gtk::Application::builder()
        .application_id("org.monolito.Settings")
        .build();
    app.connect_activate(|app| {
        if let Some(window) = app.active_window() {
            window.present();
            return;
        }
        super::window::build(app).present();
    });
    app.run()
}
