#!/bin/sh
# Requires Xvfb, xauth, dbus-run-session, xfwm4. Run as a regular user.
set -eu
cd "$(dirname "$0")/.."
exec dbus-run-session -- xvfb-run -a -s '-screen 0 1400x900x24' sh -eu -c '
  test_root=$(mktemp -d)
  export XDG_CONFIG_HOME="$test_root/config" XDG_CACHE_HOME="$test_root/cache" XDG_DATA_HOME="$test_root/data"
  mkdir -p "$XDG_CONFIG_HOME" "$XDG_CACHE_HOME" "$XDG_DATA_HOME"
  export XDG_CURRENT_DESKTOP="${MONOLITO_TEST_DESKTOP:-XFCE}" XDG_SESSION_TYPE=x11 GTK_A11Y=none GSETTINGS_BACKEND=memory
  export MONOLITO_STYLES_DIR="$(pwd)/tests/fixtures/styles.d"
  dbus-update-activation-environment XDG_CONFIG_HOME XDG_CACHE_HOME XDG_DATA_HOME
  xfwm4 --compositor=off >/tmp/monolito-xfwm-test.log 2>&1 &
  wm=$!
  trap "kill $wm 2>/dev/null || true" EXIT
  sleep 1
  if [ -n "${MONOLITO_TEST_BINARY:-}" ]; then
    "$MONOLITO_TEST_BINARY" ui_workflow --ignored --nocapture
  else
    cargo test ui_workflow -- --ignored --nocapture
  fi
'
