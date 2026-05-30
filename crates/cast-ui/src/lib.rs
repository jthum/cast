//! Cast is a standalone, themeable component library for `egui`.

mod color;
pub mod components;
pub mod foundation;
mod style;
pub mod theme;

pub use color::{contrast_ratio, mix_with_transparent};
pub use components::{
    Accordion, AccordionItem, Alert, Avatar, Badge, Button, Card, Checkbox, Combobox,
    ConfirmDialog, ConfirmDialogResponse, Dialog, DialogController, Disclosure, DisclosureResponse,
    Dropdown, EmptyState, FilterBar, FormActions, FormField, FormSection, Label, Link, ListRow,
    Loader, LoaderStyle, MenuItem, NavList, Notice, Panel, Popover, ProgressBar, Radio, RadioGroup,
    SearchInput, SegmentedControl, Select, Separator, Sheet, SheetController, Skeleton, Slider,
    Spinner, SpinnerStyle, Switch, Table, TableDetailRow, TableRow, Tabs, TextArea, TextInput,
    TextTable, Toast, ToastPlacement, ToastResponse, ToastStack, ToastStackMode,
    ToastStackResponse, Tooltip,
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
