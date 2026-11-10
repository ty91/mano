# Architecture

Mano is a local-first note-taking app combining Markdown documents, structured properties and database views, and agent-built extensions. A minimal desktop scaffold is implemented; the broader system below remains the intended design.

## Start with a small desktop app

The single Rust/GPUI app in `apps/desktop` targets macOS first. It currently opens one window and mounts one built-in panel through a factory. Panel switching and a dynamic registry are deferred until additional panels need them.

- **App:** Application startup and shutdown, window creation, and dependency wiring.
- **Workspace:** Panel registration, open instances, active state, and opening and closing panels.
- **Panel:** Its own view state, focus, subscriptions, and tasks.

The current registration point is `panels::DEFAULT_PANEL`, a factory returning GPUI's `AnyView`; Workspace does not depend on the concrete panel. The responsibilities listed above describe the intended expansion. JS plugins can later connect through an adapter at this boundary. When panel tasks are introduced, closing a panel must clean them up and prevent late results from being applied.

## Data and extension boundaries

- Markdown and frontmatter are the source of truth for note content and properties. Search indexes are rebuildable; pending changes and sync state are preserved separately.
- Note mutation rules are independent of the UI. The UI, CLI, MCP, and plugins use the same mutation path.
- Dynamic UI has two forms: declarative views shared across platforms and desktop JS plugins. Plugins operate through host-provided APIs and permissions.
- Provider adapters handle Claude/Codex execution and events. MCP uses stdio by default.
- The iPhone app uses a Swift-based UI. CloudKit integration belongs in an Apple-specific layer; mobile does not assume compatibility with desktop CLI execution or plugins.

## Monorepo layout

| Path | Responsibility |
| --- | --- |
| `apps/desktop` | GPUI desktop app |
| `apps/ios`, `apps/cli`, `apps/mcp` | Subsequent apps and external entry points |
| `crates/` | Rust core, storage, agent, and extension code extracted when reuse is needed |
| `apple/` | Swift-based CloudKit integration |
| `schemas/` | Declarative view and plugin manifest specifications |
| `docs/` | Detailed designs |

Create directories and packages as their features are implemented. Do not introduce a separate daemon, general-purpose event bus, or empty adapters upfront. Extract the shared core when a second consumer needs it, keeping it independent of GPUI and any particular agent provider.

## Detailed documents

- [Minimal desktop design](docs/desktop.md)
- [System design, data, and synchronization](docs/architecture.md)
