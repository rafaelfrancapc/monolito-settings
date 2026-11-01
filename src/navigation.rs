use crate::i18n::tr;
#[derive(Clone, Copy)]
pub struct Category {
    pub id: &'static str,
    pub title: &'static str,
    pub icon: &'static str,
    pub description: &'static str,
}
pub const CATEGORIES: &[Category] = &[
    Category {
        id: "home",
        title: "Início",
        icon: "home",
        description: "Visão geral do dispositivo e acesso às configurações.",
    },
    Category {
        id: "system",
        title: "Sistema",
        icon: "system",
        description: "Informações do dispositivo, sistema operacional e sessão gráfica.",
    },
    Category {
        id: "devices",
        title: "Dispositivos",
        icon: "devices",
        description: "Bluetooth, impressoras e dispositivos conectados.",
    },
    Category {
        id: "network",
        title: "Rede e Internet",
        icon: "network",
        description: "Conexões, Wi-Fi e preferências de rede.",
    },
    Category {
        id: "personalization",
        title: "Personalização",
        icon: "personalization",
        description: "Tema, cores e tipografia da interface.",
    },
    Category {
        id: "applications",
        title: "Aplicativos",
        icon: "applications",
        description: "Aplicativos instalados e associações de arquivos.",
    },
    Category {
        id: "accounts",
        title: "Contas",
        icon: "accounts",
        description: "Perfil de usuário e acesso a organizações.",
    },
    Category {
        id: "datetime",
        title: "Hora e Idioma",
        icon: "datetime",
        description: "Data, hora, região e idiomas do sistema.",
    },
    Category {
        id: "accessibility",
        title: "Acessibilidade",
        icon: "accessibility",
        description: "Recursos de acesso e adaptação da interface.",
    },
    Category {
        id: "security",
        title: "Segurança",
        icon: "security",
        description: "Proteção, permissões e privacidade.",
    },
    Category {
        id: "updates",
        title: "Atualizações",
        icon: "updates",
        description: "Disponibilidade e gerenciamento de atualizações.",
    },
];
pub fn normalize(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'â' | 'ã' => 'a',
            'é' | 'ê' => 'e',
            'í' => 'i',
            'ó' | 'ô' | 'õ' => 'o',
            'ú' => 'u',
            'ç' => 'c',
            _ => c,
        })
        .collect()
}
pub fn matches(category: &Category, query: &str) -> bool {
    normalize(&tr(category.title)).contains(&normalize(query))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_accents() {
        assert!(matches(&CATEGORIES[4], "PERSONALIZACAO"));
        assert!(!matches(&CATEGORIES[0], "rede"));
    }
    #[test]
    fn unique_ids() {
        let ids: std::collections::HashSet<_> = CATEGORIES.iter().map(|c| c.id).collect();
        assert_eq!(ids.len(), 11);
    }
}
