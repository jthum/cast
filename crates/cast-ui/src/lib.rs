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
    AnimationTokens, BadgeTokenOverrides, BadgeTokens, ButtonTokenOverrides, ButtonTokens,
    CastPaletteInput, CastTheme, ColorTokens, ComponentTokenOverrides, ComponentTokens,
    ControlTokens, ElevationTokens, FeedbackTokenOverrides, FeedbackTokens, FocusTokens, FontFace,
    FontStack, GoogleFontFamily, InputTokenOverrides, InputTokens, RadiusTokens,
    SemanticColorTokens, SpacingTokens, StrokeTokens, SurfaceTokenOverrides, SurfaceTokens,
    ThemeMode, ThemeSeed, TypographyTokens, apply_theme, install_font_stack, install_inter_fonts,
    set_theme, theme_for_ui,
};
