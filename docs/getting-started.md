# Getting Started

Cast is published as the `cast-ui` crate, but application code should normally alias the package as `cast` so component call sites stay concise.

## Dependency

Use Cargo's package rename support:

```toml
[dependencies]
cast = { package = "cast-ui", version = "0.1" }
eframe = "0.34"
```

Inside this repository, the workspace dependency already uses the same alias:

```toml
[workspace.dependencies]
cast = { package = "cast-ui", path = "crates/cast-ui" }
```

That gives you `cast::Button`, `cast::TextInput`, and `cast::ThemeSeed` even though the published package remains `cast-ui`.

## App Setup

Install bundled fonts and set a theme before rendering widgets:

```rust
struct MyApp {
    theme_seed: cast::ThemeSeed,
    name: String,
}

impl MyApp {
    fn new(ctx: &egui::Context) -> Self {
        cast::install_cast_fonts(ctx);
        let theme_seed = cast::ThemeSeed::for_mode(cast::ThemeMode::Light);
        cast::set_theme(ctx, theme_seed.clone().resolve());
        Self {
            theme_seed,
            name: String::new(),
        }
    }
}
```

For `eframe`, do this in the app creator:

```rust
eframe::run_native(
    "My App",
    native_options,
    Box::new(|cc| Ok(Box::new(MyApp::new(&cc.egui_ctx)))),
)
```

## First Widgets

Cast widgets are normal `egui::Widget` values:

```rust
ui.add(cast::TextInput::new(&mut self.name).hint_text("Project name"));
ui.add(cast::Button::new("Create project"));
ui.add(cast::Badge::new("Ready").intent(cast::Intent::Success).status_dot());
```

Most components use builder methods for size, intent, variant, width, and state:

```rust
ui.add(
    cast::Button::new("Delete")
        .intent(cast::Intent::Danger)
        .variant(cast::Variant::Outline)
        .size(cast::Size::Small),
);
```

## Runtime Theme Changes

Themes are not compile-time configuration. Keep a `ThemeSeed`, update it in response to user preferences, resolve it, and install the resolved theme into the `egui` context:

```rust
self.theme_seed = self.theme_seed.clone().with_mode(cast::ThemeMode::Dark);
cast::set_theme(ctx, self.theme_seed.clone().resolve());
```

Palette values can also change at runtime:

```rust
self.theme_seed.palette.primary = egui::Color32::from_rgb(147, 51, 234);
cast::set_theme(ctx, self.theme_seed.clone().resolve());
```

Every Cast widget reads the current theme from the `egui` context, so changes show up without rebuilding the app.

## Fonts

`cast::install_cast_fonts(ctx)` installs bundled Inter and JetBrains Mono roles. Applications that ship their own fonts can build a role-based stack and install it:

```rust
let fonts = cast::FontStack::from_paths(
    cast::FontPathStack::new("assets/fonts/body.ttf")
        .with_heading("assets/fonts/heading.ttf")
        .with_mono("assets/fonts/mono.ttf"),
)?;

cast::install_font_stack(ctx, &fonts);
```

The exact builder APIs are still evolving, but the direction is stable: Cast treats body, controls, strong text, headings, and monospace as separate font roles.

## Gallery

Run the gallery while developing themes or components:

```sh
cargo run -p cast-gallery
```

The gallery is both a component catalogue and a product-screen test bed. It is the fastest way to see whether a token change works across light mode, dark mode, custom palettes, dense tables, forms, overlays, and agent-oriented screens.
