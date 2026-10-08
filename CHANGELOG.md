# Changelog

## Unreleased

- Release wasm uses a stable reverse-DNS name (`com.github.owner.repo.wasm`) and a matching `.sigstore.jsonl` sidecar with build provenance and version attestations.
- Release Please opens draft pull requests; merging publishes an immutable GitHub Release.
- Contributor docs install rustup and run toolchain binaries with `"$(rustup which cargo)"` so `rust-toolchain.toml` applies even when another `cargo` is on `PATH`.
