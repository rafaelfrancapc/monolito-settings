# Monolito Settings

Painel de configurações nativo em **Rust + GTK4**, inspirado na organização visual
do Windows 11 e preparado para o Monolito Desktop sobre XFCE/X11. Não usa WebView,
libadwaita nem recursos gráficos proprietários. Esta é a primeira versão da interface,
não uma substituição de toda a infraestrutura do XFCE.

![Personalização do Monolito](docs/screenshots/personalization-light.png)

[Visual escuro](docs/screenshots/personalization-dark.png) · [Opções de Active Directory](docs/screenshots/accounts-options.png) · [Evidências de validação](docs/VALIDATION.md)

## Compilar no Arch Linux

```sh
sudo pacman -S --needed base-devel rust gtk4 librsvg pkgconf inter-font
cargo build --locked
cargo run --locked
```

Alternativamente, use Rust estável via rustup, evitando instalar dois toolchains sem
necessidade. Requer Rust >= 1.83 e GTK >= 4.8. A marca utiliza **monolito**, em minúsculas, com Montserrat Alternates Bold.
O instalador inclui a fonte, distribuída sob a SIL Open Font License. Para execução
direta pelo Cargo, instale a fonte incluída antes de abrir o aplicativo:

```sh
install -Dm644 resources/fonts/MontserratAlternates-Bold.ttf \
  ~/.local/share/fonts/monolito/MontserratAlternates-Bold.ttf
fc-cache -f ~/.local/share/fonts/monolito
```

Há fallback de fontes para sistemas em que ela ainda não tenha sido instalada.
O ambiente informado pelo usuário (GTK 4.24.1 / XFCE 4.20 / xfwm4 4.20 / X11 / tema
Chicago95) é compatível com as APIs escolhidas. A validação local utilizou GTK 4.18.6,
Rust 1.85.1 e xfwm4 4.20.0 no Debian 13 em contêiner.

No Debian/Ubuntu:

```sh
sudo apt install cargo rustc rustfmt build-essential pkg-config libgtk-4-dev librsvg2-common
cargo run --locked
```

Execute a interface **como usuário comum**, na sessão gráfica existente. A aplicação
não altera nem remove xfce4-session, xfwm4, xfconf, xfsettingsd, xfdesktop ou xfce4-panel.
Não exige um worktree separado: utilize o checkout existente.

## Instalar

```sh
cargo build --release --locked
sudo ./scripts/install.sh
```

O instalador adiciona o executável, a entrada de menu, o ícone, a fonte da marca,
o auxiliar de domínio e a política polkit. Não instala dependências e não substitui o painel original.
Para empacotamento, `DESTDIR=/caminho ./scripts/install.sh` permite montar uma árvore
de pacote. Em produção, os arquivos instalados devem pertencer a root, e o auxiliar
não pode ser gravável pelo usuário da sessão. Nunca execute o auxiliar a partir de um
checkout gravável pelo usuário como destino de uma política polkit.

## Implementado

- Janela 1100 × 720, sidebar fixa e onze categorias com pesquisa sem distinção de acentos.
  A seleção utiliza preenchimento opaco na cor de destaque, com texto e ícone em
  contraste, sem faixa lateral. A barra superior não exibe marca; não há rodapé de versão.
- Decoração própria com `GtkWindowHandle`, minimizar, maximizar/restaurar e fechar.
  O GTK/GDK cuida de arraste e duplo clique, sem manipulação manual de coordenadas.
- Navegação por `GtkStack`, rolagem por página, atalhos reais no Início e foco por teclado.
- Temas claro, escuro e automático; quatro cores de destaque; fonte e tema de ícones.
  Essas preferências são **do aplicativo**, persistidas em
  `$XDG_CONFIG_HOME/monolito/settings.json` (fallback `~/.config`).
- Automático acompanha o nome do tema GTK e `gtk-application-prefer-dark-theme`.
  Não consulta ainda o portal `org.freedesktop.appearance`.
- CSS com cores semânticas, ícones SVG originais incorporados e fontes com fallback.
  A fonte padrão de 10,5 pt corresponde a 14 px em 96 DPI e respeita a escala GTK.
- Início/Sistema leem hostname, distribuição, kernel, CPU, memória e sessão reais.
- Backend XFCE lê `/Net/ThemeName`, `/Net/IconThemeName` e `/Gtk/FontName` do canal
  `xsettings` via `xfconf-query` (cliente da API xfconf/D-Bus), com limite de tempo.
  Ausência de serviços é apresentada explicitamente. Não há escrita em xfconf nesta versão.
- Contas: usuário atual, descoberta AD, consulta de vínculo e ingresso via auxiliar polkit.
- Animações seguem `gtk-enable-animations`. Interface inicial em pt_BR, com caminho
  gettext para outros idiomas (`po/`).

Dispositivos, Rede, Aplicativos, Hora e Idioma, Acessibilidade, Segurança e Atualizações
são páginas estruturadas **em desenvolvimento**. Seus ajustes não fingem aplicar
configurações reais. NetworkManager, BlueZ, gerenciador de pacotes e edição das contas
locais permanecem para versões posteriores.

## Active Directory

Instale no sistema de destino `realmd`, `adcli`, SSSD (incluindo `sssctl`), Kerberos,
polkit, Python 3 e um agente de autenticação polkit ativo na sessão. No Arch, `realmd`
pode exigir um pacote do AUR: revise e instale com seu fluxo de empacotamento habitual.
No Debian/Ubuntu, os pacotes normalmente são:

```sh
sudo apt install realmd adcli sssd-ad sssd-tools libnss-sss libpam-sss krb5-user policykit-1
```

Pré-requisitos: DNS que resolva o AD, relógio sincronizado, hostname válido, acesso ao
controlador e conta autorizada a ingressar computadores. Configure NSS/PAM conforme a
distribuição; o auxiliar não sobrescreve essa política. Criação automática de diretório
home também depende da configuração PAM da distribuição. Ingresso não garante que
uma sessão gráfica de usuário AD possa iniciar sem essas etapas.

Em **Contas → Acessar trabalho ou escola**:

1. Informe o domínio DNS e use **Localizar domínio** para verificar a descoberta.
2. Informe administrador e senha do domínio.
3. Escolha, se necessário, as duas opções abaixo e confirme o ingresso.
4. Clique em **Ingressar no domínio** e autentique um administrador **local** no polkit.
5. Consulte o vínculo atual; encerre a sessão antes de testar uma conta do domínio.

As opções vêm desmarcadas:

- **Nome não qualificado:** configura `use_fully_qualified_names = false` somente na
  seção SSSD do domínio ingressado. Desmarcada, grava `true`. Nomes iguais entre contas
  locais e de domínio exigem cuidado; não há alteração da política de acesso do domínio.
- **allow_weak_crypto:** quando marcada, grava `allow_weak_crypto = true` em
  `[libdefaults]` de `/etc/krb5.conf`, antes do ingresso. A opção afeta Kerberos no
  computador todo e reduz a proteção; não define tipos de cifra nem restaura algoritmos
  removidos pelo MIT Kerberos ou bloqueados por políticas criptográficas da distribuição.
  Desmarcada, preserva a política Kerberos já existente.

A interface nunca roda como root. O auxiliar usa argumentos fixos, validação estrita,
stdin para a senha e subprocessos sem shell. Não registra credenciais nem repassa a
saída bruta dos comandos. O JSON é limitado a 16 KiB, comandos têm timeout de 120 s,
e operações concorrentes são bloqueadas por flock. Polkit pede autenticação a cada
ingresso e só permite sessões locais ativas; não há autorização permanente.

O ingresso usa `realm join --client-software=sssd --membership-software=adcli`.
SSSD é configurado em seu arquivo local, pois essa opção não dispõe de API xfconf;
`sssctl config-check` valida antes de reiniciar SSSD. Backups com permissão restrita
ficam em `/var/lib/monolito/backups/join-*`. Uma falha após o ingresso é reportada
como parcial e restaura a configuração SSSD anterior; o auxiliar nunca executa
`realm leave` automaticamente. Falhas/timeout do próprio realmd também podem exigir
consultar `realm list`: a ausência de sucesso no cliente não garante rollback no AD.
O auxiliar recusa novo ingresso quando já existe um domínio configurado; editar,
remover ou migrar vínculos é trabalho futuro.

**Validação AD:** testes automatizados cobrem validação, alterações restritas de
configuração, repetibilidade, rollback Kerberos e falha parcial. Não foi realizado
um ingresso real nesta máquina: ela não dispõe de controlador AD nem credenciais.
Valide esse fluxo em uma VM da distribuição e um domínio de teste antes de distribuir.

## Testes

```sh
cargo fmt --check
cargo check --locked
cargo test --locked
python3 -m unittest discover -s tests -v
```

Os dois testes de transação que verificam propriedade root dos backups devem rodar
em um contêiner de teste como root; fora dele ficam explicitamente ignorados.

Para testar a interface sem usar sua sessão pessoal, instale `xvfb`, `xauth`,
`dbus-x11` (ou equivalente), `xfwm4` e execute como usuário comum:

```sh
./scripts/gui-test.sh
```

O teste gráfico abre a janela, percorre as onze páginas, testa pesquisa com acentos,
ausência de resultados, alternância de temas, maximizar/restaurar, minimizar/reativar,
dimensão compacta e fechar. Sem display, esse teste é ignorado no `cargo test` normal.
O teste usa um diretório temporário de preferências.

## Estrutura

`window.rs` monta a janela; `navigation.rs` declara categorias; `components/` concentra
os elementos reutilizáveis; `pages/` contém páginas implementadas e um construtor
compartilhado para categorias futuras. `state.rs` persiste preferências locais;
`theme.rs` e `resources/styles/` definem aparência. `backends/` concentra todas as
operações de sistema; o auxiliar privilegiado e sua política ficam em `helper/` e
`data/`. Nenhum comando específico de XFCE é executado na camada visual.

## Limites desta entrega

Não há blur/Mica do compositor nem reprodução de recursos proprietários do Windows.
Redimensionamento usa CSD nativo do GTK; o tamanho mínimo é 760 × 520, com rolagem.
O tema Chicago95 do computador informado não está instalado no ambiente de teste;
a aplicação usa CSS próprio com prioridade de aplicação, mas uma validação visual
nessa instalação ainda é recomendada. Fontes arbitrárias muito grandes podem aumentar
a largura mínima. Diálogos auxiliares de seleção de fonte são widgets nativos GTK.
