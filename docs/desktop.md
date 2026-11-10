# Minimal Desktop Design

The scaffold in `apps/desktop` is a single GPUI app targeting macOS first. It opens one window, mounts one built-in panel, and exits when the last window closes or the user quits. The editor, storage, CloudKit, agents, CLI/MCP, and JS runtime are subsequent features.

## Implemented boundary

`App` configures the window, menus, and shutdown behavior. `Workspace` retains an `AnyView` created by the `PanelFactory` registered as `panels::DEFAULT_PANEL`. The built-in Home panel owns its rendering and follows the system appearance. The factory accepts the window and app context so later panels can create their own GPUI entities without exposing their concrete types to Workspace.

The dependency is the official `gpui` crate pinned to `0.2.2`, with resolved dependencies committed in the root lockfile. GPUI Kit and its script runtime are not dependencies. Their current release line uses a different GPUI package/version; adopting them requires an explicit compatibility decision rather than mixing the two into this scaffold.

There is no dynamic registry, panel switching, background task, or persistence yet. The contracts below are the target for those additions; the first scaffold deliberately implements only a single factory slot. Run and verification commands are in the [README](../README.md#development).

## Usage contract

| Operation | Input and result | Failure behavior |
| --- | --- | --- |
| Open a panel | Request a registered panel kind to create and activate an instance | Unknown kinds or creation failures return an error and preserve the current panel |
| Switch panels | Select an open instance to change the active panel | An unknown instance returns an error. Existing panel state is preserved |
| Close a panel | Remove an instance and clean up its subscriptions and tasks | Closing an already closed instance has no effect. Late results cannot revive it |

Initially, display one panel at a time. Multiple panels can retain their state, but tabs, splits, and docking UI are added only when needed. Opening the same panel kind again initially activates its existing instance.

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
└── src/
    ├── main.rs         # Entry point
    ├── app.rs          # Window and dependency composition
    ├── workspace.rs    # Retained panel view and window close action
    └── panels/
        ├── mod.rs      # Panel factory type and default registration
        └── home.rs     # Built-in Home panel
```

Do not create a crate for every feature. Keep registrations in one place rather than maintaining a duplicate command list. Do not introduce a general-purpose event bus, service container, empty platform adapters, or separate daemon initially. Keep Workspace internals private to its module and allow panel mutations only through open, activate, and close operations.

## Extension points

- **JS plugins:** A runtime host wraps scripts as panels and registers them. Explicit host APIs provide access to app state. Choose the runtime, permissions, execution limits, and failure isolation when implementing the integration. The panel contract alone does not isolate scripts.
- **Declarative views:** Add a panel that reads view definitions. The definition format must not contain GPUI objects so other platforms can render the same data.
- **CLI/MCP:** Route window operations to Workspace actions and note operations to the note model. Keep transport JSON and IPC formats out of the panel's internal contract.

Implement a window and one built-in panel first, then a second panel using the same contract and lifecycle checks, followed by functional panels. Establish a stable public plugin API from that usage experience.

## Verification plan

The following checks cover the planned panel-management behavior and have not been run. The current scaffold requires a real macOS launch, appearance and resize inspection, and verification of titlebar close, `Cmd-W`, and `Cmd-Q`; no automated behavioral suite exists yet.

- Switch between two built-in panels and verify state preservation and focus changes. Confirm that adding a panel requires only its implementation and registration, without modifying existing panels.
- Confirm that duplicate registration, unknown kinds, and creation failures do not change open panels or active state.
- Verify state invariants and subscription cleanup when closing the active panel, closing it repeatedly, and closing the last panel.
- When asynchronous work is introduced, verify that results for closed panels and queries completing out of order cannot overwrite the current view.
- Verify keyboard focus, resizing, and shutdown in a real macOS window. Module checks alone do not establish actual UI behavior.
