# Configurações

Painel de configurações nativo em **Rust + GTK4**, inspirado na organização visual
do Windows 11, com integração ao Cinnamon/Linux Mint e leitura de informações no XFCE.
Não usa WebView nem libadwaita. O executável continua sendo `monolito-settings` e o
identificador é `org.monolito.Settings`.

[Cobertura e integração com Cinnamon](docs/CINNAMON_INTEGRATION.md) · [Validação](docs/VALIDATION.md)

## Compilar no Arch Linux

```sh
sudo pacman -S --needed base-devel rust gtk4 librsvg pkgconf inter-font
cargo build --locked
cargo run --locked
```

Alternativamente, use Rust estável via rustup, evitando instalar dois toolchains sem
necessidade. Requer Rust >= 1.83 e GTK >= 4.8. A interface usa Inter com fallback
para as fontes disponíveis na sessão, sem fonte de marca.
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

Para gerar o pacote Debian:

```sh
./scripts/build-deb.sh
sudo apt install ./dist/monolito-settings_0.2.0_amd64.deb
```

O pacote usa uma diversão reversível do `dpkg` para ocultar a entrada principal
`cinnamon-settings.desktop`. O arquivo original é restaurado ao remover o pacote;
os módulos do Cinnamon continuam disponíveis pelos acessos internos da aplicação.

O instalador adiciona o executável, a entrada de menu e o ícone,
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
- No Cinnamon, modos claro/escuro/automático e accent color usam o portal XApp;
  tema GTK, ícones, cursor e fonte usam `org.cinnamon.desktop.interface`.
  Famílias descritas em `styles.d` são trocadas como um conjunto coordenado.
  Alterações externas atualizam a interface. Fora do Cinnamon, o aplicativo mantém
  fallback local em `$XDG_CONFIG_HOME/monolito/settings.json`.
- CSS com cores semânticas, ícones SVG originais incorporados e fontes com fallback.
  A fonte padrão de 10,5 pt corresponde a 14 px em 96 DPI e respeita a escala GTK.
- Início/Sistema leem hostname, distribuição, kernel, CPU, memória e sessão reais.
- O backend é selecionado por `XDG_CURRENT_DESKTOP`. No XFCE, lê `/Net/ThemeName`,
  `/Net/IconThemeName` e `/Gtk/FontName` do canal
  `xsettings` via `xfconf-query` (cliente da API xfconf/D-Bus), com limite de tempo.
  No Cinnamon, lê tema GTK, tema de ícones e fonte de
  `org.cinnamon.desktop.interface` via GSettings. Ausência de serviços é apresentada
  explicitamente. Controles diretos usam GIO/GSettings com validação de esquema/chave/tipo
  e sincronização de alterações externas.
- Contas: todos os usuários locais (sem enumerar contas AD), gerenciamento pelo utilitário
  nativo, hostname via D-Bus/polkit, descoberta AD, consulta de vínculo e ingresso.
- Animações seguem `gtk-enable-animations`. Interface inicial em pt_BR, com caminho
  gettext para outros idiomas (`po/`).

Todas as categorias contêm acessos funcionais aos componentes instalados do Cinnamon/Mint.
Notificações, mouse/touchpad, fonte do desktop, efeitos, acessibilidade e privacidade
têm controles diretos. Recursos avançados abrem o módulo ou aplicativo nativo específico.
Componentes ausentes são identificados; módulos adicionais instalados são descobertos
automaticamente. A pesquisa também localiza os recursos dentro das categorias.
Hora e Idioma abre `mintlocale` ou o seletor de idiomas disponível, que instala os
language packs atuais dos repositórios configurados pela distribuição.

O inventário técnico, a matriz de integrações e o plano de substituição progressiva do
centro de configurações do Cinnamon estão em [`docs/CINNAMON_INTEGRATION.md`](docs/CINNAMON_INTEGRATION.md).
O painel original deve permanecer instalado e acessível até a homologação de cada módulo.

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

Em **Contas → Domínio corporativo**:

1. Se necessário, altere o nome do computador e clique em **Aplicar nome**.
   Depois informe o domínio DNS e use **Localizar domínio** para verificar a descoberta.
2. Informe administrador e senha do domínio.
3. Escolha, se necessário, as opções abaixo e confirme o ingresso.
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
- **RC4 (arcfour):** grava `allow_rc4 = true` em `[libdefaults]`, separadamente da
  opção anterior. Requer suporte do Kerberos (MIT 1.21+) e não modifica a lista de
  cifras permitidas nem supera políticas do sistema. Desmarcada, preserva o arquivo.

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
dbus-run-session -- cargo test --locked hostname_service_contract -- --ignored
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
