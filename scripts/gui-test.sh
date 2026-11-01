#!/bin/sh
# Requires Xvfb, xauth, dbus-run-session, xfwm4. Run as a regular user.
set -eu
cd "$(dirname "$0")/.."
exec dbus-run-session -- xvfb-run -a -s '-screen 0 1400x900x24' sh -eu -c '
  xfwm4 --compositor=off >/tmp/monolito-xfwm-test.log 2>&1 &
  wm=$!
  trap "kill $wm 2>/dev/null || true" EXIT
  export XDG_CURRENT_DESKTOP=XFCE XDG_SESSION_TYPE=x11 GTK_A11Y=none
  export XDG_CONFIG_HOME=$(mktemp -d)
  sleep 1
  if [ -n "${MONOLITO_TEST_BINARY:-}" ]; then
    "$MONOLITO_TEST_BINARY" ui_workflow --ignored --nocapture
  else
    cargo test ui_workflow -- --ignored --nocapture
  fi
'
