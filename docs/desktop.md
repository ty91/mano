# Minimal Desktop Design

The scaffold in `apps/desktop` is a single GPUI app targeting macOS first. It opens one window with a sidebar and a tab bar, hosts one panel per tab, and exits when the last window closes or the user quits. The editor, storage, CloudKit, agents, CLI/MCP, and JS runtime are subsequent features.

## Implemented boundary

`App` configures the window, menus, shutdown behavior, and the embedded asset source. The window uses a transparent titlebar so the sidebar extends under the traffic lights. `Workspace` lays out a fixed-width sidebar beside a tab bar and the active tab's panel, and refreshes the window when the system appearance changes. Views read their colors from `Theme::of(window)`.

Each tab holds a `Panel`, a title plus an `AnyView`, created by the `PanelFactory` registered as `panels::DEFAULT_PANEL`; currently that is an empty panel titled "새 탭". The factory accepts the window and app context so later panels can create their own GPUI entities without exposing their concrete types to Workspace. Inactive tabs keep their views, so panel state survives switching.

`workspace::NewTab` (`Cmd-T`, the + button, or File > New Tab) opens and activates a new tab. Clicking a tab activates it. `workspace::CloseTab` (`Cmd-W` or File > Close Tab) closes the active tab and activates the most recently active remaining tab; closing the only tab closes the window, which quits the app. The titlebar close button closes the window regardless of how many tabs are open.

The sidebar renders a static list of sections and entries. Selecting an entry and expanding or collapsing a folder change only sidebar state; entries do not open tabs yet. Icons are Lucide SVGs embedded at compile time from `assets/icons`, which also holds their license.

## Keyboard shortcuts

Shortcuts are data, not code. `keymap::DEFAULT_BINDINGS` lists each binding as a keystroke sequence, a registered action name such as `workspace::NewTab`, and an optional key context. At startup `keymap::load` resolves names with GPUI's action registry, parses contexts, and binds the result; an unknown action name, keystroke, or context is an error rather than a silently missing shortcut. macOS menu items show their shortcuts from the same bindings.

To add a shortcut, declare the action with `actions!` in the module that handles it, handle it on the element that owns the behavior, and add one row to the table. Use the `Workspace` context for window-level commands; a panel that needs its own shortcuts sets its own `key_context` so its bindings apply only while it has focus. User keymap files or plugin bindings can later be parsed into the same `Binding` form and loaded after the defaults; GPUI gives later bindings precedence.

The dependency is the official `gpui` crate pinned to `0.2.2`, with resolved dependencies committed in the root lockfile. GPUI Kit and its script runtime are not dependencies. Their current release line uses a different GPUI package/version; adopting them requires an explicit compatibility decision rather than mixing the two into this scaffold.

There is no dynamic registry, panel kind selection, background task, or persistence yet. The contracts below are the target for those additions; tabs currently use a single factory slot. Run and verification commands are in the [README](../README.md#development).

## Usage contract

| Operation | Input and result | Failure behavior |
| --- | --- | --- |
| Open a panel | Request a registered panel kind to create and activate an instance | Unknown kinds or creation failures return an error and preserve the current panel |
| Switch panels | Select an open instance to change the active panel | An unknown instance returns an error. Existing panel state is preserved |
| Close a panel | Remove an instance and clean up its subscriptions and tasks | Closing an already closed instance has no effect. Late results cannot revive it |

Each tab displays one panel instance, and one tab is active at a time. Opening a new tab always creates a new instance. Splits and docking UI are added only when needed.

## Selected structure

| Candidate | Benefit | Cost and decision |
| --- | --- | --- |
| Built-in Rust panels with a small registration point | Window behavior, input, and state remain directly traceable; the first app works without JS | Requires a script adapter later, but meets the current needs and is selected |
| Compose every screen as a JS mod from the start | Agents can immediately modify the whole app | Requires the runtime, API, reload behavior, and error isolation before the first screen; deferred |

The app uses registered factories rather than branching on panel implementations. Built-in panels use the same path. This is a desktop-specific boundary for GPUI views and lifetimes, not a general plugin SDK or a shared iOS UI specification.

## Ownership and boundaries

- **App:** Owns startup and shutdown, window creation, built-in panel registration, and dependency wiring. It does not store note data or individual panel state.
- **Workspace:** Owns panel kind registrations, open instances, and the active instance. Kind IDs and instance IDs are distinct, and duplicate kind registration is rejected. The active instance must either be open or absent.
- **Panel:** Owns its view state, focus target, subscriptions, and tasks. It is created when opened and cleaned up when closed. It does not directly mutate another panel's internal state or Workspace storage.
- **Note model:** When editing is introduced, own content, properties, and mutation rules independently of the UI. Panels receive query results and request changes. Extract this boundary into `crates/core` when another consumer, such as a separate CLI or iOS app, needs it.
- **Platform integration:** Isolate OS-specific APIs in `platform/` when they are introduced. Do not add integrations such as CloudKit to the basic Workspace or panel interface.

Closing the active panel selects the most recently active remaining panel. If no panels remain, there is no active instance. Workspace coordinates removal and cleanup so callers do not need to sequence them.

Panel tasks must not block GPUI's UI execution. When asynchronous queries are introduced, identify results by panel instance and request generation; only the latest request may update the view. Closing requests cancellation and prevents subsequent results from being applied. It does not promise to reverse external effects that have already completed. The initial scope does not persist tasks or view state or resume them after shutdown.

## File layout

The scaffold uses this layout:

```text
apps/desktop/
├── Cargo.toml
├── assets/icons/       # Embedded Lucide SVG icons and license
└── src/
    ├── main.rs         # Entry point
    ├── app.rs          # Window and dependency composition
    ├── assets.rs       # Embedded asset source and icon names
    ├── keymap.rs       # Default key bindings and binding loader
    ├── theme.rs        # Appearance-dependent colors
    ├── workspace.rs    # Tabs, tab actions, and window layout
    ├── sidebar.rs      # Static navigation sidebar
    └── panels/
        ├── mod.rs      # Panel type, factory type, and default registration
        └── blank.rs    # Empty panel for new tabs
```

Do not create a crate for every feature. Keep registrations in one place rather than maintaining a duplicate command list. Do not introduce a general-purpose event bus, service container, empty platform adapters, or separate daemon initially. Keep Workspace internals private to its module and allow panel mutations only through open, activate, and close operations.

## Extension points

- **JS plugins:** A runtime host wraps scripts as panels and registers them. Explicit host APIs provide access to app state. Choose the runtime, permissions, execution limits, and failure isolation when implementing the integration. The panel contract alone does not isolate scripts.
- **Declarative views:** Add a panel that reads view definitions. The definition format must not contain GPUI objects so other platforms can render the same data.
- **CLI/MCP:** Route window operations to Workspace actions and note operations to the note model. Keep transport JSON and IPC formats out of the panel's internal contract.

Implement a window and one built-in panel first, then a second panel using the same contract and lifecycle checks, followed by functional panels. Establish a stable public plugin API from that usage experience.

## Verification plan

Unit tests use GPUI's test platform to check that the default bindings resolve, that unknown actions are rejected, and that `Cmd-T`/`Cmd-W` open and close tabs, return to the most recently active tab, and close the window after the last tab. A real macOS launch is still needed for appearance, resizing, focus after clicks, the titlebar close button, and `Cmd-Q`. The following checks cover planned panel-management behavior and have not been run.

- Switch between two built-in panels and verify state preservation and focus changes. Confirm that adding a panel requires only its implementation and registration, without modifying existing panels.
- Confirm that duplicate registration, unknown kinds, and creation failures do not change open panels or active state.
- Verify state invariants and subscription cleanup when closing the active panel, closing it repeatedly, and closing the last panel.
- When asynchronous work is introduced, verify that results for closed panels and queries completing out of order cannot overwrite the current view.
- Verify keyboard focus, resizing, and shutdown in a real macOS window. Module checks alone do not establish actual UI behavior.
