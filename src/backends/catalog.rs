//! Cinnamon modules and Mint tools. Never interpolate user input into commands.
use gtk::{gio, glib, prelude::*};
use std::{collections::HashSet, path::Path};

pub const MODULE_DIR: &str = "/usr/share/cinnamon/cinnamon-settings/modules";
#[derive(Clone)]
pub enum Target {
    Module(String),
    Commands(&'static [&'static str]),
    Desktop(gio::DesktopAppInfo),
}
#[derive(Clone)]
pub struct Setting {
    pub id: String,
    pub category: &'static str,
    pub title: String,
    pub description: String,
    pub target: Target,
}

// Stable module names from cinnamon-settings; no GTK3 widget embedding is required.
pub const MODULES: &[(&str, &str, &str, &str)] = &[
    (
        "display",
        "system",
        "Monitores",
        "Resolução, escala e disposição das telas",
    ),
    (
        "nightlight",
        "system",
        "Luz noturna",
        "Temperatura de cor e programação",
    ),
    (
        "sound",
        "system",
        "Som",
        "Saída, entrada e volume dos aplicativos",
    ),
    ("power", "system", "Energia", "Suspensão, bateria e brilho"),
    (
        "notifications",
        "system",
        "Notificações",
        "Alertas e comportamento das notificações",
    ),
    (
        "general",
        "system",
        "Geral",
        "Preferências da sessão e do compositor",
    ),
    (
        "info",
        "system",
        "Informações do sistema",
        "Hardware e versão do sistema",
    ),
    (
        "mouse",
        "devices",
        "Mouse e touchpad",
        "Botões, rolagem e sensibilidade",
    ),
    (
        "keyboard",
        "devices",
        "Teclado",
        "Digitação, atalhos e layouts",
    ),
    (
        "gestures",
        "devices",
        "Gestos",
        "Ações de deslizar, pinçar e tocar",
    ),
    (
        "wacom",
        "devices",
        "Mesa digitalizadora",
        "Caneta e calibração",
    ),
    (
        "color",
        "devices",
        "Gerenciamento de cores",
        "Perfis de monitores e impressoras",
    ),
    (
        "thunderbolt",
        "devices",
        "Thunderbolt",
        "Dispositivos e autorização",
    ),
    (
        "network",
        "network",
        "Conexões de rede",
        "Wi-Fi, Ethernet, VPN e proxy",
    ),
    (
        "backgrounds",
        "personalization",
        "Plano de fundo",
        "Imagens e apresentação de slides",
    ),
    (
        "themes",
        "personalization",
        "Temas do desktop",
        "Aplicativos, ícones, cursor e área de trabalho",
    ),
    (
        "fonts",
        "personalization",
        "Fontes do desktop",
        "Tipografia, escala e renderização",
    ),
    (
        "effects",
        "personalization",
        "Efeitos",
        "Animações e transições",
    ),
    (
        "desktop",
        "personalization",
        "Área de trabalho",
        "Ícones e disposição",
    ),
    (
        "panel",
        "personalization",
        "Painéis",
        "Posição, tamanho e comportamento",
    ),
    (
        "applets",
        "personalization",
        "Applets",
        "Complementos do painel",
    ),
    (
        "desklets",
        "personalization",
        "Desklets",
        "Complementos da área de trabalho",
    ),
    (
        "extensions",
        "personalization",
        "Extensões",
        "Recursos adicionais do Cinnamon",
    ),
    (
        "windows",
        "personalization",
        "Janelas",
        "Título, foco, alternância e organização",
    ),
    (
        "tiling",
        "personalization",
        "Organização de janelas",
        "Ajuste e encaixe de janelas",
    ),
    (
        "workspaces",
        "personalization",
        "Espaços de trabalho",
        "Organização das áreas de trabalho",
    ),
    (
        "hotcorner",
        "personalization",
        "Cantos ativos",
        "Ações nos cantos da tela",
    ),
    (
        "default",
        "applications",
        "Aplicativos padrão",
        "Associações e mídias removíveis",
    ),
    (
        "startup",
        "applications",
        "Aplicativos na inicialização",
        "Programas iniciados com a sessão",
    ),
    (
        "actions",
        "applications",
        "Ações do gerenciador de arquivos",
        "Extensões do menu de contexto",
    ),
    (
        "user",
        "accounts",
        "Minha conta",
        "Foto, nome e senha da conta atual",
    ),
    (
        "calendar",
        "datetime",
        "Data e hora",
        "Relógio, fuso horário e calendário",
    ),
    (
        "accessibility",
        "accessibility",
        "Acessibilidade",
        "Visão, audição, digitação e mouse",
    ),
    (
        "privacy",
        "security",
        "Privacidade",
        "Arquivos recentes e histórico",
    ),
    (
        "screensaver",
        "security",
        "Bloqueio de tela",
        "Bloqueio automático e proteção da sessão",
    ),
];

// Candidates are fixed argv-style command lines parsed by GIO, never a shell.
const TOOLS: &[(&str, &str, &str, &[&str])] = &[
    (
        "cinnamon-control-center",
        "system",
        "Central completa do Cinnamon",
        &["cinnamon-settings"],
    ),
    (
        "printers",
        "devices",
        "Impressoras",
        &["system-config-printer"],
    ),
    (
        "bluetooth",
        "devices",
        "Bluetooth",
        &["blueberry", "blueman-manager"],
    ),
    ("disks", "devices", "Discos", &["gnome-disks"]),
    (
        "drivers",
        "devices",
        "Gerenciador de drivers",
        &["cinnamon-driver-manager", "driver-manager"],
    ),
    (
        "nvidia",
        "devices",
        "Configurações NVIDIA",
        &["nvidia-settings"],
    ),
    (
        "languages",
        "datetime",
        "Idiomas e pacotes de idioma",
        &["mintlocale", "gnome-language-selector"],
    ),
    (
        "input-method",
        "datetime",
        "Método de entrada",
        &["mintlocale-im"],
    ),
    (
        "users",
        "accounts",
        "Usuários e grupos",
        &["cinnamon-settings-users", "mintsysadm --user"],
    ),
    (
        "account-details",
        "accounts",
        "Detalhes da conta",
        &["mintsysadm-settings-user", "cinnamon-settings user"],
    ),
    (
        "online-accounts",
        "accounts",
        "Contas online",
        &["gnome-online-accounts-gtk"],
    ),
    (
        "login",
        "accounts",
        "Janela de início de sessão",
        &[
            "pkexec lightdm-settings",
            "lightdm-gtk-greeter-settings-pkexec",
            "gdm-settings",
        ],
    ),
    (
        "fingerprints",
        "security",
        "Impressões digitais",
        &["fingwit"],
    ),
    (
        "firewall",
        "security",
        "Firewall",
        &["gufw", "firewall-config"],
    ),
    (
        "administration",
        "system",
        "Administração do sistema",
        &["pkexec mintsysadm"],
    ),
    (
        "reports",
        "system",
        "Relatórios do sistema",
        &["mintreport"],
    ),
    (
        "services",
        "system",
        "Serviços do sistema",
        &["systemd-manager-pkexec"],
    ),
    (
        "software",
        "applications",
        "Gerenciador de aplicativos",
        &["mintinstall", "pamac-manager", "gnome-software"],
    ),
    (
        "packages",
        "applications",
        "Gerenciador de pacotes",
        &[
            "pkexec synaptic",
            "dnfdragora",
            "gpk-application",
            "octopi",
            "pamac-manager",
            "yumex",
        ],
    ),
    (
        "updates",
        "updates",
        "Gerenciador de atualizações",
        &["mintupdate"],
    ),
    (
        "sources",
        "updates",
        "Fontes de aplicativos",
        &["pkexec mintsources", "software-properties-gtk"],
    ),
    (
        "snapshots",
        "updates",
        "Restauração do sistema",
        &["timeshift-launcher"],
    ),
    ("backup", "updates", "Cópia de segurança", &["mintbackup"]),
];

fn command_available(command: &str) -> bool {
    let parts: Vec<_> = command.split_whitespace().collect();
    let Some(program) = parts.first() else {
        return false;
    };
    if glib::find_program_in_path(program).is_none() {
        return false;
    }
    if *program == "pkexec" {
        return parts
            .get(1)
            .is_some_and(|p| glib::find_program_in_path(p).is_some());
    }
    if command == "cinnamon-settings user" {
        return Path::new(MODULE_DIR).join("cs_user.py").is_file();
    }
    true
}

impl Target {
    fn command(&self) -> Option<String> {
        match self {
            Self::Module(id) => {
                let desktop_id = format!("cinnamon-{id}-panel.desktop");
                let exists = Path::new(MODULE_DIR).join(format!("cs_{id}.py")).is_file()
                    || gio::DesktopAppInfo::new(&desktop_id).is_some();
                (exists && command_available("cinnamon-settings"))
                    .then(|| format!("cinnamon-settings {id}"))
            }
            Self::Commands(commands) => commands
                .iter()
                .find(|c| command_available(c))
                .map(|c| (*c).into()),
            Self::Desktop(_) => None,
        }
    }
    pub fn available(&self) -> bool {
        match self {
            Self::Desktop(app) => glib::find_program_in_path(app.executable()).is_some(),
            _ => self.command().is_some(),
        }
    }
    pub fn launch(&self) -> Result<(), String> {
        let context = gtk::gdk::Display::default().map(|d| d.app_launch_context());
        match self {
            Self::Desktop(app) => app.launch(&[], context.as_ref()),
            _ => {
                let command = self
                    .command()
                    .ok_or("O componente não está instalado nesta sessão.")?;
                gio::AppInfo::create_from_commandline(&command, None, gio::AppInfoCreateFlags::NONE)
                    .and_then(|app| app.launch(&[], context.as_ref()))
            }
        }
        .map_err(|_| {
            "Não foi possível abrir as configurações. Verifique o componente instalado.".into()
        })
    }
}

pub fn users_target() -> Target {
    Target::Commands(&["cinnamon-settings-users", "mintsysadm --user"])
}

pub fn search_category(category: &str, query: &str) -> bool {
    TOOLS.iter().any(|(_, group, title, _)| {
        *group == category && crate::navigation::normalize(&crate::i18n::tr(title)).contains(query)
    })
}

pub fn catalog() -> Vec<Setting> {
    let mut result: Vec<_> = MODULES
        .iter()
        .map(|(id, category, title, description)| Setting {
            id: (*id).into(),
            category,
            title: (*title).into(),
            description: (*description).into(),
            target: Target::Module((*id).into()),
        })
        .collect();
    result.extend(TOOLS.iter().map(|(id, category, title, commands)| Setting {
        id: (*id).into(),
        category,
        title: (*title).into(),
        description: "Abrir no aplicativo do sistema".into(),
        target: Target::Commands(commands),
    }));
    // Cover future/distribution-specific Python panels without importing or executing them.
    if let Ok(entries) = std::fs::read_dir(MODULE_DIR) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if let Some(id) = name.strip_prefix("cs_").and_then(|s| s.strip_suffix(".py")) {
                if id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
                    && !result.iter().any(|s| s.id == id)
                {
                    let app = gio::DesktopAppInfo::new(&format!("cinnamon-settings-{id}.desktop"));
                    result.push(Setting {
                        id: id.into(),
                        category: "system",
                        title: app
                            .map(|a| a.display_name().to_string())
                            .unwrap_or_else(|| id.replace('_', " ")),
                        description: "Configuração adicional do Cinnamon".into(),
                        target: Target::Module(id.into()),
                    });
                }
            }
        }
    }
    // Installed Settings launchers extend coverage for tools shipped by other Mint versions.
    let known_programs: HashSet<_> = TOOLS
        .iter()
        .flat_map(|(_, _, _, commands)| commands.iter())
        .filter_map(|c| c.split_whitespace().find(|p| *p != "pkexec"))
        .collect();
    for app in gio::AppInfo::all() {
        let Ok(app) = app.downcast::<gio::DesktopAppInfo>() else {
            continue;
        };
        let id = app.id().unwrap_or_default().to_string();
        if id.starts_with("org.monolito.") {
            continue;
        }
        let extra_panel = id
            .strip_prefix("cinnamon-")
            .and_then(|id| id.strip_suffix("-panel.desktop"))
            .is_some_and(|module| !MODULES.iter().any(|(id, _, _, _)| *id == module));
        if !extra_panel && (id.starts_with("cinnamon-") || !app.should_show()) {
            continue;
        }
        if !app
            .categories()
            .is_some_and(|c| c.split(';').any(|s| s == "Settings"))
        {
            continue;
        }
        let exe = app.executable();
        let program = exe.file_name().and_then(|p| p.to_str()).unwrap_or_default();
        if !extra_panel && known_programs.contains(program) {
            continue;
        }
        result.push(Setting {
            id,
            category: "system",
            title: app.display_name().into(),
            description: "Configuração adicional instalada".into(),
            target: Target::Desktop(app),
        });
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn routes_are_unique_and_have_existing_categories() {
        let mut ids = HashSet::new();
        for (id, category) in MODULES
            .iter()
            .map(|(id, category, _, _)| (id, category))
            .chain(TOOLS.iter().map(|(id, category, _, _)| (id, category)))
        {
            assert!(ids.insert(*id));
            assert!(crate::navigation::CATEGORIES
                .iter()
                .any(|c| c.id == *category));
        }
    }
}
