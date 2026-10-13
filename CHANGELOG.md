# Changelog

## Unreleased

- Install downloads the latest wasm and `.sigstore.jsonl` sidecar from GitHub `/releases/latest/download` using the stable reverse-DNS names.
- Install paths use `file:~`. Zellij 0.45.1 expands `~` in `file:` plugin locations.
- Release Please uses the `rust` release type, so the release pull request bumps `Cargo.toml`. The tag stays `vMAJOR.MINOR.PATCH`.
- Snapshot tests run every `shots/*.yaml`. `screenshot.svg` stays at the repository root. Other pictures stay beside their YAML scripts.

## [0.4.0](https://github.com/fulldecent/zellij-plugin-template/compare/v0.3.0...v0.4.0) (2026-10-08)


### Features

* simpler install ([3b9a272](https://github.com/fulldecent/zellij-plugin-template/commit/3b9a27245bc0bebbbffe8d55dda914256fce22ef))

## [0.3.0](https://github.com/fulldecent/zellij-plugin-template/compare/0.2.0...v0.3.0) (2026-10-08)


### Features

* add release process ([78107e2](https://github.com/fulldecent/zellij-plugin-template/commit/78107e24183227c19067949abea64bf34e9932b6))


### Bug Fixes

* build snapshot wasm with rust-toolchain.toml ([e70bf6d](https://github.com/fulldecent/zellij-plugin-template/commit/e70bf6d2a7477a3828c6c487a0557b1eb3fc0a06))
