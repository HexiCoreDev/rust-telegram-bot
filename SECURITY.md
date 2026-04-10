# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 1.0.0-beta.5 | Yes |

## Reporting a Vulnerability

If you discover a security vulnerability in rust-tg-bot, please report it responsibly.

**Do NOT open a public GitHub issue for security vulnerabilities.**

Instead, email: **[judechinedu122@gmail.com](mailto:judechinedu122@gmail.com)**

Include:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

## Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 1 week
- **Fix or mitigation**: Depends on severity, typically within 2 weeks for critical issues

## Scope

This policy covers:

- The `rust-tg-bot-raw`, `rust-tg-bot-ext`, `rust-tg-bot-macros`, and `rust-tg-bot` crates
- Security issues in request handling, token management, webhook validation
- Dependency vulnerabilities that affect this project

## Best Practices for Bot Developers

- Never commit your bot token to version control
- Use environment variables (`TELEGRAM_BOT_TOKEN`) for token storage
- Enable webhook `secret_token` validation in production
- Use HTTPS for webhook endpoints
- Restrict bot permissions to the minimum needed
- Validate user input before processing
