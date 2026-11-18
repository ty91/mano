# Mano

A local-first note-taking app that combines free-form Markdown writing with database views. Mano aims to support iCloud sync between Mac and iPhone, local agent integration, and agent-built interfaces and plugins.

The desktop scaffold opens a GPUI window with a static navigation sidebar and empty tabs. Note editing, storage, synchronization, agents, and plugins are planned. This monorepo contains the app and will host its shared core as additional consumers need it.

## Development

The initial target is macOS. Install Rust 1.93.1 or later and full Xcode with its command-line tools selected. GPUI builds native code and Metal shaders; the standalone Command Line Tools installation is not sufficient.

```sh
cargo run --locked -p mano-desktop
```

The window follows the system appearance. Open a tab with `Cmd-T` or the + button and close the active tab with `Cmd-W`. Closing the last tab or the window, or quitting with `Cmd-Q` or the Mano menu, exits the process.

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --locked
```

Unit tests cover the keymap and tab behavior on GPUI's test platform. Verify the actual window, appearance, resizing, and each exit path on macOS; the test platform does not establish those behaviors.

## Design

- [Architecture overview](ARCHITECTURE.md)
- [System design and repository layout](docs/architecture.md)
- [Minimal desktop design](docs/desktop.md)
