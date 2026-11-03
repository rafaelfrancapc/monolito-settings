#!/bin/sh
set -eu
cd "$(dirname "$0")/.."

version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n 1)
test -n "$version"
arch=${DEB_ARCH:-$(dpkg --print-architecture)}
destination=${1:-dist}
mkdir -p "$destination"
stage=$(mktemp -d "${TMPDIR:-/tmp}/monolito-settings.XXXXXX")
trap 'rm -rf "$stage"' EXIT HUP INT TERM

cargo build --release --locked
DESTDIR="$stage" ./scripts/install.sh
install -Dm644 packaging/debian/cinnamon-settings.desktop \
    "$stage/usr/share/applications/cinnamon-settings.desktop"
install -d -m755 "$stage/DEBIAN"
sed -e "s/@VERSION@/$version/g" -e "s/@ARCH@/$arch/g" \
    packaging/debian/control.in >"$stage/DEBIAN/control"
for script in preinst postinst postrm; do
    install -m755 "packaging/debian/$script" "$stage/DEBIAN/$script"
done

package="$destination/monolito-settings_${version}_${arch}.deb"
dpkg-deb --build --root-owner-group "$stage" "$package"
printf '%s\n' "$package"
