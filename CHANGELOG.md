# Changelog

## Unreleased

- Install downloads the latest wasm and `.sigstore.jsonl` sidecar from GitHub `/releases/latest/download` using the stable reverse-DNS names.
- Install paths use `$HOME`. Zellij does not expand `~` in `file:` locations.
- Snapshot tests run every `shots/*.yaml`. Gitignore covers stray `*.wasm`.

## [0.5.0](https://github.com/fulldecent/zellij-plugin-template/compare/v0.4.0...v0.5.0) (2026-10-10)


### Features

* collect plugin practices from status-bar-ng ([1415454](https://github.com/fulldecent/zellij-plugin-template/commit/141545426d5d2a72d0bbf9f2d7b13553a7b615a3))


### Bug Fixes

* compare snapshot output names from the YAML name field ([2aa4027](https://github.com/fulldecent/zellij-plugin-template/commit/2aa4027dac6763db10716fbea604db053fd50404))
* copy each snapshot svg by name ([37e484e](https://github.com/fulldecent/zellij-plugin-template/commit/37e484edab1189b7d57734fcf6e43407989e0dd8))
* fail empty snapshot glob and install via rustup run ([8c2d390](https://github.com/fulldecent/zellij-plugin-template/commit/8c2d39082c2b4bbd34a0e9c8423629a8e74e9d5c))
* invoke cargo only via rustup which ([58bd5d7](https://github.com/fulldecent/zellij-plugin-template/commit/58bd5d7a8eb987d0a86b8aa85b999172a1640890))
* invoke cargo with rustup which, matching rust-template ([3a99e15](https://github.com/fulldecent/zellij-plugin-template/commit/3a99e1534f6608efc822bde71800f8aebfbb2968))
* rename LICENSE to LICENSE.md ([4f1557a](https://github.com/fulldecent/zellij-plugin-template/commit/4f1557a2aec389795e9c25374cccad10cbe13de4))
* trap snapshot temp dirs and keep legacy layout sed ([55bd200](https://github.com/fulldecent/zellij-plugin-template/commit/55bd200851e92d6b1a4c0803ff590b4d566c2bf4))

## [0.4.0](https://github.com/fulldecent/zellij-plugin-template/compare/v0.3.0...v0.4.0) (2026-10-08)


### Features

* simpler install ([3b9a272](https://github.com/fulldecent/zellij-plugin-template/commit/3b9a27245bc0bebbbffe8d55dda914256fce22ef))

## [0.3.0](https://github.com/fulldecent/zellij-plugin-template/compare/0.2.0...v0.3.0) (2026-10-08)


### Features

* add release process ([78107e2](https://github.com/fulldecent/zellij-plugin-template/commit/78107e24183227c19067949abea64bf34e9932b6))


### Bug Fixes

* build snapshot wasm with rust-toolchain.toml ([e70bf6d](https://github.com/fulldecent/zellij-plugin-template/commit/e70bf6d2a7477a3828c6c487a0557b1eb3fc0a06))
