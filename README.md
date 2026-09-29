# Zellij plugin template

[![Lint](https://github.com/fulldecent/zellij-plugin-template/actions/workflows/lint.yml/badge.svg?branch=main)](https://github.com/fulldecent/zellij-plugin-template/actions/workflows/lint.yml)
[![CI](https://github.com/fulldecent/zellij-plugin-template/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/fulldecent/zellij-plugin-template/actions/workflows/ci.yml)

> [!TIP]
>
> To use this template with your own project:
>
> - Replace the above heading with your project name.
> - Update the status badge and all other mentions of "zellij-plugin-template" (except the references section) to instead point to your own repo.
> - Update the description in Cargo.toml.
> - Address and remove this and all other "TIP" items below.

## What this plugin does

> [!TIP]
>
> Include screenshots of your actual product here.

This is an opinionated template for a Zellij plugin that provides:

- Install/upgrade instructions which allow your customers to avoid compiling and always remember which version they installed
- Development instructions which allow your contributors to setup with zero programming experience
- [Continuous integration testing](.github/workflows/ci.yml) with GitHub Actions.
- A documented and [automated](.github/workflows/release.yml) version release process
- Modern development best practices: [.gitignore](.gitignore), [enforced formatting](.github/workflows/lint.yml)
- A minimal implementation to extend, which is suitable for every plugin type (status bar, tab bar, pane, floating pane)

The included plugin renders:

```text
////////////////////////
////////////////////////
////////////////////////
////////////////////////
```

It does not ask for permissions to run. It's just slashes.

## Installation

Install the [latest released version](https://github.com/fulldecent/zellij-plugin-template/releases) by downloading and copying the .wasm file into `~/.config/zellij/plugins/`:

```sh
# Run this from your download folder to match your downloaded version number.
mkdir -p ~/.config/zellij/plugins
cp fulldecent-zellij-plugin-template-v*.*.*.wasm ~/.config/zellij/plugins/
```

## Usage

1. Try it inside Zellij:

   ```sh
   # Add like "9.9.9.wasm" after the "v" here based on your downloaded version number.
   zellij action start-or-reload-plugin file:~/.config/zellij/plugins/fulldecent-zellij-plugin-template-v
   ```

2. Create a personal default layout if you don't have one:

   ```sh
   mkdir -p ~/.config/zellij/layouts
   DEST=~/.config/zellij/layouts/default.kdl
   [ -f "$DEST" ] || zellij setup --dump-layout default > $DEST
   ```

3. Edit that file and replace the default tab-bar with this plugin:

   Add like "9.9.9.wasm" after the "v" here based on your downloaded version number.

   ```diff
   - plugin location="tab-bar"
   + plugin location="file:~/.config/zellij/plugins/fulldecent-zellij-plugin-template-v"
   ```

> [!TIP]
>
> Use different instructions in step 3 above if you want to replace something other than the tab-bar, or want to use a keybinding.

## Upgrading

Repeat the same installation + usage above. Your new version will supercede the old version.

## Development

Clone the repo:

```sh
git clone https://github.com/fulldecent/zellij-plugin-template.git ~/Developer/fulldecent-zellij-plugin-template
cd ~/Developer/fulldecent-zellij-plugin-template
```

Setup Rust without an extra unnecessary `curl | sh` by using your package manager (on macOS: `brew install rustup`). This requires a few extra `PATH` workarounds below.

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

## References

1. We use an MIT license for this template. You should carefully consider which license to apply to your own project.
1. Zellij offers another installation method that is simpler and insecure. That points your layout configuration to a HTTPS URL. We consider that feature wrong and deprecated. [TODO: create and link upstream issue]
1. We speak of the official rustup installation recommendation as ugly. [Reported upstream](https://github.com/rust-lang/rust/issues/163468).
1. This project is built based on [best practices documented in zellij-plugin-template](https://github.com/fulldecent/zellij-plugin-template), release 1.0.0.
1. This project is built based on [best practices documented in project-template](https://github.com/fulldecent/project-template), release 1.0.0.
