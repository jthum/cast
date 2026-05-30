# Cast

Cast is a themeable component library for `egui`, inspired by shadcn and built for immediate-mode Rust applications.

Cast provides polished, composable Rust widgets for building product interfaces with `egui`. It is not a DOM or CSS port. The goal is to bring the spirit of shadcn-style application components to native immediate-mode UI: consistent defaults, semantic theme tokens, runtime theme switching, and APIs that feel natural in Rust.

The repository is named `cast`. The published crate is intended to be named `cast-ui`, with the Rust module path `cast_ui`.

## Goals

- Provide a serious app UI foundation for `egui`.
- Keep theming runtime-changeable and Rust-native.
- Prefer semantic design tokens over hardcoded component styling.
- Ship strong light and dark defaults.
- Support reusable product UI primitives without encoding app-specific concepts.
- Maintain a gallery that acts as documentation, visual coverage, and a theme test bed.

## Non-Goals

- Cast is not a Turin-specific UI crate.
- Cast does not implement Turin's semantic UI protocol.
- Cast does not try to reproduce browser layout, CSS, or Radix primitives.
- Cast does not require a single visual style or compile-time-only themes.

## Workspace

- `crates/cast-ui`: reusable `egui` component library.
- `crates/cast-gallery`: native gallery app for visual documentation and theme testing.

## Current Status

This repository is in active component-library prototyping. The workspace currently contains:

- Runtime-changeable theme model with light and dark defaults.
- Semantic tokens for colors, spacing, radius, typography, controls, and focus.
- Core controls including `Button`, `Badge`, `Checkbox`, `Radio`, `Switch`, `Slider`, `TextInput`, `SearchInput`, `Dropdown`, `Tabs`, and `SegmentedControl`.
- Surfaces and feedback including `Card`, `Panel`, `Alert`, `Tooltip`, `Popover`, `Dialog`, `Disclosure`, and `Accordion`.
- Data display primitives including `Table`, `TextTable`, expandable table details, list rows, and separators.
- A native gallery app with runtime light/dark switching, token editing, component examples, and reusable app patterns.

## Development

Check the workspace:

```sh
cargo check --workspace
```

Format the workspace:

```sh
cargo fmt --all
```

Run the gallery:

```sh
cargo run -p cast-gallery
```

## Direction

The first implementation milestone is the foundation: theme tokens, `egui` style integration, runtime theme switching, and a focused set of core components. Broader controls, overlays, data display, navigation, command surfaces, serialization, and Dashbase/Baseline theme import experiments should build on that foundation rather than define it.

## Patterns

Reusable composed UI blocks live in `crates/cast-gallery/src/patterns` while their API and visual language are still being proven. These are not low-level Cast widgets yet; they are examples of how primitives combine into product surfaces such as command palettes, shell chrome, related activity, and expandable entity tables.
