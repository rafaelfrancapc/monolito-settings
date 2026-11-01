#!/bin/sh
# Optional Linux development container. Docker must already be configured.
# In managed environments, preserve platform proxy defaults and mount the CA bundle.
set -eu
cd "$(dirname "$0")/.."
case "${1:-}" in
  create)
    docker run -d --name monolito-dev \
      --mount "type=bind,src=$(pwd),dst=/app" \
      --mount type=bind,src=/etc/ssl/certs/ca-certificates.crt,dst=/run/ca-bundle.pem,readonly \
      -e CARGO_HTTP_CAINFO=/run/ca-bundle.pem -w /app debian:trixie-slim sleep infinity
    docker exec monolito-dev sh -ec 'apt-get -o Acquire::https::CaInfo=/run/ca-bundle.pem update; apt-get -o Acquire::https::CaInfo=/run/ca-bundle.pem install -y --no-install-recommends cargo rustc rustfmt libgtk-4-dev librsvg2-common build-essential xvfb xauth xdotool dbus-x11 xfwm4 xfconf fonts-inter python3'
    docker exec monolito-dev useradd -m -u 1000 monolito
    ;;
  build) docker exec monolito-dev sh -ec 'cargo fmt --check; cargo check --locked; cargo build --locked; cargo test --locked; python3 -m unittest discover -s tests -v' ;;
  gui)
    test_binary=$(docker exec monolito-dev sh -c 'find target/debug/deps -maxdepth 1 -name "monolito_settings-*" -type f -executable | head -n 1')
    test -n "$test_binary"
    docker exec -u 1000:1000 -e HOME=/home/monolito -e "MONOLITO_TEST_BINARY=/app/$test_binary" monolito-dev ./scripts/gui-test.sh
    ;;
  *) echo 'Usage: scripts/dev-container.sh create|build|gui'; exit 2 ;;
esac
