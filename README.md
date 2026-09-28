# Zellij plugin template

[![Lint](https://github.com/fulldecent/zellij-plugin-template/actions/workflows/lint.yml/badge.svg?branch=main)](https://github.com/fulldecent/zellij-plugin-template/actions/workflows/lint.yml)
[![CI](https://github.com/fulldecent/zellij-plugin-template/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/fulldecent/zellij-plugin-template/actions/workflows/ci.yml)

> [!TIP]
>
> To use this template with your own project:
>
> - Replace the above heading with your project name.
> - Update the status badge and all other mentions of this repo (except the references section) to instead point to your own repo.

> [!WARNING]
> Need a full read through here!
>
> - Do NOT use any github action other than github-supported / see dtolnay/rust-toolchain@stable

## What this plugin does

This is an opinionated template for a Zellij plugin that provides:

- Install/upgrade instructions which allow your customers to avoid compiling and always remember which version they installed
- Development instructions which allow your contributors to setup with zero programming experience
- A documented version release process
- GitHub Actions workflows to [test and build the plugin](.github/workflows/ci.yml) and to [attach the built artifact to releases](.github/workflows/release.yml)
- Modern development best practices: [.gitignore](.gitignore), [enforced formatting](.github/workflows/lint.yml)
- A minimal implementation to extend, which is suitable for every plugin type (status bar, tab bar, pane, floating pane)

The included plugin renders:

```text
////////////////////////
////////////////////////
////////////////////////
////////////////////////
```

It does not handle the keyboard, read configuration, or start workers. The first time it loads, Zellij asks for permission to read application state. That permission supplies the active theme colors.

## Installation

1. Install the [latest released version](https://github.com/fulldecent/zellij-plugin-template/releases) by downloading and copying the WASM file into `~/.config/zellij/plugins/`:

   ```sh
   mkdir -p ~/.config/zellij/plugins
   cp fulldecent-zellij-plugin-template-v1.0.0.wasm \
      ~/.config/zellij/plugins/
   ```

2. (Optional) test it using the supplied demo-layout before you cutover:

   ```sh
   ...
   ```

3. Add this plugin to your default layout:

   ```sh
   ...
   ... create a layout based on default
   ...
   ensure this does NOT overwrite any existing customized layout
   ...
   and then splice in our template
   ...
   include github note here about tweaking this based on WHICH kind of plugin we have
   ```

   







```text
~/.config/zellij/plugins/
└── fulldecent-zellij-plugin-template-v0.1.0.wasm
```

The filename includes the plugin identifier and the version. A file named only `plugin.wasm` does not say which version is installed, whether an update exists, or whether a bug report applies.

On GitHub, name the file `<github-user>-<repository>-v<version>.wasm`. Repository names are not globally unique. The GitHub user and repository are. This repository publishes `fulldecent-zellij-plugin-template-v0.1.0.wasm`. Another project would publish a name such as `fulldecent-zellij-awesome-v1.2.3.wasm`.

Download the `.wasm` file attached to the GitHub release, then install it:

```bash
mkdir -p ~/.config/zellij/plugins
cp fulldecent-zellij-plugin-template-v0.1.0.wasm \
   ~/.config/zellij/plugins/
```

Load it from a layout or a keybinding:

```kdl
plugin location="file:~/.config/zellij/plugins/fulldecent-zellij-plugin-template-v0.1.0.wasm"
```

## Usage

Point a pane at the installed file. These examples use `fulldecent-zellij-plugin-template-v0.1.0.wasm`.

> [!WARNING]
> for your project
>
> SELECT ONE OF THESE !
>
> 

Status bar:

```kdl
layout {
    default_tab_template {
        pane size=1 borderless=true {
            plugin location="zellij:tab-bar"
        }
        children
        pane size=1 borderless=true {
            plugin location="file:~/.config/zellij/plugins/fulldecent-zellij-plugin-template-v0.1.0.wasm"
        }
    }
    pane
}
```

Tab bar:

```kdl
layout {
    default_tab_template {
        pane size=1 borderless=true {
            plugin location="file:~/.config/zellij/plugins/fulldecent-zellij-plugin-template-v0.1.0.wasm"
        }
        children
        pane size=1 borderless=true {
            plugin location="zellij:status-bar"
        }
    }
    pane
}
```

Pane:

```kdl
layout {
    pane
    pane {
        plugin location="file:~/.config/zellij/plugins/fulldecent-zellij-plugin-template-v0.1.0.wasm"
    }
}
```

Floating pane:

```kdl
layout {
    pane
    floating_panes {
        pane {
            plugin location="file:~/.config/zellij/plugins/fulldecent-zellij-plugin-template-v0.1.0.wasm"
        }
    }
}
```

## Upgrading

Copy the new versioned file next to the old one:

```bash
cp fulldecent-zellij-plugin-template-v0.2.0.wasm \
   ~/.config/zellij/plugins/
```

The directory then shows which versions are present:

```text
fulldecent-zellij-plugin-template-v0.1.0.wasm
fulldecent-zellij-plugin-template-v0.2.0.wasm
```

Change the `plugin location` to the new filename. No extra tool is required. Remove the old file when you no longer want that version.

## Development

Develop in `~/Development/<repository-name>`.

> [!TIP]
> Replace `<repository-name>` with the repository name. For this repository, that path is `~/Development/zellij-plugin-template`. Remove this note from your repository.

> [!WARNING]
>
> NEED ACTUAL rust command lines STARTING from a virgin macOS + Xcode + homebrew

`rust-toolchain.toml` installs Rust, rustfmt, clippy, and the `wasm32-wasip1` target. `.cargo/config.toml` makes `cargo build` compile that target, so the debug artifact is:

```text
~/Development/<repository-name>/target/wasm32-wasip1/debug/plugin.wasm
```

The release artifact, before the release workflow renames it, is:

```text
~/Development/<repository-name>/target/wasm32-wasip1/release/plugin.wasm
```

The Cargo package name is `plugin`. That name is the `<plugin>` portion of the path above. If you rename the package, update the `file:` locations in `dev/` to match.

> [!NOTE]
> This template ships multiple development layouts.
> Select the layout appropriate for your plugin type.
> Delete the others after initial setup.
> Remove this note from your repository.

| Layout | Command |
| --- | --- |
| Status bar | `zellij -l dev/plugin-dev.status-bar.kdl` |
| Tab bar | `zellij -l dev/plugin-dev.tab-bar.kdl` |
| Pane | `zellij -l dev/plugin-dev.pane.kdl` |
| Floating pane | `zellij -l dev/plugin-dev.floating-pane.kdl` |

Zellij joins a relative `file:` location to the directory where the session was started. These layouts use `file:./target/wasm32-wasip1/debug/plugin.wasm`, which is the debug build when that directory is `~/Development/<repository-name>`. `file:../target/...` would point outside that directory, so these layouts do not use it. An absolute path is not required.

Build, test, and launch from `~/Development/<repository-name>`:

```bash
cargo build
cargo test --target "$(rustc -vV | sed -n 's/^host: //p')"
zellij -l dev/plugin-dev.status-bar.kdl
```

`cargo test` runs on the host target. Since Zellij 0.45, plugin host functions are no-ops there, so the test harness can run. `cargo build` still produces the WebAssembly plugin because `.cargo/config.toml` selects `wasm32-wasip1`.

After a code change, build again and reload the running plugin. Run this inside the session, from a shell whose working directory is `~/Development/<repository-name>`:

```bash
cargo build
zellij action start-or-reload-plugin \
  file:./target/wasm32-wasip1/debug/plugin.wasm
```

Use the same `file:` string the layout uses. Zellij treats a different path as a different plugin and opens another pane.

## Releasing

Plugin versions use Semantic Versioning: `1.0.0`, `1.1.0`, `2.0.0`. The version belongs to the plugin. It does not encode a Zellij version, a Rust version, or a WASI version. Keep the `version` in `Cargo.toml` and the git tag on the same number. Release tags use that number, such as `1.0.0`.

Pushing any branch runs `.github/workflows/ci.yml`:

```text
cargo fmt --check
cargo clippy
cargo test
cargo build
```

Clippy denies warnings. The test step passes the host target, as in Development. `.github/workflows/lint.yml` checks Prettier and markdownlint.

Pushing a tag such as `1.2.3` runs `.github/workflows/release.yml`. The workflow checks out the repository, installs Rust, installs the `wasm32-wasip1` target, runs `cargo build --release`, renames the artifact, and attaches that file to the GitHub release. The filename is `<github-user>-<repository>-v<version>.wasm`, taken from the GitHub repository and the tag. A tag `1.2.3` on this repository attaches `fulldecent-zellij-plugin-template-v1.2.3.wasm`. A copied repository gets its own user and repository in the filename without editing the workflow.

The release attaches that one file. It does not attach checksums or signatures. The version in the filename is the user-facing identifier.

## Maintenance and dependency updates

Do this every month or so and please send a PR here if you see updates available:

1. Identify external Actions in [.github/workflows](./.github/workflows) scripts and look for available new versions. Review and then update to the new version if it is safe. GitHub-supported Actions (i.e. under the actions/ organization) may require only cursory review.
1. Review the Rust toolchain in `rust-toolchain.toml` and the `zellij-tile` version in `Cargo.toml`. Update them together when a newer stable pair is appropriate, then refresh `Cargo.lock`.

## References

1. We use an MIT license for this template. You should carefully consider which license to apply to your own project.
1. Zellij offers another installation method that is simpler and insecure. That points your layout configuration to a HTTPS URL. We consider that feature wrong and deprecated. [TODO: create and link upstream issue]
1. This project is built based on [best practices documented in zellij-plugin-template](https://github.com/fulldecent/zellij-plugin-template), release 1.0.0.
1. This project is built based on [best practices documented in project-template](https://github.com/fulldecent/project-template), release 1.0.0.
