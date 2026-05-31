//! Cast is a standalone, themeable component library for `egui`.

mod color;
pub mod components;
pub mod foundation;
mod style;
pub mod theme;

pub use color::{contrast_ratio, mix_with_transparent};
pub use components::{
    Accordion, AccordionItem, ActionRow, AgentComposer, AgentComposerResponse, Alert,
    ApprovalPanel, ApprovalPanelResponse, ArtifactCard, ArtifactCardResponse, Avatar, Badge,
    Button, Card, ChatMessage, ChatRole, Checkbox, CodeOutputPanel, Combobox, ConfirmDialog,
    ConfirmDialogResponse, DateInput, Dialog, DialogController, Disclosure, DisclosureResponse,
    Dropdown, EmptyState, FilterBar, FormActions, FormField, FormSection, Kbd, Label, Link,
    ListRow, Loader, LoaderStyle, MenuItem, MessageThread, MessageThreadUi, NavList, Notice,
    NumberInput, Panel, Popover, ProgressBar, Radio, RadioGroup, RunPhase, RunTimeline,
    RunTimelineItem, SearchInput, SegmentedControl, Select, Separator, Sheet, SheetController,
    Skeleton, Slider, Spinner, SpinnerStyle, SurfaceChrome, SurfaceSectionStyle, Switch, Table,
    TableDetailRow, TableRow, Tabs, TextArea, TextInput, TextTable, TimeInput, Toast,
    ToastPlacement, ToastResponse, ToastStack, ToastStackMode, ToastStackResponse, ToolCall,
    ToolCallBlock, ToolCallStatus, ToolOutput, ToolOutputKind, Tooltip, ValidationIssue,
    ValidationSummary,
};
pub use egui;
pub use foundation::{Intent, Orientation, Placement, Size, Variant};
pub use theme::{
    AnimationTokens, BadgeTokenOverrides, BadgeTokens, ButtonTokenOverrides, ButtonTokens,
    CastPaletteInput, CastTheme, ColorTokens, ComponentTokenOverrides, ComponentTokens,
    ControlTokens, ElevationTokens, FeedbackTokenOverrides, FeedbackTokens, FocusTokens, FontFace,
    FontPathStack, FontStack, FontStackBuilder, GoogleFontFamily, InputTokenOverrides, InputTokens,
    RadiusTokens, ScrollTokens, SemanticColorTokens, ShadowTokens, SpacingTokens, StrokeTokens,
    SurfaceSectionTokenOverrides, SurfaceSectionTokens, SurfaceTokenOverrides, SurfaceTokens,
    ThemeMode, ThemeSeed, TypographyTokens, apply_theme, install_cast_fonts, install_font_stack,
    install_inter_fonts, set_theme, theme_for_ui,
};
