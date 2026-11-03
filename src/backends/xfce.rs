use super::{base_system_info, command, SettingsBackend, SystemInfo};
pub struct XfceBackend;
impl SettingsBackend for XfceBackend {
    fn read_system(&self) -> SystemInfo {
        let mut info = base_system_info();
        info.integration_title = "Integração com XFCE".into();
        match command("xfconf-query", &["-c", "xsettings", "-l"]) {
            Ok(_) => {
                for property in ["/Net/ThemeName", "/Net/IconThemeName", "/Gtk/FontName"] {
                    if let Ok(v) = command("xfconf-query", &["-c", "xsettings", "-p", property]) {
                        info.settings.insert(property.into(), v);
                    }
                }
                info.integration = "XFCE / xfconf (somente leitura nesta versão)".into();
            }
            Err(_) => info.integration = "Serviço xfconf indisponível nesta sessão".into(),
        }
        info
    }
}
