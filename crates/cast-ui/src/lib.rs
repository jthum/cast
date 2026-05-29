//! Cast is a standalone, themeable component library for `egui`.

mod color;
pub mod components;
pub mod foundation;
mod style;
pub mod theme;

pub use color::{contrast_ratio, mix_with_transparent};
pub use components::{
    Alert, Badge, Button, Card, Checkbox, Dropdown, FilterBar, Label, Link, ListRow, MenuItem,
    NavList, Notice, Panel, Radio, SearchInput, SegmentedControl, Separator, Slider, Switch, Table,
    TableRow, Tabs, TextInput, TextTable,
};
pub use egui;
pub use foundation::{Intent, Orientation, Placement, Size, Variant};
pub use theme::{
    AnimationTokens, BadgeTokenOverrides, BadgeTokens, ButtonTokenOverrides, ButtonTokens,
    CastPaletteInput, CastTheme, ColorTokens, ComponentTokenOverrides, ComponentTokens,
    ControlTokens, ElevationTokens, FeedbackTokenOverrides, FeedbackTokens, FocusTokens, FontFace,
    FontPathStack, FontStack, FontStackBuilder, GoogleFontFamily, InputTokenOverrides, InputTokens,
    RadiusTokens, ScrollTokens, SemanticColorTokens, SpacingTokens, StrokeTokens,
    SurfaceTokenOverrides, SurfaceTokens, ThemeMode, ThemeSeed, TypographyTokens, apply_theme,
    install_cast_fonts, install_font_stack, install_inter_fonts, set_theme, theme_for_ui,
};
