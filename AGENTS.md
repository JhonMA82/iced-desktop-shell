# Guidelines for AI Agents Working on `iced-desktop-shell`

Welcome to `iced-desktop-shell`. Please follow these mandatory architectural rules and workflows when inspecting, modifying, or extending this repository.

---

## 1. Architectural Roles

- **`src/app/`**: Application orchestrator. Owns `AppState`, dispatches messages, routes commands to shell and domain handlers, and manages task propagation.
- **`crates/desktop-shell/src/`**: Reusable technical desktop shell infrastructure (independent `desktop-shell` crate). Zero domain knowledge. Defines generic commands, dock layout, panels, ribbons, tokens, and preferences. Contains no concrete application commands by default.
- **`src/demo/`**: Replaceable technical demo showcasing Explorer, Workspace canvas, Inspector properties, Output diagnostics, and domain-specific commands.

---

## 2. Hard Constraints (What Agents Must NEVER Do)

1. **NEVER inject domain or business logic into `crates/desktop-shell/src/`**: If a concept relates to CAD, CNC, projects, components, or domain data, it belongs in `demo/` (or the client crate), never in `desktop-shell`.
2. **NEVER register domain commands inside `crates/desktop-shell/src/`**: The shell provides `Command`, `CommandId`, `CommandRegistry`, and `Shortcut`. All concrete commands (`app.*`, `view.*`, `help.*`) must be defined and registered in `src/demo/commands.rs`.
3. **NEVER trigger side-effects or direct logic from widgets**: Widgets must emit `ShellMessage::ExecuteCommand(CommandId)` or domain messages.
4. **NEVER disperse magic numbers**: Spacing, radii, heights, and panel dimensions must use constants from `crates/desktop-shell/src/theme/tokens.rs`.
5. **NEVER introduce unjustified dependencies**: Do not add `iced_aw`, plugin runtimes, or scripting engines unless explicitly instructed.
6. **NEVER use unconditional `.unwrap()` or `.expect()` in application paths**: Handle errors gracefully, log with `tracing`, and fall back to safe defaults.

---

## 3. Iced 0.14 API Constraints (Learned 2026-09)

The crate targets `iced = "0.14"`. Its widget API differs from older examples — do not reintroduce removed helpers:

- **No `horizontal_space` / `horizontal_rule` / `vertical_rule`**: use `Space::new().width(Length::Fill)` for horizontal gaps and `iced::widget::rule::horizontal(n)` / `rule::vertical(n)` for dividers.
- **`row()` / `column()` helpers require a children iterator** (`row([a, b])`); for incremental builder style use `Row::new()` / `Column::new()` with `.push()`.
- **`Space::new()` takes zero arguments**: set size via `.width()` / `.height()` builders.
- **`rule::Style` has no `Default` impl**: construct it with all fields (`color`, `radius`, `fill_mode`, `snap`). `button::Style` and `container::Style` still support `..Default::default()`.
- **Generic views need `M: Clone`**: converting `Button` into `Element` requires it — keep the bound consolidated in the `where` clause (e.g. `M: From<ShellMessage> + Clone + 'a`) to satisfy `clippy::multiple_bound_locations`.
- **Padding arrays must be uniform**: `[0, tokens::SPACING_SM]` fails (int vs float) — write `[0.0, tokens::SPACING_SM]`.
- **Prefer `.as_slice()`** when passing a local array as `&[T]`; other formulations confuse rust-analyzer.

---

## 4. Step-by-Step Workflows

### How to Add a Domain Feature (Application-Specific)

1. Add state in `src/demo/state.rs`.
2. Add corresponding action in `src/demo/message.rs` (`DemoMessage`).
3. Handle state mutation in `src/app/update.rs` (`handle_demo_message`).
4. Update UI in `src/demo/explorer.rs`, `src/demo/workspace.rs`, or `src/demo/inspector.rs`.

### How to Add an Application Action / Command

1. Define the command ID constant in `src/demo/commands.rs` (e.g. `APP_NEW`, `APP_QUIT`, `VIEW_*`).
2. Register the command in `register_demo_commands()` in `src/demo/commands.rs` with its label, description, icon, and shortcut.
3. Add the handler function in `src/app/handlers.rs` and register it in `command_table()`.
4. Expose the command in Ribbon tabs (`demo_ribbon_tabs()` in `src/demo/mod.rs`) or Menu bar (`menu_items` in `src/app/view.rs`).

---

## 5. Verification Checklist

Before submitting changes, ensure:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo check
```

---

## 6. Scaffolding

`iced-desktop-shell` ships a declarative application generator: one core, presets as data plus composition (no `templates/<preset>/` copies of whole apps).

```bash
cargo xtask generate my-app --layout technical-ribbon
cargo xtask generate machine-tool --layout technical-ribbon --without inspector
cargo xtask generate converter --layout minimal --theme light
cargo xtask list-presets   # presets with defaults and panel slots (agent-friendly)
cargo xtask list-features  # valid --with / --without values
```

- Presets: `technical-ribbon`, `ide`, `studio`, `operator`, `minimal` (kebab names are the CLI contract).
- Only implemented capabilities are accepted in `--with` (`ribbon`, `menu`, `toolbar`, `activity-bar`, `explorer`, `inspector`, `bottom-panel`, `statusbar`, `persistence`, `theme`); anything else is a clear error.
- Incoherent combinations are rejected, never generated broken: `--without workspace` fails, `ribbon` requires `technical-ribbon`, `activity-bar` requires `ide`.
- Generated projects are standalone, compilable crates depending on `desktop-shell` (git URL by default, `--shell-path <dir>` for local verification) plus `iced` only. The output is normal, editable code: `src/main.rs` (runner), `src/app.rs` (State/Message/Update/View), `src/demo.rs` (single-file domain stub), `.scaffold.toml` (informative metadata, not a source of truth).
- Generator crates `crates/scaffold` (model + validation + render, zero external deps) and `xtask` (thin CLI, zero external deps) must stay dependency-free: hand-rolled arg parsing, TOML emitted as text.
- Full direction: `docs/scaffolding-direction.md`.
