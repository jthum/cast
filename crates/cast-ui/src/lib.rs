//! Cast is a standalone, themeable component library for `egui`.

mod color;
pub mod components;
pub mod foundation;
mod style;
pub mod theme;

pub use color::contrast_ratio;
pub use components::{
    Alert, Badge, Button, Card, Checkbox, Label, Link, Notice, Panel, SearchInput, Separator,
    Switch, TextInput,
};
pub use egui;
pub use foundation::{Intent, Orientation, Placement, Size, Variant};
pub use theme::{
    AnimationTokens, BadgeTokens, ButtonTokens, CastPaletteInput, CastTheme, ColorTokens,
    ComponentTokens, ControlTokens, ElevationTokens, FeedbackTokens, FocusTokens, InputTokens,
    RadiusTokens, SemanticColorTokens, SpacingTokens, StrokeTokens, SurfaceTokens, ThemeMode,
    TypographyTokens, apply_theme, set_theme, theme_for_ui,
};
