# Traduções

Os textos de interface passam por `i18n::tr`, que usa `glib::dgettext` no domínio
`monolito-settings`. O catálogo fonte é pt_BR; traduções podem usar qualquer locale.
Instale catálogos compilados em `/usr/share/locale/<locale>/LC_MESSAGES/monolito-settings.mo`.
A locale segue o GTK/GLib. As constantes de navegação e os conjuntos de linhas também
precisam ser incluídos na extração: `scripts/extract-messages.py` gera um POT a partir
dos literais de interface, inclusive tabelas declarativas. Revise o POT antes de traduzir.
Mensagens de erro originadas no SO e dados de hardware não são traduzidos.
