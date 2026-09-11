# iced-desktop-shell

> An experimental technical desktop application shell boilerplate built with Rust and [Iced](https://iced.rs).

`iced-desktop-shell` is an experimental starter boilerplate designed for building technical, engineering, industrial, and productivity desktop applications in Rust. It explores a classic desktop shell layout using a top menu bar, data-driven ribbon, collapsible panels, properties inspector, diagnostics output, status bar, and centralized commands.

![iced-desktop-shell screenshot](assets/screenshot.png)

---

## Architectural Principles & Scope

- **Clean Elm Architecture**: State / Message / Update / View pattern with clear separation between reusable shell framing (`src/shell/`) and domain logic (`src/demo/`).
- **Data-Driven Ribbon**: Tabs, groups, and items driven by data structures referencing command IDs.
- **Centralized Command System**: Widgets emit `CommandId` dispatched centrally; command execution can return asynchronous tasks (such as `iced::exit()`).
- **Generic Panel Registry**: Panels are registered dynamically via `PanelId`, `PanelLocation` (Left, Right, Bottom), and configurable sizes (`PanelState.size`). `DockLayout.order` is the single source of truth for panel arrangement.
- **Shortcuts with Event Status**: Listens using `event::listen_with` to ensure global keyboard shortcuts do not intercept keystrokes in active input widgets.
- **Design Tokens & Theming**: Design tokens (`tokens.rs`) and color palettes (`DARK_PALETTE`, `LIGHT_PALETTE`) with Dark Mode as default.
- **Pure Defaults & Persistence**: `ShellState::default()` is pure (no disk I/O); persistent preferences are loaded explicitly by `AppState::new()` and deserialize safely with `#[serde(default)]`.
- **Licensing**: Dual MIT / Apache-2.0.

> **Note on Docking**: This initial v0.1 version implements a multi-panel layout manager governing panel visibility, positions, sizes, and layout order. It is structured so that full `PaneGrid`-based docking (splits, dragging, and tabbed reorganization) can be introduced in subsequent work without changing the core application API.

---

## Intended Target Domains

- Industrial configuration and diagnostics utilities
- CNC and motion control interfaces
- Hardware test and telemetry dashboards
- CAD/CAM and technical geometry viewers
- Embedded devtools and network analyzers

---

## Project Structure

```text
iced-desktop-shell/
├── Cargo.toml                  # Simplified dependencies (iced = "0.14")
├── Cargo.lock                  # Lockfile
├── .gitignore                  # Git ignore rules
├── rust-toolchain.toml         # Rust toolchain pinning (stable)
├── README.md                   # Project overview
├── AGENTS.md                   # AI agent constraints and guidelines
├── LICENSE-MIT                 # MIT license
├── LICENSE-APACHE              # Apache 2.0 license
├── .github/workflows/ci.yml    # CI workflow
├── docs/ARCHITECTURE.md        # Technical architecture document
├── assets/                     # Static assets (README screenshot)
├── crates/desktop-shell/       # Independent shell crate (commands, dock, ribbon, theme, persistence)
│   ├── Cargo.toml              # Package `desktop-shell` v0.1.0 (path dependency of the app)
│   ├── src/                    # Zero domain knowledge: generic shell infrastructure
│   └── tests/shell_tests.rs    # Shell-only tests
├── tests/app_tests.rs          # App/demo-level tests
└── src/
    ├── lib.rs                  # Library root exposing app and demo
    ├── main.rs                 # Tracing subscriber & application runner
    ├── app/                    # Application coordination (State, Message, Update, View)
    └── demo/                   # Technical demo (Application-level domain)
        ├── commands.rs         # Demo command registration (app.new, app.open, app.quit, etc.)
        ├── explorer.rs         # Project explorer tree panel
        ├── workspace.rs        # Technical preview viewport
        ├── inspector.rs        # Properties editor
        └── state.rs            # Demo state, entities, and diagnostics log
```

> The reusable shell lives in the independent `desktop-shell` crate, so the compiler enforces the shell/domain boundary.

---

## Getting Started

### Prerequisites

- Rust stable via [rustup](https://rustup.rs/) (the exact toolchain is pinned in `rust-toolchain.toml`).
- A graphical session (X11 or Wayland) to run the application window.
- On Debian/Ubuntu Linux, the system libraries installed by CI: `libxkbcommon-dev libwayland-dev libasound2-dev libfontconfig1-dev`.

### Running the Application

```bash
cargo run
```

For smoother rendering use `cargo run --release` (slower first build). The first build takes a while because it compiles the Iced GUI stack; subsequent builds are incremental.

### Quick Tour of the Demo

1. Select **Component A** in the Project Explorer (left panel).
2. Adjust its **X / Y** coordinates and enabled state in the Properties inspector (right panel).
3. Toggle the theme with **Appearance → Toggle Theme** in the ribbon, or clear the log from the Output panel (bottom).

### Running Checks and Tests

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo check
```

---

## Generating New Applications (Scaffolding)

This repository is also a generator for technical desktop apps (see `docs/scaffolding-direction.md` for the full direction). One core, presets as data plus composition:

```bash
cargo xtask generate printnc-config --layout technical-ribbon
cargo xtask generate machine-tool --layout technical-ribbon --without inspector
```

Presets (`cargo xtask list-presets`): `technical-ribbon` (CAD/CAE-style ribbon shell), `ide` (workbench with activity rail), `studio` (workspace-first canvas), `operator` (monitoring/HMI), `minimal` (single-task utility). Capabilities are tuned with `--with` / `--without` (`cargo xtask list-features`); generated projects are standalone crates with normal, editable code.

---

## License

Dual-licensed under either:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))
