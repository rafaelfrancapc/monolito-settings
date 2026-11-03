use super::desktop_settings;
use crate::state::Preferences;
use gtk::{gio, prelude::*};
use serde::Deserialize;
use std::{fs, path::PathBuf};

const INTERFACE: &str = "org.cinnamon.desktop.interface";
const CINNAMON: &str = "org.cinnamon.theme";
const PORTAL: &str = "org.x.apps.portal";

#[derive(Clone, Debug, Deserialize)]
struct StyleFile {
    styles: Vec<Style>,
}

#[derive(Clone, Debug, Deserialize)]
struct Style {
    #[allow(dead_code)]
    #[serde(default)]
    mixed: Vec<Variant>,
    #[serde(default)]
    dark: Vec<Variant>,
    #[serde(default)]
    light: Vec<Variant>,
}

#[derive(Clone, Debug, Deserialize)]
struct Variant {
    #[serde(default)]
    themes: Option<String>,
    #[serde(default)]
    gtk: Option<String>,
    #[serde(default)]
    icons: Option<String>,
    #[serde(default)]
    cinnamon: Option<String>,
    #[serde(default)]
    cursor: Option<String>,
    color: String,
}

impl Variant {
    fn gtk(&self) -> Option<&str> {
        self.gtk.as_deref().or(self.themes.as_deref())
    }
    fn icons(&self) -> Option<&str> {
        self.icons.as_deref().or(self.themes.as_deref())
    }
    fn cinnamon(&self) -> Option<&str> {
        self.cinnamon.as_deref().or(self.themes.as_deref())
    }
    fn cursor(&self) -> Option<&str> {
        self.cursor.as_deref().or(self.themes.as_deref())
    }
}

impl Style {
    fn variants(&self, mode: &str) -> &[Variant] {
        match mode {
            "dark" => &self.dark,
            "light" => &self.light,
            _ => &self.mixed,
        }
    }
}

#[derive(Clone)]
pub struct SystemAppearance {
    interface: gio::Settings,
    cinnamon: Option<gio::Settings>,
    portal: Option<gio::Settings>,
    accent_portal: Option<gio::Settings>,
    styles: Vec<Style>,
}

impl SystemAppearance {
    pub fn detect() -> Option<Self> {
        let interface = desktop_settings::setting(INTERFACE, "gtk-theme", "s")?;
        for (key, kind) in [
            ("icon-theme", "s"),
            ("cursor-theme", "s"),
            ("font-name", "s"),
        ] {
            desktop_settings::setting(INTERFACE, key, kind)?;
        }
        Some(Self {
            interface,
            cinnamon: desktop_settings::setting(CINNAMON, "name", "s"),
            portal: desktop_settings::setting(PORTAL, "color-scheme", "s"),
            accent_portal: desktop_settings::setting(PORTAL, "accent-rgb", "s"),
            styles: load_styles(),
        })
    }

    pub fn sync_preferences(&self, preferences: &mut Preferences) {
        preferences.font = self.interface.string("font-name").into();
        preferences.icons = self.interface.string("icon-theme").into();
        if let Some(accent) = self
            .accent_portal
            .as_ref()
            .map(|settings| settings.string("accent-rgb"))
            .filter(|accent| !accent.is_empty())
        {
            preferences.accent = accent_for_color(&accent).into();
        } else if let Some((_, _, variant)) = self.active_variant() {
            preferences.accent = accent_for_color(&variant.color).into();
        }
        if let Some(portal) = &self.portal {
            preferences.theme = match portal.string("color-scheme").as_str() {
                "prefer-dark" => "dark",
                "prefer-light" => "light",
                _ => "auto",
            }
            .into();
        } else if let Some((_, mode, _)) = self.active_variant() {
            preferences.theme = mode.into();
        } else {
            preferences.theme = if self
                .interface
                .string("gtk-theme")
                .to_ascii_lowercase()
                .contains("dark")
            {
                "dark"
            } else {
                "light"
            }
            .into();
        }
    }

    pub fn gtk_theme(&self) -> String {
        self.interface.string("gtk-theme").into()
    }
    pub fn icon_theme(&self) -> String {
        self.interface.string("icon-theme").into()
    }
    pub fn cursor_theme(&self) -> String {
        self.interface.string("cursor-theme").into()
    }
    pub fn font(&self) -> String {
        self.interface.string("font-name").into()
    }
    pub fn connect_changed<F: Fn() + Clone + 'static>(&self, callback: F) {
        for key in ["gtk-theme", "icon-theme", "cursor-theme", "font-name"] {
            self.interface.connect_changed(Some(key), {
                let callback = callback.clone();
                move |_, _| callback()
            });
        }
        if let Some(settings) = &self.portal {
            settings.connect_changed(Some("color-scheme"), {
                let callback = callback.clone();
                move |_, _| callback()
            });
        }
        if let Some(settings) = &self.accent_portal {
            settings.connect_changed(Some("accent-rgb"), {
                let callback = callback.clone();
                move |_, _| callback()
            });
        }
        if let Some(settings) = &self.cinnamon {
            settings.connect_changed(Some("name"), move |_, _| callback());
        }
    }

    fn active_variant(&self) -> Option<(usize, &'static str, &Variant)> {
        let gtk = self.interface.string("gtk-theme");
        let icons = self.interface.string("icon-theme");
        let cursor = self.interface.string("cursor-theme");
        let cinnamon = self.cinnamon.as_ref().map(|s| s.string("name"));
        for (style_index, style) in self.styles.iter().enumerate() {
            for mode in ["mixed", "dark", "light"] {
                if let Some(variant) = style.variants(mode).iter().find(|variant| {
                    variant.gtk().is_none_or(|value| value == gtk)
                        && variant.icons().is_none_or(|value| value == icons)
                        && variant.cursor().is_none_or(|value| value == cursor)
                        && cinnamon.as_ref().is_none_or(|active| {
                            variant.cinnamon().is_none_or(|value| value == active)
                        })
                }) {
                    return Some((style_index, mode, variant));
                }
            }
        }
        None
    }
}

fn load_styles() -> Vec<Style> {
    let mut directories = std::env::var_os("MONOLITO_STYLES_DIR")
        .map(PathBuf::from)
        .into_iter()
        .collect::<Vec<_>>();
    directories.push(PathBuf::from("/usr/share/cinnamon/styles.d"));
    directories.push(gtk::glib::user_data_dir().join("cinnamon/styles.d"));
    let mut styles = Vec::new();
    for directory in directories {
        let Ok(entries) = fs::read_dir(directory) else {
            continue;
        };
        let mut paths: Vec<_> = entries.flatten().map(|entry| entry.path()).collect();
        paths.sort();
        for path in paths
            .into_iter()
            .filter(|path| path.extension().is_some_and(|e| e == "styles"))
        {
            if let Ok(file) = fs::read(&path)
                .ok()
                .and_then(|data| serde_json::from_slice::<StyleFile>(&data).ok())
                .ok_or(())
            {
                styles.extend(file.styles);
            }
        }
    }
    styles
}

fn parse_color(value: &str) -> (i32, i32, i32) {
    let value = value.trim().trim_start_matches('#');
    if value.len() != 6 {
        return (40, 100, 189);
    }
    (
        i32::from_str_radix(&value[0..2], 16).unwrap_or(40),
        i32::from_str_radix(&value[2..4], 16).unwrap_or(100),
        i32::from_str_radix(&value[4..6], 16).unwrap_or(189),
    )
}

fn accent_rgb(value: &str) -> (i32, i32, i32) {
    parse_color(match value {
        "violet" => "#7040B0",
        "green" => "#217346",
        "terracotta" => "#A3482C",
        _ => "#2864BD",
    })
}

fn color_distance(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

fn accent_for_color(value: &str) -> &'static str {
    let color = parse_color(value);
    ["blue", "violet", "green", "terracotta"]
        .into_iter()
        .min_by_key(|accent| color_distance(color, accent_rgb(accent)))
        .unwrap_or("blue")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_cinnamon_style_variants_and_maps_accents() {
        let file: StyleFile = serde_json::from_str(
            r##"{"styles":[{"name":"Corporate","default":"light","light":[
              {"name":"blue","themes":"Mint-Y","color":"#2864BD","default":"true"},
              {"name":"green","gtk":"Mint-Y-Green","icons":"Mint-Y-Green","cinnamon":"Mint-Y-Green","cursor":"Bibata","color":"#217346"}],
              "dark":[{"name":"blue","themes":"Mint-Y-Dark","color":"#2864BD"}]}]}"##,
        )
        .unwrap();
        assert_eq!(file.styles[0].light[1].cursor(), Some("Bibata"));
        assert_eq!(accent_for_color("#1f7548"), "green");
    }
}
