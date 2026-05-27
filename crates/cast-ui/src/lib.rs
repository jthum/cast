//! Cast is a standalone, themeable component library for `egui`.

pub mod components;
pub mod foundation;
mod style;
pub mod theme;

pub use components::{
    Badge, Button, Card, Checkbox, Panel, SearchInput, Separator, Switch, TextInput,
};
pub use egui;
pub use foundation::{Intent, Orientation, Placement, Size, Variant};
pub use theme::{
    AnimationTokens, CastPaletteInput, CastTheme, ColorTokens, ControlTokens, ElevationTokens,
    FocusTokens, RadiusTokens, SpacingTokens, StrokeTokens, ThemeMode, TypographyTokens,
    apply_theme, set_theme, theme_for_ui,
};
