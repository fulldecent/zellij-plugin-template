# Changelog

## [0.3.0](https://github.com/fulldecent/zellij-plugin-template/compare/0.2.0...v0.3.0) (2026-10-08)


### Features

* add release process ([78107e2](https://github.com/fulldecent/zellij-plugin-template/commit/78107e24183227c19067949abea64bf34e9932b6))


### Bug Fixes

* build snapshot wasm with rust-toolchain.toml ([e70bf6d](https://github.com/fulldecent/zellij-plugin-template/commit/e70bf6d2a7477a3828c6c487a0557b1eb3fc0a06))

## Changelog

## Unreleased

- Release wasm uses a stable reverse-DNS name (`com.github.owner.repo.wasm`) and a matching `.sigstore.jsonl` sidecar with build provenance and version attestations.
- Release Please opens draft pull requests; merging publishes an immutable GitHub Release.
- Contributor docs install rustup and run toolchain binaries with `"$(rustup which cargo)"` so `rust-toolchain.toml` applies even when another `cargo` is on `PATH`.
