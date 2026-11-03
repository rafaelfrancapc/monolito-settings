use super::{base_system_info, desktop_settings, SettingsBackend, SystemInfo};
use gtk::prelude::*;

pub struct CinnamonBackend;

const INTERFACE_SCHEMA: &str = "org.cinnamon.desktop.interface";

impl SettingsBackend for CinnamonBackend {
    fn read_system(&self) -> SystemInfo {
        let mut info = base_system_info();
        info.integration_title = "Integração com Cinnamon".into();

        for (key, title) in [
            ("gtk-theme", "Tema GTK"),
            ("icon-theme", "Tema de ícones"),
            ("font-name", "Fonte da interface"),
        ] {
            if let Some(settings) = desktop_settings::setting(INTERFACE_SCHEMA, key, "s") {
                info.settings
                    .insert(title.into(), settings.string(key).into());
            }
        }
        info.integration = if info.settings.is_empty() {
            "Esquemas do Cinnamon não disponíveis nesta sessão."
        } else {
            "Cinnamon / GSettings"
        }
        .into();
        info
    }
}
