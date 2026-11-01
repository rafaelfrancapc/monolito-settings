#!/bin/sh
# Run after cargo build --release; use sudo for a system install.
set -eu
cd "$(dirname "$0")/.."
install -Dm755 target/release/monolito-settings "${DESTDIR:-}/usr/bin/monolito-settings"
install -Dm644 data/org.monolito.Settings.desktop "${DESTDIR:-}/usr/share/applications/org.monolito.Settings.desktop"
install -Dm644 resources/icons/personalization-symbolic.svg "${DESTDIR:-}/usr/share/icons/hicolor/scalable/apps/org.monolito.Settings.svg"
install -Dm755 helper/monolito-directory-helper "${DESTDIR:-}/usr/libexec/monolito-directory-helper"
install -Dm644 data/org.monolito.Settings.directory.policy "${DESTDIR:-}/usr/share/polkit-1/actions/org.monolito.Settings.directory.policy"
install -Dm644 resources/fonts/MontserratAlternates-Bold.ttf "${DESTDIR:-}/usr/share/fonts/truetype/monolito/MontserratAlternates-Bold.ttf"
install -Dm644 resources/fonts/OFL.txt "${DESTDIR:-}/usr/share/doc/monolito-settings/fonts/OFL.txt"
if [ -z "${DESTDIR:-}" ] && command -v fc-cache >/dev/null 2>&1; then
    fc-cache -f /usr/share/fonts/truetype/monolito
fi
