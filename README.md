# Mano

A local-first note-taking app that combines free-form Markdown writing with database views. Mano aims to support iCloud sync between Mac and iPhone, local agent integration, and agent-built interfaces and plugins.

The desktop scaffold opens a GPUI window with one built-in panel. Note editing, storage, synchronization, agents, and plugins are planned. This monorepo contains the app and will host its shared core as additional consumers need it.

## Development

The initial target is macOS. Install Rust 1.93.1 or later and full Xcode with its command-line tools selected. GPUI builds native code and Metal shaders; the standalone Command Line Tools installation is not sufficient.

```sh
cargo run --locked -p mano-desktop
```

The window follows the system appearance. Close it with the titlebar button or `Cmd-W`, or quit with `Cmd-Q` or the Mano menu. Closing the last window exits the process.

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --locked
```

There are no automated behavioral tests yet. Verify the actual window, resizing, and each exit path on macOS; compilation does not establish those behaviors.

## Design

- [Architecture overview](ARCHITECTURE.md)
- [System design and repository layout](docs/architecture.md)
- [Minimal desktop design](docs/desktop.md)
