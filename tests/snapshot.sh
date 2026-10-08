#!/bin/sh
# Build the plugin wasm and require render() bytes to match shots/screenshot.ansi.txt.
#
# zellij-plugin-snapshot is a command. [dependencies] and [dev-dependencies]
# link a library into the wasm or into `cargo test`, so they leave this binary
# uninstalled. Install the pinned release first:
#   "$(rustup which cargo)" install zellij-plugin-snapshot --version 0.2.2 --locked
# https://github.com/fulldecent/zellij-plugin-snapshot
set -eu

root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
cd "$root"

# Homebrew rust ignores rust-toolchain.toml. Use the rustup-selected cargo.
if command -v rustup >/dev/null 2>&1; then
  cargo=$("$(command -v rustup)" which cargo)
else
  cargo=cargo
fi

# `cargo install` places zellij-plugin-snapshot in ~/.cargo/bin.
PATH="${HOME}/.cargo/bin:${PATH}"
export PATH

if ! command -v zellij-plugin-snapshot >/dev/null 2>&1; then
  echo "zellij-plugin-snapshot 0.2.2 is not on PATH." >&2
  echo '"$(rustup which cargo)" install zellij-plugin-snapshot --version 0.2.2 --locked' >&2
  exit 1
fi

# .cargo/config.toml already selects wasm32-wasip1 for a plain `cargo build`.
# Name the target here so this script still builds the plugin wasm if that
# default is removed. `cargo test` cannot do this job: it locks the target
# directory, and it runs the host harness rather than the plugin wasm.
"$cargo" build --release --locked --target wasm32-wasip1

out=$(mktemp -d)
trap 'rm -rf "$out"' EXIT

# shots/screenshot.yaml sets `name: screenshot`, so the host writes
# screenshot.ansi.txt and screenshot.svg.
zellij-plugin-snapshot shots/screenshot.yaml --out "$out"

if ! diff -u shots/screenshot.ansi.txt "$out/screenshot.ansi.txt"; then
  echo "Render bytes differ from shots/screenshot.ansi.txt." >&2
  echo "When that change is intended, refresh the committed files:" >&2
  echo "  zellij-plugin-snapshot shots/screenshot.yaml --out /tmp/shots" >&2
  echo "  cp /tmp/shots/screenshot.ansi.txt shots/screenshot.ansi.txt" >&2
  echo "  cp /tmp/shots/screenshot.svg screenshot.svg" >&2
  exit 1
fi
