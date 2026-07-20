# Contributing to VPN Hizb

Thank you for your interest in contributing to VPN Hizb!

## How to Contribute

### Reporting Bugs

1. Check [existing issues](https://github.com/ajangsupardi/vpn-hizb/issues) to avoid duplicates
2. Open a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - OS and version info

### Suggesting Features

1. Open an issue with the `enhancement` label
2. Describe the feature and its use case

### Submitting Code

1. Fork the repository
2. Create a branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Test thoroughly
5. Commit with clear messages
6. Push and open a Pull Request

## Development Setup

```bash
git clone https://github.com/ajangsupardi/vpn-hizb.git
cd vpn-hizb
pnpm install
cargo tauri dev
```

## Code Style

### Rust

- Follow standard `rustfmt` formatting
- Use descriptive variable names
- Handle errors with `Result` types

### Svelte

- Use Svelte 5 runes (`$state`, `$derived`, `$effect`)
- Follow existing component patterns
- Keep components small and focused

## Commits

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new server filter
fix: resolve connection timeout
docs: update README
```

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
