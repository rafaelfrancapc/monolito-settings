use crate::state::Preferences;
use gtk::gdk;
pub fn apply(provider: &gtk::CssProvider, prefs: &Preferences) {
    let settings = gtk::Settings::default().expect("GTK settings");
    // Do not replace the system theme: application-priority, scoped CSS owns our surfaces.
    let dark = match prefs.theme.as_str() {
        "dark" => true,
        "light" => false,
        _ => {
            settings.is_gtk_application_prefer_dark_theme()
                || settings
                    .gtk_theme_name()
                    .is_some_and(|s| s.to_lowercase().contains("dark"))
        }
    };
    let colors = if dark {
        [
            "#17191D", "#25282E", "#30343B", "#F2F3F5", "#B7BDC7", "#414650", "#273A55", "#17191D",
        ]
    } else {
        [
            "#F6F7F9", "#FFFFFF", "#EBEDF1", "#1B1D22", "#606774", "#D9DDE4", "#E2ECFA", "#FFFFFF",
        ]
    };
    let accent = match (prefs.accent.as_str(), dark) {
        ("violet", false) => "#7040B0",
        ("violet", true) => "#C6A0F6",
        ("green", false) => "#217346",
        ("green", true) => "#8BD5A0",
        ("terracotta", false) => "#A3482C",
        ("terracotta", true) => "#F5A58B",
        (_, true) => "#89B4FA",
        _ => "#2864BD",
    };
    let mut css = String::new();
    for (name, value) in [
        "canvas",
        "primary",
        "secondary",
        "text",
        "muted",
        "border",
        "subtle",
        "on_accent",
    ]
    .iter()
    .zip(colors)
    {
        css += &format!("@define-color {name} {value};\n");
    }
    css += &format!("@define-color accent {accent};\n");
    css += include_str!("../resources/styles/app.css");
    provider.load_from_data(&css);
    settings.set_gtk_font_name(Some(&prefs.font));
    settings.set_gtk_icon_theme_name(Some(&prefs.icons));
}
pub fn install(prefs: &Preferences) -> gtk::CssProvider {
    let p = gtk::CssProvider::new();
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("display"),
        &p,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    apply(&p, prefs);
    p
}
