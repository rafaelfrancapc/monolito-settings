pub mod accounts;
pub mod home;
pub mod personalization;
pub mod system;
use crate::{components::*, navigation::Category};
use gtk::prelude::*;
pub fn placeholder(c: &Category) -> gtk::ScrolledWindow {
    let (scroll, body) = page(c.title, c.description);
    body.append(&label(
        "Em desenvolvimento — estes ajustes ainda não estão disponíveis.",
        "notice",
    ));
    let card = card();
    let entries: &[(&str, &str)] = match c.id {
        "devices" => &[
            ("Bluetooth", "Conectar e gerenciar dispositivos"),
            ("Impressoras e scanners", "Preferências de impressão"),
            ("Mouse e touchpad", "Velocidade e gestos"),
        ],
        "network" => &[
            ("Wi-Fi e Ethernet", "Integração com NetworkManager prevista"),
            ("VPN", "Conexões de trabalho"),
            ("Proxy", "Configurações de acesso à rede"),
        ],
        "applications" => &[
            ("Aplicativos instalados", "Gerenciar programas"),
            ("Aplicativos padrão", "Associações de arquivos"),
        ],
        "datetime" => &[
            ("Data e hora", "Sincronização e fuso horário"),
            ("Idioma e região", "Idioma de exibição e formatos"),
        ],
        "accessibility" => &[
            ("Tamanho do texto", "Escala da interface"),
            (
                "Efeitos visuais",
                "O aplicativo respeita a preferência GTK por animações",
            ),
        ],
        "security" => &[
            ("Privacidade", "Permissões de aplicativos"),
            ("Firewall", "Integração prevista"),
        ],
        _ => &[(
            "Atualizações do sistema",
            "Integração com o gerenciador de pacotes prevista",
        )],
    };
    for (title, desc) in entries {
        let r = row(title, desc, c.icon);
        r.append(&label("Não disponível", "badge"));
        card.append(&r);
    }
    body.append(&card);
    scroll
}
