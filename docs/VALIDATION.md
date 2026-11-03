# Validação

## Integração Cinnamon e apresentação corporativa

Validação em contêiner Debian 13 com GTK 4.18.6 e Rust 1.85.1:

- Formatação, compilação e testes Rust passaram (8 testes; 2 testes de integração executados separadamente).
- Teste de hostname em D-Bus privado passou: nome permanente/ativo, confirmação,
  autorização recusada e falha parcial. Não utiliza o barramento de sistema.
- Oito testes Python passaram, incluindo a opção RC4 independente e os testes de rollback.
- O teste gráfico passou em XFCE/X11 e com detecção Cinnamon e esquemas reais
  cinnamon-desktop 6.4.1. O backend GSettings em memória valida alterações em ambas
  as direções sem gravar preferências de usuário. Também cobre a observação externa de
  modo claro/escuro, cor, tema GTK, ícones e fonte, sem seletor de cor próprio.
- O pacote `monolito-settings_0.2.0_amd64.deb` foi inspecionado e passou por ciclo
  de desempacotamento/remoção em Debian trixie. O `dpkg-divert` ocultou o launcher
  principal do Cinnamon e restaurou seu conteúdo original na remoção.
- Capturas de teste inspecionadas: prévia sem marca, cards com contorno discreto,
  contas locais e hostname. Estrutura e onze categorias preservadas.

Não equivale a homologação completa em Linux Mint/Cinnamon: utilitários externos,
hardware, Wayland, polkit interativo e AD real precisam ser verificados no destino.
Os registros e imagens a seguir são históricos da primeira entrega.

## Validação da primeira entrega

Ambiente: contêiner Debian 13, Rust/Cargo 1.85.1, GTK 4.18.6,
xfwm4 4.20.0, Xvfb/X11, sessão D-Bus. Interface e testes gráficos executados
como usuário comum (UID 1000). Os testes do auxiliar privilegiado utilizam
arquivos temporários e comandos simulados dentro do contêiner.

| Verificação | Resultado |
| --- | --- |
| `cargo fmt --check` | Passou |
| `cargo check --locked` | Passou, sem avisos |
| `cargo build --locked` | Passou |
| `cargo build --release --locked -j 2` | Passou |
| `cargo test --locked` | 3 passaram; teste gráfico ignorado neste comando |
| `scripts/gui-test.sh` | 1 teste gráfico passou separadamente |
| `python3 -m unittest discover -s tests -v` | 7 passaram no contêiner |
| Instalação com `DESTDIR=/tmp/monolito-package` | Executável, ícone, desktop, auxiliar e política instalados |
| Propriedade/permissão do auxiliar instalado | root, 0755 |
| Contraste dos tokens de texto | 13 pares verificados, todos ≥ 4,5:1 |

O teste GTK abre uma janela real e verifica as onze páginas, pesquisa sem acentos,
resultado vazio, restauração da lista, temas claro/escuro/automático, maximização,
restauração, minimização/reativação, tamanho 800 × 600 e fechamento. O carregador SVG
é exercitado para detectar a ausência de librsvg. Não depende de root nem modifica
as preferências de uma sessão pessoal.

Verificação adicional por eventos X11: duplo clique na barra criou e removeu os
estados `_NET_WM_STATE_MAXIMIZED_HORZ` e `_NET_WM_STATE_MAXIMIZED_VERT`. Arraste na
barra moveu a janela de (0, 0) para (150, 80). O movimento é delegado ao GTK/GDK.
As capturas em `screenshots/` mostram a aplicação real, não mockups.

No Xvfb sem compositor, o renderer Cairo do GTK apresentou artefatos de redesenho
na sidebar após transições. A inspeção com OpenGL não apresentou o problema.
O aplicativo e o roteiro de teste deixam o GTK escolher o renderer; o programa não
impõe Cairo. O teste gráfico também passou com o renderer padrão.

## Limites

- Não houve conexão a um controlador AD real nem autenticação de domínio.
- Polkit/realmd são integrados pelo auxiliar, mas a autenticação interativa e o fluxo
  completo devem ser homologados numa VM com agente polkit e domínio acessível.
- Os testes do auxiliar cobrem entradas inválidas, escopo SSSD, preservação de realms
  Kerberos, repetibilidade, rollback após falha e resultado parcial após ingresso.
- O teste de instalação usa uma árvore temporária; não ingressa o contêiner em AD,
  não instala políticas na sessão do usuário e não altera seus arquivos de sistema.
- Não foi testado o tema Chicago95 nem o hardware da máquina Arch descrita no anexo.
- Ferramentas externas ausentes no contêiner aparecem desabilitadas e continuam
  dependendo dos pacotes correspondentes no Linux Mint.

## Refinamento visual e editorial

- Removidos o texto da barra superior e o rodapé da navegação com marca/versão.
- Item selecionado com fundo opaco na cor de destaque e texto/ícone no token
  `on_accent`, sem faixa lateral. A aparência segue automaticamente o Mint.
- A prévia não contém marca textual e usa apenas tipografia de interface com fallback.
- Descrições e atalhos revisados para linguagem funcional e profissional.
- `cargo fmt --check`, `cargo check --locked`, compilação normal/release e os
  três testes Rust passaram. O teste gráfico existente passou separadamente.
- Capturas clara/escura atualizadas a partir da aplicação em execução com OpenGL.
- A barra sem título manteve o duplo clique para maximizar/restaurar e o arraste
  no XFWM: janela maximizada em 1400 × 900 e restaurada em 1100 × 720;
  arraste confirmado até (150, 80).
- Instalação em árvore temporária validada, incluindo executável, ícone e integrações.
