# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **Do NOT** open a public issue
2. Email **ajang@duck.com** with:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

## Response

- Acknowledgment within 48 hours
- Fix or mitigation plan within 7 days
- Credit in release notes (unless anonymity is requested)

## Security Considerations

- VPN Hizb requires `sudo` for OpenVPN operations
- Credentials are stored in memory only, never on disk
- IPv6 is disabled during VPN sessions to prevent leaks
- No telemetry or data collection
