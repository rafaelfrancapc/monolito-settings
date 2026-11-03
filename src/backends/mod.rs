pub mod appearance;
pub mod catalog;
pub mod cinnamon;
pub mod desktop_settings;
pub mod directory;
pub mod local_accounts;
pub mod xfce;
use std::collections::BTreeMap;
#[derive(Default, Debug)]
pub struct SystemInfo {
    pub hostname: String,
    pub distribution: String,
    pub kernel: String,
    pub cpu: String,
    pub memory: String,
    pub session: String,
    pub desktop: String,
    pub settings: BTreeMap<String, String>,
    pub integration_title: String,
    pub integration: String,
}
pub trait SettingsBackend: Send + Sync {
    fn read_system(&self) -> SystemInfo;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DesktopBackend {
    Cinnamon,
    Xfce,
    Generic,
}

impl DesktopBackend {
    pub fn detect() -> Self {
        let desktop = [
            "XDG_CURRENT_DESKTOP",
            "XDG_SESSION_DESKTOP",
            "DESKTOP_SESSION",
        ]
        .into_iter()
        .find_map(|name| std::env::var(name).ok().filter(|value| !value.is_empty()))
        .unwrap_or_default();
        Self::detect_from(&desktop)
    }

    fn detect_from(desktop: &str) -> Self {
        let names = desktop
            .split([':', ';'])
            .map(str::trim)
            .map(str::to_ascii_lowercase)
            .collect::<Vec<_>>();
        if names
            .iter()
            .any(|name| name == "cinnamon" || name == "x-cinnamon")
        {
            Self::Cinnamon
        } else if names.iter().any(|name| name == "xfce" || name == "xfce4") {
            Self::Xfce
        } else {
            Self::Generic
        }
    }

    pub fn read_system(self) -> SystemInfo {
        match self {
            Self::Cinnamon => cinnamon::CinnamonBackend.read_system(),
            Self::Xfce => xfce::XfceBackend.read_system(),
            Self::Generic => {
                let mut info = base_system_info();
                info.integration_title = "Integração do desktop".into();
                info.integration =
                    "Desktop não reconhecido; nenhuma configuração foi alterada.".into();
                info
            }
        }
    }
}

pub(crate) fn base_system_info() -> SystemInfo {
    fn read(path: &str) -> String {
        std::fs::read_to_string(path).unwrap_or_default()
    }
    fn value(s: &str, key: &str, separator: char) -> Option<String> {
        s.lines().find_map(|line| {
            let (k, v) = line.split_once(separator)?;
            (k.trim() == key).then(|| v.trim().trim_matches('"').to_string())
        })
    }
    SystemInfo {
        hostname: read("/etc/hostname").trim().into(),
        distribution: value(&read("/etc/os-release"), "PRETTY_NAME", '=').unwrap_or_default(),
        kernel: read("/proc/sys/kernel/osrelease").trim().into(),
        cpu: value(&read("/proc/cpuinfo"), "model name", ':').unwrap_or_default(),
        memory: value(&read("/proc/meminfo"), "MemTotal", ':').unwrap_or_default(),
        session: std::env::var("XDG_SESSION_TYPE").unwrap_or_default(),
        desktop: std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default(),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::DesktopBackend;

    #[test]
    fn selects_desktop_backend_from_composite_names() {
        assert_eq!(
            DesktopBackend::detect_from("X-Cinnamon"),
            DesktopBackend::Cinnamon
        );
        assert_eq!(
            DesktopBackend::detect_from("Cinnamon:GNOME"),
            DesktopBackend::Cinnamon
        );
        assert_eq!(DesktopBackend::detect_from("XFCE"), DesktopBackend::Xfce);
        assert_eq!(
            DesktopBackend::detect_from("GNOME"),
            DesktopBackend::Generic
        );
    }
}
// All subprocess operations are bounded, avoid shells, and belong to backends only.
pub fn command(program: &str, args: &[&str]) -> Result<String, String> {
    let out = std::process::Command::new("timeout")
        .args(["8s", program])
        .args(args)
        .env("LC_ALL", "C")
        .output()
        .map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        Err(format!("{program}: {}", out.status))
    }
}
