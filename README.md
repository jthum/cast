# Cast

Cast is a themeable component library for `egui`, inspired by shadcn and built for immediate-mode Rust applications.

Cast provides polished, composable Rust widgets for building product interfaces with `egui`. It is not a DOM, CSS, or Radix port. The goal is to bring the spirit of shadcn-style application components to native immediate-mode UI: consistent defaults, semantic theme tokens, runtime theme switching, and APIs that feel natural in Rust.

The repository and project are called `cast`. The crates.io package is `cast-ui`, because it is explicit and available. In application code, prefer aliasing the dependency as `cast` so widgets read cleanly as `cast::Button`, `cast::Table`, and `cast::AgentComposer`.

## Install

In `Cargo.toml`, depend on the `cast-ui` package under the local crate name `cast`:

```toml
[dependencies]
cast = { package = "cast-ui", version = "0.1" }
eframe = "0.34"
```

For local workspace development, this repository already uses the same alias:

```toml
[workspace.dependencies]
cast = { package = "cast-ui", path = "crates/cast-ui" }
```

## Quick Start

Install Cast fonts and set a runtime theme when your `eframe` app starts:

```rust
eframe::run_native(
    "My Cast App",
    native_options,
    Box::new(|cc| {
        cast::install_cast_fonts(&cc.egui_ctx);
        let theme = cast::ThemeSeed::for_mode(cast::ThemeMode::Light).resolve();
        cast::set_theme(&cc.egui_ctx, theme);
        Ok(Box::new(MyApp::default()))
    }),
)
```

Use widgets directly in `egui` UI code:

```rust
ui.add(cast::Button::new("Run"));
ui.add(cast::Badge::new("Ready").intent(cast::Intent::Success).status_dot());
ui.add(cast::TextInput::new(&mut self.name).hint_text("Project name"));
```

Themes are ordinary runtime values. Update the seed, resolve it, and call `cast::set_theme(ctx, theme)` whenever the user changes mode, palette, density, radius, typography, or component overrides.

## What Cast Includes

- Runtime-changeable theme model with light and dark defaults.
- Semantic color families for primary, secondary, neutral, success, warning, danger, and info.
- Tokens for colors, spacing, radius, stroke, typography, controls, tone, motion, scroll, elevation, and component roles.
- Core controls including `Button`, `Badge`, `Checkbox`, `Radio`, `Switch`, `Slider`, `TextInput`, `SearchInput`, `TextArea`, `Select`, `Dropdown`, `Combobox`, `Tabs`, and `SegmentedControl`.
- Surfaces and overlays including `Card`, `Panel`, `Alert`, `Tooltip`, `Popover`, `HoverCard`, `Dialog`, `Sheet`, `Disclosure`, and `Accordion`.
- Data display primitives including `Table`, `TextTable`, expandable table details, list rows, pagination, calendars, progress, skeletons, and report/chart primitives.
- Agent-oriented primitives including `AgentComposer`, `MessageThread`, `ChatMessage`, `ToolCall`, `ToolCallBlock`, `RunTimeline`, `CodeOutputPanel`, `ArtifactCard`, `ApprovalPanel`, `ContextPanel`, `PlanList`, and `PatchReviewPanel`.
- A native gallery app for visual documentation, theme testing, component examples, and reusable app patterns.

## Workspace

- `crates/cast-ui`: reusable `egui` component library published as `cast-ui`.
- `crates/cast-gallery`: native gallery app for visual documentation, layout exploration, and theme testing.
- `crates/cast-font-tool`: helper tooling for font fetching and packaging.
- `crates/eframe-baseline`: small baseline app for comparing native `eframe` resource usage.

## Development

Check the workspace:

```sh
cargo check --workspace
```

Format the workspace:

```sh
cargo fmt --all
```

Run tests:

```sh
cargo test
```

Run the gallery:

```sh
cargo run -p cast-gallery
```

## Documentation

- [Getting Started](docs/getting-started.md): dependency aliasing, app setup, fonts, themes, and first widgets.
- [Components](docs/components.md): component families, naming, table guidance, and agent workflow primitives.
- [Design Tokens](docs/tokens.md): runtime theme model, token groups, derivation rules, and overrides.
- [Patterns](docs/patterns.md): reusable composed UI blocks that live in the gallery while their shape is still being proven.

## Design Direction

Cast should feel like a native Rust UI library, not a web compatibility layer. Components should be themeable, accessible by default, and composed from semantic tokens rather than arbitrary local styling. The gallery should show both primitive variants and realistic product screens, because a component can look good in isolation and still feel too busy in an actual workflow.

The current bias is toward calm, dense desktop interfaces: clear hierarchy, quiet neutral chrome, contextual disclosure, strong runtime theming, and enough component coverage for real application screens. Agentic workflows are first-class because Turin-style interfaces need chat, tools, logs, approvals, artifacts, and editable structured output, but Cast itself remains a general-purpose `egui` component library.
