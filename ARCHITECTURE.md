# Architecture

Mano is a local-first note-taking app combining Markdown documents, structured properties and database views, and agent-built extensions. The project is in the design phase; the structure below describes the intended implementation.

## Start with a small desktop app

Start with a single Rust/GPUI app in `apps/desktop`, targeting macOS first. The initial scope covers a window and panel display, switching, and lifecycle management.

- **App:** Application startup and shutdown, window creation, and dependency wiring.
- **Workspace:** Panel registration, open instances, active state, and opening and closing panels.
- **Panel:** Its own view state, focus, subscriptions, and tasks.

Built-in panels use the same registration path. JS plugins can later connect through an adapter at this boundary. Closing a panel cleans up its tasks and prevents late results from being applied.

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
