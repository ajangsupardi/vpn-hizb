<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="VPN Hizb Logo" width="128" height="128" />
</p>

<h1 align="center">VPN Hizb</h1>

<p align="center">
  <strong>Public VPN for Everyone</strong><br/>
  A free and open-source VPN client that provides secure internet access through a global network of public VPN servers.
</p>

<p align="center">
  <a href="https://github.com/ajangsupardi/vpn-hizb/releases/latest"><img alt="Latest Release" src="https://img.shields.io/github/v/release/ajangsupardi/vpn-hizb?style=flat-square"></a>
  <a href="https://github.com/ajangsupardi/vpn-hizb/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/github/license/ajangsupardi/vpn-hizb?style=flat-square"></a>
  <a href="https://github.com/ajangsupardi/vpn-hizb/issues"><img alt="Issues" src="https://img.shields.io/github/issues/ajangsupardi/vpn-hizb?style=flat-square"></a>
</p>

---

## Features

- **VPN Gate Integration** — Fetches public VPN servers from VPN Gate API
- **IP Leak Detection** — Monitors for real IP exposure
- **IPv6 Management** — Disables IPv6 during VPN session to prevent leaks
- **Bilingual Support** — English and Indonesian
- **One-Click Connect** — Simple and intuitive interface

## Requirements

- Linux (amd64)
- `openvpn`
- `curl`
- `pkexec` (for privilege escalation)

## Installation

### Download

Download the latest `.deb` package from [Releases](https://github.com/ajangsupardi/vpn-hizb/releases/latest).

### Install

```bash
sudo dpkg -i vpn-hizb_1.0.0_amd64.deb
sudo apt-get install -f
```

Or use `gdebi`:

```bash
sudo gdebi vpn-hizb_1.0.0_amd64.deb
```

### First Run

1. Launch VPN Hizb from your application menu
2. Click **Settings** in the top bar to set up system permissions
3. Enter your password when prompted (one time only)
4. Refresh the server list and connect

## Uninstall

```bash
sudo dpkg --purge vpn-hizb
rm -rf ~/.local/share/com.hizb.vpn-hizb
```

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v18+)
- [pnpm](https://pnpm.io/)
- Tauri CLI

```bash
cargo install tauri-cli
```

### Build

```bash
git clone https://github.com/ajangsupardi/vpn-hizb.git
cd vpn-hizb
pnpm install
cargo tauri build
```

The `.deb` package will be at `src-tauri/target/release/bundle/deb/`.

## Architecture

```
vpn-hizb/
├── src/                    # Svelte 5 frontend
│   ├── pages/              # Dashboard
│   ├── components/         # UI components
│   └── lib/                # i18n, commands
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri IPC commands
│   │   ├── services/       # OpenVPN, VPN Gate, IP check
│   │   ├── models/         # Data models
│   │   ├── db/             # SQLite database
│   │   └── i18n/           # Translations (EN/ID)
│   └── tauri.conf.json     # Tauri configuration
└── static/                 # Static assets
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the [MIT License](LICENSE).

## Disclaimer

This software is provided for educational and research purposes. Users are responsible for complying with local laws and regulations regarding VPN usage.

## Author

**Ajang Supardi** — [ajang@duck.com](mailto:ajang@duck.com)
