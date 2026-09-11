# Architecture Overview — `iced-desktop-shell`

## 1. Objective

`iced-desktop-shell` is an experimental technical desktop application shell boilerplate in Rust using Iced 0.14. It establishes a clean, decoupled, and maintainable foundation for technical, engineering, industrial, and productivity desktop software.

---

## 2. Decoupled Structure: App vs Shell vs Demo

```
┌─────────────────────────────────────────────────────────┐
│                       src/app/                          │
│          Application Orchestrator & Dispatch            │
├────────────────────────────┬────────────────────────────┤
│         src/shell/         │         src/demo/          │
│   Reusable UI Foundation   │   Replaceable Domain Demo  │
│  (Ribbon, Panels, Theme,   │   (Tree, Entities, Canvas, │
│    Commands, Persistence)  │    Diagnostics, Inspector) │
└────────────────────────────┴────────────────────────────┘
```

- **`src/shell/`**: Reusable desktop shell infrastructure. Completely domain-agnostic. Contains zero references to CAD, CNC, or application entities. Can be extracted to a standalone crate (`crates/desktop-shell`) without refactoring.
- **`src/demo/`**: Replaceable technical demo. Registers its own commands (`app.new`, `app.open`, `app.save`, `app.quit`) in `src/demo/commands.rs`, panels (`demo.explorer`, `demo.inspector`, `demo.output`), and ribbon tabs in `src/demo/mod.rs`.
- **`src/app/`**: Root orchestrator. Connects `ShellState` and `DemoState`, routes messages, and handles command dispatching and task propagation.

---

## 3. The State → Message → Update → View Cycle

1. **State**: The application state is fully deterministic and tree-structured:

   ```rust
   pub struct AppState {
       pub shell: ShellState,
       pub demo: DemoState,
   }
   ```

2. **Message**: Discrete user actions are captured in hierarchical enums:

   ```rust
   pub enum Message {
       Shell(ShellMessage),
       Demo(DemoMessage),
       Event(iced::Event, iced::event::Status),
   }
   ```

3. **Update**: Pure state transition function returning an asynchronous `Task<Message>`:

   ```rust
   pub fn update(state: &mut AppState, message: Message) -> Task<Message>
   ```

   Command execution directly returns `Task<Message>`. For example, `app.quit` returns `iced::exit()`, while standard commands return `Task::none()`.
4. **View**: Declarative view construction rendering the shell frame and slotting domain components into dedicated panel regions:

   ```rust
   pub fn view(state: &AppState) -> Element<'_, Message>
   ```

---

## 4. Application Boot & Iced 0.14 Lifecycle

The application initializes via `iced::application`:

```rust
iced::application(AppState::new, update, view)
    .title("iced-desktop-shell")
    .theme(|state: &AppState| state.shell.theme.to_iced_theme())
    .subscription(|_state: &AppState| {
        event::listen_with(|event, status, _window| {
            Some(Message::Event(event, status))
        })
    })
    .window(...)
    .run()
```

`AppState::new()` returns `(AppState, Task<Message>)` as the boot function, cleanly aligning with Iced 0.14 idioms.

---

## 5. Style Compatibility in Iced 0.14

`container::Style` and `button::Style` are instantiated using `..Default::default()` for forwards-compatibility. `rule::Style` is the exception: it does **not** implement `Default` in Iced 0.14, so it must be constructed with all fields explicitly (`color`, `radius`, `fill_mode`, `snap`).

---

## 6. Generic Panel System & Layout Order

- **`PanelId`**: Extensible string-based identifier (`PanelId(String)`).
- **`DockLayout`**: Dynamic collection of panels (`HashMap<PanelId, PanelState>`) queryable by `PanelLocation` (`Left`, `Right`, `Bottom`).
- **Single Source of Truth**: `DockLayout.order` defines the display order of panels.
- **Ownership in `render_shell`**: Panels are classified by location into owned vectors (`left_panels`, `right_panels`, `bottom_panels`) and moved into the widget tree without borrowing or cloning.
- **Dynamic Sizing**: Uses `panel.size` in rendering rather than fixed token constants.

---

## 7. Global Shortcuts & Event Status

Shortcuts are listened to using `event::listen_with`, allowing inspection of `iced::event::Status`. Keystrokes are only processed as global shortcuts when `status == iced::event::Status::Ignored`, preventing accidental triggering when user input widgets are focused.

---

## 8. Pure Default & Resilient Persistence

- `ShellState::default()` and `AppState::default()` are pure and perform zero disk I/O.
- Preferences are loaded explicitly during application startup (`AppState::new`).
- `ShellPreferences` uses `#[serde(default)]` to safely survive missing or newly added JSON fields.
