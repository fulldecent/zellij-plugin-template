# Development layouts

Start Zellij from `~/Development/<repository-name>`.

> [!TIP]
> Replace `<repository-name>` with the repository name. For this repository, that directory is `~/Development/zellij-plugin-template`. Remove this note from your repository.

Zellij resolves a relative `file:` location against that directory. These layouts use `file:./target/wasm32-wasip1/debug/plugin.wasm`.

| Layout | Launch |
| --- | --- |
| Status bar | `zellij -l dev/plugin-dev.status-bar.kdl` |
| Tab bar | `zellij -l dev/plugin-dev.tab-bar.kdl` |
| Pane | `zellij -l dev/plugin-dev.pane.kdl` |
| Floating pane | `zellij -l dev/plugin-dev.floating-pane.kdl` |

Build first with `cargo build`. The debug artifact is `~/Development/<repository-name>/target/wasm32-wasip1/debug/plugin.wasm`.

Select the layout that matches the plugin you are writing. Delete the others after initial setup.
