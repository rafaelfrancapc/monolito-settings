mod app;
mod backends;
mod components;
mod i18n;
mod navigation;
mod pages;
mod state;
mod theme;
mod window;

fn main() -> gtk::glib::ExitCode {
    app::run()
}

#[cfg(test)]
mod ui_tests;
