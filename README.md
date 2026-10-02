# Zellij plugin template

[![Lint](https://github.com/fulldecent/zellij-plugin-template/actions/workflows/lint.yml/badge.svg?branch=main)](https://github.com/fulldecent/zellij-plugin-template/actions/workflows/lint.yml)
[![CI](https://github.com/fulldecent/zellij-plugin-template/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/fulldecent/zellij-plugin-template/actions/workflows/ci.yml)

> [!WARNING]
>
> To use this template with your own project:
>
> - Replace the above heading with your project name.
> - Update the status badge and all other mentions of "zellij-plugin-template" (except the references section) to instead point to your own repo.
> - Update the description in Cargo.toml.
> - Address and remove this and all other "TIP" items below.

## What this plugin does

> [!WARNING]
>
> Include selling points and screenshots of your actual product here.

This is an opinionated template for a Zellij plugin that provides:

- Install/upgrade instructions which allow your customers to avoid compiling and always remember which version they installed
- Development instructions which allow your contributors to setup with zero programming experience
- [Continuous integration testing](.github/workflows/ci.yml) with GitHub Actions.
- A render snapshot checked by [tests/snapshot.sh](tests/snapshot.sh), using [Zellij Plugin Snapshot](https://github.com/fulldecent/zellij-plugin-snapshot)
- A documented and [automated](.github/workflows/release.yml) version release process
- Modern development best practices: [.gitignore](.gitignore), [enforced formatting](.github/workflows/lint.yml)
- A minimal implementation to extend, which is suitable for every plugin type (status bar, tab bar, pane, floating pane)

This plugin renders:

![Plugin pane filled with slash characters](screenshot.svg)

It does not require any permissions to run. It's just slashes.

## Try it out

1. Select the latest wasm release and install to your plugins folder:

   ```sh
   ver=$(curl -fsSL https://api.github.com/repos/fulldecent/zellij-plugin-template/releases/latest \
     | python3 -c 'import json,sys; print(json.load(sys.stdin)["tag_name"])')
   name="fulldecent-zellij-plugin-template-${ver}.wasm"
   mkdir -p ~/.config/zellij/plugins
   curl -fL -o ~/.config/zellij/plugins/"$name" \
     "https://github.com/fulldecent/zellij-plugin-template/releases/download/${ver}/${name}"
   ```

2. Now open Zellij and try the plugin from inside it (without adding it to your layout yet):

   ```sh
   zellij
   
   # Run this next command from INSIDE your Zellij session
   plugin=$(printf '%s\n' ~/.config/zellij/plugins/fulldecent-zellij-plugin-template-*.wasm | sort -V | tail -n 1)
   zellij action start-or-reload-plugin "file:${plugin}"
   ```

## Installation

> [!WARNING]
>
> Modify instruction if your plugin is something other than a tab bar replacement.

You will be adding this plugin to your default layout. 

1. Complete the try it out instructions above to download the plugin.

2. First, create a default layout if you don't already have one:

   ```sh
   mkdir -p ~/.config/zellij/layouts
   DEST=~/.config/zellij/layouts/default.kdl
   [ -f "$DEST" ] || zellij setup --dump-layout default > $DEST
   ```

3. Then, replace `tab-bar` with this plugin:

   ```diff
   - plugin location="tab-bar"
   + plugin location="file:~/.config/zellij/plugins/fulldecent-zellij-plugin-template-...
   ```

   Or, non-interactively:

   ```sh
   plugin=$(printf '%s\n' ~/.config/zellij/plugins/fulldecent-zellij-plugin-template-*.wasm | sort -V | tail -n 1)
   DEST=~/.config/zellij/layouts/default.kdl
   # macOS sed writes the file in place. Replaces tab-bar or an older copy of this plugin.
   sed -i '' \
     -e "s|plugin location=\"tab-bar\"|plugin location=\"file:${plugin}\"|" \
     -e "s|plugin location=\"file:.*fulldecent-zellij-plugin-template-.*\\.wasm\"|plugin location=\"file:${plugin}\"|" \
     "$DEST"
   ```

## Upgrading

Repeat the same try it out + installation instructions above.

## Development

1. Clone the repo:

   ```sh
   git clone https://github.com/fulldecent/zellij-plugin-template.git ~/Developer/fulldecent-zellij-plugin-template
   cd ~/Developer/fulldecent-zellij-plugin-template
   ```

2. Setup Rust without an extra unnecessary `curl | sh` by using your package manager (on macOS: `brew install rustup`). This requires a few extra `PATH` workarounds below.

Set up dependencies:

```sh
PATH="$(dirname "$(realpath "$(which rustup)")"):$PATH" cargo check
```

Build:

```sh
PATH="$(dirname "$(realpath "$(which rustup)")"):$PATH" cargo build # debug
```

Run tests:

```sh
PATH="$(dirname "$(realpath "$(which rustup)")"):$PATH" cargo test --target "$(rustc -vV | sed -n 's/^host: //p')"
```

`cargo test` runs the Rust tests in `src/main.rs`. [tests/snapshot.sh](tests/snapshot.sh) builds the release wasm, runs [shots/screenshot.yaml](shots/screenshot.yaml), and exits non-zero unless those bytes match [shots/screenshot.ansi.txt](shots/screenshot.ansi.txt).

The host is [Zellij Plugin Snapshot](https://github.com/fulldecent/zellij-plugin-snapshot) 0.2.2. Install that published crate once. `cargo install` compiles it and puts `zellij-plugin-snapshot` on `PATH`. `[dependencies]` and `[dev-dependencies]` link a library into the plugin wasm or into `cargo test`. This host is a command, so those fields leave it uninstalled.

```sh
PATH="$(dirname "$(realpath "$(which rustup)")"):$PATH" cargo install zellij-plugin-snapshot --version 0.2.2 --locked
sh tests/snapshot.sh
```

[screenshot.svg](screenshot.svg) is that same pane, shown above. The test compares the ANSI file. When the picture should change, write both files again:

```sh
PATH="$(dirname "$(realpath "$(which rustup)")"):$PATH" zellij-plugin-snapshot shots/screenshot.yaml --out /tmp/shots
cp /tmp/shots/screenshot.ansi.txt shots/screenshot.ansi.txt
cp /tmp/shots/screenshot.svg screenshot.svg
```

`tests/snapshot.sh` passes `--target wasm32-wasip1`. [.cargo/config.toml](.cargo/config.toml) selects that target for a plain `cargo build`. The flag keeps the snapshot build on the plugin wasm if that default is removed.

Run this directly with:

>[!TIP]
>Keep all just one of these, depending on what kind of plugin you have. Delete those unused files in the layout/ folder.
>
>And the `start-or-reload-plugin` example only makes sense if you are using a pane type.

```sh
zellij --layout layout/plugin-dev.pane.kdl
zellij --layout layout/plugin-dev.floating-pane.kdl
zellij --layout layout/plugin-dev.tab-bar.kdl
zellij --layout layout/plugin-dev.status-bar.kdl
```

Or in an existing session, access a shell and use:

```sh
zellij action start-or-reload-plugin file:target/wasm32-wasip1/debug/plugin.wasm
```

## Releasing

Plugin versions use [Semantic Versioning](https://semver.org/).

Update the version in `Cargo.toml` and then create a GitHub release with the same version to kick of an automated deployment. This will automatically attach the built binary to the release.

## Maintenance and dependency updates

Do this every month or so and please send a PR here if you see updates available:

1. Identify external Actions in [.github/workflows](./.github/workflows) scripts and look for available new versions. Review and then update to the new version if it is safe. GitHub-supported Actions (i.e. under the actions/ organization) may require only cursory review.
1. Review the Rust toolchain in `rust-toolchain.toml`. Update it when a newer stable version is appropriate.
1. Review [zellij-plugin-snapshot](https://github.com/fulldecent/zellij-plugin-snapshot) releases. Update every `cargo install zellij-plugin-snapshot --version` line to that same published version. Regenerate [shots/screenshot.ansi.txt](shots/screenshot.ansi.txt) and [screenshot.svg](screenshot.svg) when the host output changes.

## References

1. We use an MIT license for this template. You should carefully consider which license to apply to your own project.
1. Zellij offers another installation method that is simpler and insecure. That points your layout configuration to a HTTPS URL. We consider that feature wrong and deprecated. [TODO: create and link upstream issue]
1. We speak of the official rustup installation recommendation as ugly. [Reported upstream](https://github.com/rust-lang/rust/issues/163468).
1. Render snapshots follow [Zellij Plugin Snapshot](https://github.com/fulldecent/zellij-plugin-snapshot) 0.2.2. Installation follows [`cargo install`](https://doc.rust-lang.org/cargo/commands/cargo-install.html). The test compares [shots/screenshot.ansi.txt](shots/screenshot.ansi.txt). [screenshot.svg](screenshot.svg) is the picture in this README.
1. This project is built based on [best practices documented in zellij-plugin-template](https://github.com/fulldecent/zellij-plugin-template), release 1.0.0.
1. This project is built based on [best practices documented in project-template](https://github.com/fulldecent/project-template), release 1.0.0.
