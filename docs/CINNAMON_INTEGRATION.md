# Configurações no Cinnamon e Linux Mint

O nome exibido é **Configurações**. O executável `monolito-settings`, o ID
`org.monolito.Settings`, os caminhos de preferências e a política polkit mantêm
sua identidade técnica. As onze categorias e a estrutura da janela foram preservadas.

## Cobertura

O catálogo em `src/backends/catalog.rs` distribui os módulos do Cinnamon nas
categorias existentes. Foi confrontado com os módulos Python, os painéis do
cinnamon-control-center e a lista de aplicativos externos de
[cinnamon-settings](https://github.com/linuxmint/cinnamon/blob/master/files/usr/share/cinnamon/cinnamon-settings/cinnamon-settings.py).

| Categoria | Acessos |
| --- | --- |
| Sistema | Monitores, luz noturna, som, energia, notificações, geral, informações, relatórios, administração, serviços e central completa do Cinnamon |
| Dispositivos | Mouse/touchpad, teclado, gestos, mesa digitalizadora, cores, Thunderbolt, impressoras, Bluetooth, discos, drivers e NVIDIA |
| Rede e Internet | Painel de rede com Wi-Fi, Ethernet, VPN e proxy |
| Personalização | Plano de fundo, temas, fontes, efeitos, área de trabalho, painéis, applets, desklets, extensões, janelas, encaixe, espaços de trabalho e cantos ativos |
| Aplicativos | Padrões, mídias removíveis, inicialização, ações do Nemo, gerenciadores de aplicativos e pacotes |
| Contas | Contas locais, nome/senha/grupos no gerenciador nativo, conta atual, contas online, tela de login e Active Directory |
| Hora e Idioma | Data/hora, calendário, idiomas e métodos de entrada |
| Acessibilidade | Visão, audição, teclado e mouse |
| Segurança | Privacidade, bloqueio, firewall e impressões digitais |
| Atualizações | Atualizações, fontes, snapshots e cópias de segurança |

A instalação pode não fornecer todos esses componentes. Os acessos conhecidos continuam
visíveis, com “Não instalado” quando o destino não existe. O catálogo também descobre
módulos Python adicionais, painéis C e launchers de configurações instalados. Extensões
desconhecidas ficam em Sistema. A busca localiza categorias, recursos e ferramentas.

Os cards de configuração usam GIO para abrir o módulo ou aplicativo específico. Executáveis
alternativos são resolvidos por disponibilidade (por exemplo, Blueberry/Blueman,
GUFW/firewall-config e versões novas/antigas da administração de contas). Não há shell
nem incorporação de janelas GTK3 dentro da janela GTK4. Aplicativos externos permanecem
processos separados, conforme a estratégia de integração gradual. O sucesso de lançamento
significa que GIO aceitou o pedido, não que uma operação administrativa foi executada.

## Controles diretos

Na sessão Cinnamon, as seguintes preferências são alteradas na própria janela:

- notificações e notificações sobre janelas em tela cheia;
- mouse para canhotos, rolagem natural e localização do ponteiro;
- toque para clicar, bloqueio ao digitar e rolagem natural do touchpad;
- animações, barras de rolagem, efeitos de janelas e animação de início da sessão;
- ampliador, teclado na tela e leitor de tela;
- arquivos recentes e bloqueio da proteção de tela.

Cada controle exige esquema, chave e tipo compatíveis. O vínculo GIO/GSettings mantém
o valor sincronizado nos dois sentidos e respeita a possibilidade de escrita da chave
(incluindo bloqueios administrativos). Os controles não disponíveis são omitidos; o
acesso ao painel completo continua presente. Recursos como leitor de tela também
dependem dos respectivos componentes da sessão.

O card inteiro “Cores, modo e temas” abre o painel Temas do Mint, onde ficam accent
color e os modos misto, claro e escuro. O aplicativo observa `org.x.apps.portal`,
`org.cinnamon.desktop.interface`, `org.cinnamon.theme` e o catálogo `styles.d` para
acompanhar automaticamente modo, cor, GTK, ícones, cursor e fonte. Não há subprocessos
`gsettings` nem um segundo seletor de cor concorrente.

Idiomas e pacotes de idioma abrem `mintlocale`, com fallback para
`gnome-language-selector`. A instalação e atualização dos language packs permanecem
com o gerenciador da distribuição, respeitando seus repositórios, locale e dependências.

## Contas e domínio

A lista lê exclusivamente `/etc/passwd`, sem NSS, getent, SSSD ou winbind. Isso exclui
contas provenientes do Active Directory. Todas as entradas locais são preservadas:
usuários comuns aparecem primeiro, e contas de serviço/root ficam em “Contas de sistema”.
O limiar segue `UID_MIN` de `/etc/login.defs` (fallback 1000).
“Atualizar usuários” relê a base após mudanças externas.

“Gerenciar ↗” abre Usuários e grupos; selecione ali a conta para alterar nome, senha,
grupos ou remover/criar usuários. O aplicativo não lê senhas nem arquivos shadow.
Não existe parâmetro portátil para pré-selecionar uma conta em todas as versões do
gerenciador nativo.

“Editar sssd.conf” abre `admin:///etc/sssd/sssd.conf` no editor de texto padrão. O
backend administrativo do GVfs solicita autorização e evita executar o editor inteiro
como root. Se o arquivo ainda não existir, o atalho informa essa condição sem criá-lo.

O ingresso AD preserva descoberta, consulta de vínculo, confirmação, nomes não
qualificados, polkit, backups e tratamento de falhas parciais. A interface apresenta
somente os campos e avisos essenciais.

“Aplicar nome” usa `org.freedesktop.hostname1` no barramento de sistema, com autorização
interativa. Atualiza o nome permanente e o ativo e confirma ambos por D-Bus. O nome segue
um rótulo DNS de até 15 caracteres para compatibilidade AD/NetBIOS. Em falha parcial,
a interface informa que o nome permanente pode ter sido alterado. Um nome pendente
de aplicação impede o ingresso.

As opções legadas começam desmarcadas e são independentes:

- `allow_weak_crypto`: grava essa opção em `[libdefaults]`;
- RC4/arcfour: grava `allow_rc4 = true`, opção disponível em MIT Kerberos recente
  (1.21+). Não força listas de cifras nem ultrapassa políticas criptográficas do sistema.

Desmarcar preserva a política existente; não desabilita configurações anteriores.
Kerberos antigo pode ignorar `allow_rc4`, e uma lista explícita de cifras que exclua
RC4 continua prevalecendo. Falha de ingresso restaura o arquivo anterior; se o ingresso
já ocorreu, o resultado parcial é informado sem executar `realm leave`.
Reinstale o auxiliar junto com o aplicativo para atualizar o protocolo.

## Validação e limites

- Testes Rust: catálogo/categorias, detecção do desktop, contas locais e validação DNS/hostname.
- Teste D-Bus em sessão privada: atualização e confirmação dos dois nomes, autorização
  recusada e falha parcial, sem acessar o barramento de sistema.
- Testes Python: protocolo estrito, SSSD, Kerberos, opt-in RC4, idempotência e rollback.
- Teste GTK/X11: navegação, busca por recursos, aparência, janela compacta e controles
  de janela. Executado em XFCE e com detecção Cinnamon + esquemas reais 6.4.1,
  usando backend GSettings em memória; valida alteração de controles e mudanças externas.

Esses testes não representam uma sessão completa do Linux Mint. Abertura e comportamento
de cada utilitário Mint, autorização polkit real, hardware, Wayland e ingresso em um AD
real ainda precisam de homologação no destino. Não foram alteradas configurações reais
do computador nem instalado o aplicativo durante a validação.

O instalador simples mantém uma entrada própria. O pacote Debian usa `dpkg-divert`
para preservar e ocultar o launcher principal do Cinnamon enquanto o pacote estiver
instalado. A remoção restaura o arquivo original. O executável `cinnamon-settings`
continua acessível pelas rotas internas para módulos ainda especializados.
