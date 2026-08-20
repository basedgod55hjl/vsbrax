# Repository Guidelines

## Development Workflow

- **Stay on your own branch** - Do not take, cherry-pick, merge, or copy code from other
  people's or other agents' branches. Only work from your branch and its base (e.g. `main`).
  If you need something that lives on another branch, tell the user and let them decide;
  never pull it in yourself.

## Install Notes
- `~/.local/bin/VS-BRAX Agent` is the launcher symlink used from `PATH`.
- `~/.VS-BRAX Agent/builds/current/VS-BRAX Agent` is the active local/source-build channel; self-dev builds and `scripts/install_release.sh` point the launcher here.
- `~/.VS-BRAX Agent/builds/stable/VS-BRAX Agent` is the stable release channel; `scripts/install.sh` installs this and points the launcher here.
- `~/.VS-BRAX Agent/builds/versions/<version>/VS-BRAX Agent` stores immutable binaries.
- `~/.VS-BRAX Agent/builds/canary/VS-BRAX Agent` still exists for canary/testing flows, but it is not the primary self-dev install path.
- On Windows, the equivalents are `%LOCALAPPDATA%\\VS-BRAX Agent\\bin\\VS-BRAX Agent.exe` for the launcher, `%LOCALAPPDATA%\\VS-BRAX Agent\\builds\\stable\\VS-BRAX Agent.exe` for stable, and `%LOCALAPPDATA%\\VS-BRAX Agent\\builds\\versions\\<version>\\VS-BRAX Agent.exe` for immutable installs; `scripts/install.ps1` currently installs the stable channel.
- Ensure `~/.local/bin` is **before** `~/.cargo/bin` in `PATH`.
