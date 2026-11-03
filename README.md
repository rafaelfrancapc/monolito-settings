# Settings

A native GTK4 settings application for Cinnamon and Linux Mint. It keeps the
familiar category layout while bringing common desktop controls into one place
and opening the appropriate Mint or Cinnamon tool for specialised tasks.

The visible application name is **Settings**. `monolito-settings` and
`org.monolito.Settings` remain technical identifiers for compatibility.

## Highlights

- Eleven searchable settings categories.
- Direct Cinnamon controls for notifications, mouse and touchpad, accessibility,
  privacy, animations and desktop effects.
- Full-card shortcuts to Cinnamon and Mint tools; unavailable components are
  clearly marked instead of failing silently.
- System appearance follows the Mint Themes panel automatically, including
  colour, light/dark mode, GTK theme, icons, cursor and font.
- Local user accounts only; Active Directory/NSS accounts are not listed.
- Native hostname management and an authorised shortcut to edit `sssd.conf`.
- Language and language-pack management through Mint Locale or Language Support.

## Install on Debian, Ubuntu or Linux Mint

Download the `.deb` from the [latest release](https://github.com/rafaelfrancapc/monolito-settings/releases/latest), then run:

```sh
sudo apt install ./monolito-settings_*.deb
```

While installed, the package hides the main Cinnamon Settings launcher with a
reversible `dpkg-divert`. Removing this package restores the original launcher.
The Cinnamon modules themselves remain available from Settings.

## Build from source

```sh
sudo apt install cargo rustc rustfmt build-essential pkg-config libgtk-4-dev librsvg2-common
cargo build --release --locked
sudo ./scripts/install.sh
```

To create a Debian package:

```sh
./scripts/build-deb.sh
```

Run the application as the regular desktop user, not as root.

## Cinnamon and Mint integration

Appearance is managed by the standard Mint Themes panel. The application
observes the system settings and updates itself automatically; it does not keep
a competing accent-colour or theme selector.

For user names, passwords, groups, printers, drivers, update management and
other distribution-specific features, Settings opens the corresponding native
utility. This preserves the distribution's authentication and policy handling.

More detail is available in [Cinnamon integration](docs/CINNAMON_INTEGRATION.md).

## Development

```sh
cargo fmt --check
cargo check --locked
cargo test --locked
python3 -m unittest discover -s tests -v
./scripts/gui-test.sh
```

See [validation notes](docs/VALIDATION.md) for the tested environments and
integration limits.
