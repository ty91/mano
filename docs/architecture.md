# System Architecture

## Product direction

Mano is a local-first note-taking app combining Markdown documents with Notion-style properties, relations, and database views. Document content remains continuous Markdown text; the storage model does not divide it into independently editable blocks.

This is an initial design, not an implemented system. The first target platforms are macOS and iOS.

The initial implementation follows the [minimal desktop design](desktop.md). Start with a single app in `apps/desktop` and extract the shared packages and integrations below when actual consumers need them.

## Components

| Area | Direction |
| --- | --- |
| Mac app | GPUI-based UI and Markdown editor |
| iPhone app | SwiftUI/UIKit-based UI, note editing, and shared view definitions |
| Shared core | Rust models and rules for notes, properties, relations, queries, and mutations |
| Local storage | Markdown source files, SQLite search and relation indexes, and separate sync state |
| Device sync | Swift CloudKit/CKSyncEngine integration with each user's private database |
| Agent execution | Claude Code and Codex CLI processes on Mac |
| Agent access to the app | App CLI and stdio MCP server |
| Dynamic UI | Shared declarative view definitions and Mac JavaScript plugins |

## Monorepo layout

Executable and distributable units belong in `apps/`, reusable Rust code in `crates/`, and Apple-specific integration in `apple/`. This is the target layout; create directories and packages as they are implemented.

```text
mano/
├── apps/
│   ├── desktop/            # GPUI desktop app, targeting macOS first
│   ├── ios/                # Swift-based iPhone app
│   ├── cli/                # CLI for people and agents
│   └── mcp/                # stdio MCP adapter
├── crates/
│   ├── core/               # Note, property, relation, and view models and mutation rules
│   ├── storage/            # Markdown, indexes, change records, and file watching
│   ├── agents/             # Claude/Codex execution, events, cancellation, and sessions
│   └── extensions/         # JS runtime, host API, and plugin lifecycle
├── apple/
│   └── ManoCloudKit/       # Swift sync package shared by Mac and iOS
├── schemas/                # Shared view definitions and plugin manifest specifications
├── docs/
│   ├── architecture.md
│   └── desktop.md
└── README.md
```

Rust packages will use a root Cargo workspace; Apple packages will use Swift Package Manager. A JavaScript plugin runtime alone does not justify a Node/Bun workspace. Decide on one separately if frontend builds or SDK packaging require it.

`core` must not depend on GPUI, Swift UI frameworks, or agent providers. Define an explicit FFI boundary between Swift and the Rust core shared with iOS; do not assume UI code is shared.

## Data and synchronization

- `.md` files and frontmatter are the source of truth for note content and properties. Relations use stable note IDs rather than file paths.
- Simple tables within a document are stored as Markdown tables. Database views over multiple notes are stored as separate definitions.
- SQLite search and relation indexes are rebuilt from the source data. Pending changes, version information needed for conflict handling, and CKSyncEngine state are persisted separately from rebuildable indexes.
- Mutations from the UI, CLI, MCP, and plugins use the same core API. Initially, the desktop app coordinates mutations in-process. Separate CLI/MCP clients will connect to that mutation path through IPC. Changes made by external editors are detected and handled through the same flow.
- The Swift sync layer maps local changes to CloudKit records. It handles account changes, offline retries, deletions, and concurrent edits without silently overwriting note content on conflict.
- CloudKit removes the need to operate a separate server, but does not implement the app's entire sync logic. CloudKit records do not automatically appear as files in iCloud Drive.

## Agents and extensions

An execution manager on Mac starts `claude -p` or `codex exec` and forwards structured events to the UI. Provider adapters hide their differences. Local execution refers to the location of tools and processes; model inference in the default Claude/Codex setup runs in the cloud.

The CLI and MCP expose shared operations such as searching and editing notes, changing properties, creating views, and checking plugins. The stdio MCP server is a small process that runs when needed; a separate HTTP server is not required. Whether tasks must survive closing the app window determines the local core's process lifetime.

Dynamic UI has two levels:

1. **Declarative views:** Standardize tables, boards, forms, charts, and their data bindings. Agents generate definitions that Mac and iPhone render with their own UI implementations.
2. **Mac JS plugins:** Like Herdr mods, scripts compose panels and behavior while GPUI renders them. Extensions use the APIs exposed by the app; compatibility with the web DOM or arbitrary React libraries is not assumed.

Plugins receive API type declarations and a check command. Failed hot reloads retain the previous version. Define execution budgets, error isolation, and permissions for note access, networking, and external commands. Embedding a JS runtime alone does not provide a security boundary or process isolation.

## Platform boundaries and major costs

- **Mac:** Prefer GPUI with script extensions. Herdr/gpui-shell is a reference implementation; the version and stability of any adopted runtime require separate validation. The largest implementation costs are the Markdown editor, including Korean IME input, selection, undo, and table editing, and the plugin API.
- **iPhone:** Support shared data and declarative views. Do not assume Mac CLI tools or GPUI JS plugins can run unchanged. Executable plugins would require a separate assessment of iOS constraints and distribution rules.
- **Remote AI requests:** Asking a Mac agent to work from an iPhone is separate from document sync. The Mac and its execution process must be running; CloudKit does not execute jobs.
- **Distribution:** Prioritize evaluating direct Mac distribution with Developer ID signing and notarization for local tool integration. CloudKit remains available with the appropriate entitlements and provisioning.

## References

- [Herdr mods](https://github.com/herdrdev/herdr-app/blob/master/docs/mods.md)
- [GPUI Kit](https://github.com/longbridge/gpui-kit)
- [CKSyncEngine](https://developer.apple.com/documentation/cloudkit/cksyncengine-4b4w9)
- [Developer ID and CloudKit](https://developer.apple.com/developer-id/)
- [Programmatic Claude Code execution](https://code.claude.com/docs/en/headless)
- [Codex SDK](https://github.com/openai/codex/tree/main/sdk/typescript)
- [MCP transports](https://modelcontextprotocol.io/specification/2025-11-25/basic/transports)
