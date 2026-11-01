pub fn tr(text: &str) -> String {
    gtk::glib::dgettext(Some("monolito-settings"), text).to_string()
}
