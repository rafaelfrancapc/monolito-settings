use super::{command, SettingsBackend, SystemInfo};
use std::{env, fs};
pub struct XfceBackend;
fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default()
}
fn value(s: &str, key: &str, separator: char) -> Option<String> {
    s.lines().find_map(|line| {
        let (k, v) = line.split_once(separator)?;
        (k.trim() == key).then(|| v.trim().trim_matches('"').to_string())
    })
}
impl SettingsBackend for XfceBackend {
    fn read_system(&self) -> SystemInfo {
        let mut info = SystemInfo {
            hostname: read("/etc/hostname").trim().into(),
            distribution: value(&read("/etc/os-release"), "PRETTY_NAME", '=').unwrap_or_default(),
            kernel: read("/proc/sys/kernel/osrelease").trim().into(),
            cpu: value(&read("/proc/cpuinfo"), "model name", ':').unwrap_or_default(),
            memory: value(&read("/proc/meminfo"), "MemTotal", ':').unwrap_or_default(),
            session: env::var("XDG_SESSION_TYPE").unwrap_or_default(),
            desktop: env::var("XDG_CURRENT_DESKTOP").unwrap_or_default(),
            ..Default::default()
        };
        match command("xfconf-query", &["-c", "xsettings", "-l"]) {
            Ok(_) => {
                for property in ["/Net/ThemeName", "/Net/IconThemeName", "/Gtk/FontName"] {
                    if let Ok(v) = command("xfconf-query", &["-c", "xsettings", "-p", property]) {
                        info.xfce.insert(property.into(), v);
                    }
                }
                info.integration = "XFCE / xfconf".into();
            }
            Err(_) => info.integration = "Serviço xfconf indisponível nesta sessão".into(),
        }
        info
    }
}
