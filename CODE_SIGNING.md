# Code signing policy

Free code signing provided by [SignPath.io](https://about.signpath.io), certificate by [SignPath Foundation](https://signpath.org).

## What is signed

| Platform | Artifacts | Signed |
|---|---|---|
| Windows | `ASCII Local_x.x.x_x64-setup.exe` (NSIS installer), `ASCII Local_x.x.x_x64_en-US.msi` | Yes, via SignPath |
| macOS | `.dmg` | No |
| Android | `ASCII-Local.apk` | No |

Only binaries built from this repository's source code are signed. No third-party or upstream binaries are signed under this policy.

## Build and signing process

1. Releases are built from the `main` branch by the public GitHub Actions workflow [`.github/workflows/release.yml`](.github/workflows/release.yml).
2. The Windows installers produced by the workflow are submitted to SignPath for signing.
3. Each signing request must be manually approved by an approver (see below) before the signed files are published.
4. Signed installers are published on [GitHub Releases](https://github.com/BorisM-afk/ascii-local/releases).

Binaries are never built or signed on a personal machine.

## Team roles

ASCII Local is currently maintained by a single person.

| Role | Members |
|---|---|
| Committers and reviewers | [@BorisM-afk](https://github.com/BorisM-afk) |
| Approvers | [@BorisM-afk](https://github.com/BorisM-afk) |

- **Committers** may push to the repository. Contributions from other people are only accepted through pull requests, which are reviewed by a committer before being merged.
- **Approvers** check that each signing request comes from the official workflow on `main` and corresponds to an intended release before approving it.
- All team members use multi-factor authentication on GitHub and SignPath.

## Privacy policy

This program will not transfer any information to other networked systems unless specifically requested by the user or the person installing or operating it.

In practice, ASCII Local works entirely offline: a Content-Security-Policy blocks all network connections from the application, photos and videos are processed only on the user's device, and exported images are stripped of metadata such as GPS location.

## Reporting

To report a problem with a signed binary (for example a file that does not match this policy), open an issue on [GitHub](https://github.com/BorisM-afk/ascii-local/issues) or write to boris.fr.67@gmail.com.
