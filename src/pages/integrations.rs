use crate::{
    backends::{catalog, desktop_settings, DesktopBackend},
    components::*,
    i18n::tr,
};
use gtk::prelude::*;

// Documented keys used by Cinnamon's own settings modules. Everything else has a native route.
const SWITCHES: &[(&str, &str, &str, &str)] = &[
    (
        "system",
        "Exibir notificações",
        "org.cinnamon.desktop.notifications",
        "display-notifications",
    ),
    (
        "system",
        "Notificações em tela cheia",
        "org.cinnamon.desktop.notifications",
        "fullscreen-notifications",
    ),
    (
        "devices",
        "Mouse para canhotos",
        "org.cinnamon.desktop.peripherals.mouse",
        "left-handed",
    ),
    (
        "devices",
        "Rolagem natural do mouse",
        "org.cinnamon.desktop.peripherals.mouse",
        "natural-scroll",
    ),
    (
        "devices",
        "Localizar ponteiro com Ctrl",
        "org.cinnamon.desktop.peripherals.mouse",
        "locate-pointer",
    ),
    (
        "devices",
        "Tocar para clicar",
        "org.cinnamon.desktop.peripherals.touchpad",
        "tap-to-click",
    ),
    (
        "devices",
        "Desativar touchpad ao digitar",
        "org.cinnamon.desktop.peripherals.touchpad",
        "disable-while-typing",
    ),
    (
        "devices",
        "Rolagem natural do touchpad",
        "org.cinnamon.desktop.peripherals.touchpad",
        "natural-scroll",
    ),
    (
        "personalization",
        "Animações da interface",
        "org.cinnamon.desktop.interface",
        "enable-animations",
    ),
    (
        "personalization",
        "Barras de rolagem sobrepostas",
        "org.cinnamon.desktop.interface",
        "gtk-overlay-scrollbars",
    ),
    (
        "personalization",
        "Efeitos de janela",
        "org.cinnamon",
        "desktop-effects",
    ),
    (
        "personalization",
        "Animação ao iniciar a sessão",
        "org.cinnamon",
        "startup-animation",
    ),
    (
        "accessibility",
        "Alto contraste",
        "org.x.apps.portal",
        "high-contrast",
    ),
    (
        "accessibility",
        "Ampliador de tela",
        "org.cinnamon.desktop.a11y.applications",
        "screen-magnifier-enabled",
    ),
    (
        "accessibility",
        "Teclado na tela",
        "org.cinnamon.desktop.a11y.applications",
        "screen-keyboard-enabled",
    ),
    (
        "accessibility",
        "Leitor de tela",
        "org.cinnamon.desktop.a11y.applications",
        "screen-reader-enabled",
    ),
    (
        "security",
        "Lembrar arquivos recentes",
        "org.cinnamon.desktop.privacy",
        "remember-recent-files",
    ),
    (
        "security",
        "Bloquear ao ativar a proteção de tela",
        "org.cinnamon.desktop.screensaver",
        "lock-enabled",
    ),
];

pub fn launch_button(target: catalog::Target, status: &gtk::Label) -> gtk::Button {
    let b = button("Abrir ↗");
    b.set_tooltip_text(Some(&tr("Abrir no aplicativo do sistema")));
    b.set_sensitive(target.available());
    if !b.is_sensitive() {
        b.set_tooltip_text(Some(&tr("Componente não instalado")));
    }
    let status = status.clone();
    b.connect_clicked(move |_| {
        status.set_text(&match target.launch() {
            Ok(()) => tr("Configurações abertas no aplicativo do sistema."),
            Err(error) => tr(&error),
        });
    });
    b
}

pub fn launch_card(
    title: &str,
    description: &str,
    icon_name: &str,
    target: catalog::Target,
    status: &gtk::Label,
) -> gtk::Button {
    let row = system_row(title, description, icon_name);
    let available = target.available();
    if available {
        let next = icon("chevron", 16);
        row.append(&next);
    } else {
        row.append(&label("Não instalado", "badge"));
    }
    let button = gtk::Button::new();
    button.set_child(Some(&row));
    button.set_sensitive(available);
    button.set_tooltip_text(Some(&tr(if available {
        "Abrir configuração"
    } else {
        "Componente não instalado"
    })));
    accessible(&button, &format!("{} — {}", tr("Abrir"), tr(title)));
    let status = status.clone();
    button.connect_clicked(move |_| {
        status.set_text(&match target.launch() {
            Ok(()) => tr("Configurações abertas no aplicativo do sistema."),
            Err(error) => tr(&error),
        });
    });
    button
}

pub fn append(body: &gtk::Box, category: &str) {
    let status = label("", "secondary");
    if DesktopBackend::detect() == DesktopBackend::Cinnamon {
        let direct = card();
        for (_, title, schema, key) in SWITCHES.iter().filter(|s| s.0 == category) {
            if let Some(settings) = desktop_settings::setting(schema, key, "b") {
                let r = system_row(title, "", catalog::direct_icon(schema));
                let switch = gtk::Switch::new();
                switch.set_widget_name(&format!("{schema}/{key}"));
                switch.set_valign(gtk::Align::Center);
                accessible(&switch, title);
                settings.bind(key, &switch, "active").build();
                r.append(&switch);
                direct.append(&r);
            }
        }
        if direct.first_child().is_some() {
            body.append(&label("Preferências do desktop", "section-title"));
            body.append(&direct);
        }
    }
    body.append(&label("Configurações do sistema", "section-title"));
    let routes = card();
    for setting in catalog::catalog()
        .into_iter()
        .filter(|s| s.category == category && !(category == "personalization" && s.id == "themes"))
    {
        routes.append(&launch_card(
            &setting.title,
            &setting.description,
            catalog::icon_name(&setting.id),
            setting.target,
            &status,
        ));
    }
    body.append(&routes);
    body.append(&status);
}
