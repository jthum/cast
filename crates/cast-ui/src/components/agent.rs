use egui::{Color32, InnerResponse, Response, RichText, Ui, Widget};

use crate::{
    color::mix_with_transparent,
    components::{Badge, Button, Loader, TextArea},
    foundation::{Intent, Size, Variant},
    theme::{CastTheme, ThemeMode, theme_for_ui},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ChatRole {
    User,
    #[default]
    Assistant,
    System,
    Tool,
}

#[derive(Clone, Debug)]
pub struct ChatMessage {
    role: ChatRole,
    title: String,
    body: String,
    metadata: Option<String>,
    intent: Intent,
    streaming: bool,
    width: Option<f32>,
}

impl ChatMessage {
    #[must_use]
    pub fn new(role: ChatRole, body: impl Into<String>) -> Self {
        Self {
            role,
            title: chat_role_label(role).to_owned(),
            body: body.into(),
            metadata: None,
            intent: chat_role_intent(role),
            streaming: false,
            width: None,
        }
    }

    #[must_use]
    pub fn user(body: impl Into<String>) -> Self {
        Self::new(ChatRole::User, body)
    }

    #[must_use]
    pub fn assistant(body: impl Into<String>) -> Self {
        Self::new(ChatRole::Assistant, body)
    }

    #[must_use]
    pub fn system(body: impl Into<String>) -> Self {
        Self::new(ChatRole::System, body)
    }

    #[must_use]
    pub fn tool(body: impl Into<String>) -> Self {
        Self::new(ChatRole::Tool, body)
    }

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    #[must_use]
    pub fn metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    #[must_use]
    pub fn streaming(mut self, streaming: bool) -> Self {
        self.streaming = streaming;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
}

impl Widget for ChatMessage {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let colors = chat_message_colors(&theme, self.role, self.intent);

        chat_message_frame(&theme, colors)
            .show(ui, |ui| {
                if let Some(width) = self.width {
                    let inner_width = frame_inner_width(width, theme.spacing.md);
                    ui.set_width(inner_width);
                    ui.set_max_width(inner_width);
                }

                ui.horizontal(|ui| {
                    paint_role_dot(ui, &theme, self.intent);
                    ui.label(
                        RichText::new(self.title)
                            .font(theme.typography.strong.clone())
                            .color(theme.colors.text)
                            .extra_letter_spacing(theme.typography.letter_spacing),
                    );
                    ui.add_space(theme.spacing.xs);
                    if let Some(metadata) = self.metadata {
                        ui.label(
                            RichText::new(metadata)
                                .font(theme.typography.caption.clone())
                                .color(theme.colors.text_subtle)
                                .extra_letter_spacing(theme.typography.letter_spacing),
                        );
                    }
                    if self.streaming {
                        ui.add_space(theme.spacing.xs);
                        ui.add(Loader::new().intent(Intent::Info).size(Size::Small));
                    }
                });
                ui.add_space(theme.spacing.xs);
                ui.add(
                    egui::Label::new(
                        RichText::new(self.body)
                            .font(theme.typography.body.clone())
                            .color(theme.colors.text)
                            .extra_letter_spacing(theme.typography.letter_spacing),
                    )
                    .wrap(),
                );
            })
            .response
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToolCallStatus {
    Queued,
    Running,
    #[default]
    Succeeded,
    Failed,
}

#[derive(Clone, Debug)]
pub struct ToolCall {
    name: String,
    status: ToolCallStatus,
    body: Option<String>,
    metadata: Option<String>,
    width: Option<f32>,
}

impl ToolCall {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: ToolCallStatus::Succeeded,
            body: None,
            metadata: None,
            width: None,
        }
    }

    #[must_use]
    pub fn status(mut self, status: ToolCallStatus) -> Self {
        self.status = status;
        self
    }

    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    #[must_use]
    pub fn metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
}

impl Widget for ToolCall {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let intent = tool_call_intent(self.status);

        egui::Frame::new()
            .fill(tool_call_fill(&theme))
            .stroke(egui::Stroke::new(
                theme.stroke.sm,
                tool_call_border(&theme, intent),
            ))
            .corner_radius(egui::CornerRadius::same(theme.radius.md as u8))
            .inner_margin(egui::Margin::symmetric(
                theme.spacing.md as i8,
                theme.spacing.sm as i8,
            ))
            .show(ui, |ui| {
                if let Some(width) = self.width {
                    let inner_width = frame_inner_width(width, theme.spacing.md);
                    ui.set_width(inner_width);
                    ui.set_max_width(inner_width);
                }

                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(self.name)
                            .font(theme.typography.button.clone())
                            .color(theme.colors.text)
                            .extra_letter_spacing(theme.typography.letter_spacing),
                    );
                    if let Some(metadata) = self.metadata {
                        ui.label(
                            RichText::new(metadata)
                                .font(theme.typography.caption.clone())
                                .color(theme.colors.text_subtle)
                                .extra_letter_spacing(theme.typography.letter_spacing),
                        );
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(
                            Badge::new(tool_call_status_label(self.status))
                                .intent(intent)
                                .status_dot(),
                        );
                    });
                });

                if let Some(body) = self.body {
                    ui.add_space(theme.spacing.xs);
                    ui.add(
                        egui::Label::new(
                            RichText::new(body)
                                .font(theme.typography.small.clone())
                                .color(theme.colors.text_muted)
                                .extra_letter_spacing(theme.typography.letter_spacing),
                        )
                        .wrap(),
                    );
                }
            })
            .response
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToolOutputKind {
    #[default]
    Text,
    Code,
    Json,
    Log,
    Error,
}

#[derive(Clone, Debug)]
pub struct ToolOutput {
    title: String,
    body: String,
    kind: ToolOutputKind,
    metadata: Option<String>,
    width: Option<f32>,
}

impl ToolOutput {
    #[must_use]
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            kind: ToolOutputKind::Text,
            metadata: None,
            width: None,
        }
    }

    #[must_use]
    pub fn kind(mut self, kind: ToolOutputKind) -> Self {
        self.kind = kind;
        self
    }

    #[must_use]
    pub fn metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
}

impl Widget for ToolOutput {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let intent = tool_output_intent(self.kind);

        egui::Frame::new()
            .fill(tool_output_fill(&theme, self.kind))
            .stroke(egui::Stroke::new(
                theme.stroke.sm,
                mix_with_transparent(intent_color(&theme, intent), 0.22),
            ))
            .corner_radius(egui::CornerRadius::same(theme.radius.md as u8))
            .inner_margin(egui::Margin::same(theme.spacing.md as i8))
            .show(ui, |ui| {
                if let Some(width) = self.width {
                    let inner_width = frame_inner_width(width, theme.spacing.md);
                    ui.set_width(inner_width);
                    ui.set_max_width(inner_width);
                }

                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(self.title)
                            .font(theme.typography.button.clone())
                            .color(theme.colors.text)
                            .extra_letter_spacing(theme.typography.letter_spacing),
                    );
                    if let Some(metadata) = self.metadata {
                        ui.label(
                            RichText::new(metadata)
                                .font(theme.typography.caption.clone())
                                .color(theme.colors.text_subtle)
                                .extra_letter_spacing(theme.typography.letter_spacing),
                        );
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(
                            Badge::new(tool_output_kind_label(self.kind))
                                .intent(intent)
                                .status_dot(),
                        );
                    });
                });
                ui.add_space(theme.spacing.xs);
                ui.add(
                    egui::Label::new(
                        RichText::new(self.body)
                            .font(tool_output_font(&theme, self.kind))
                            .color(tool_output_text_color(&theme, self.kind))
                            .extra_letter_spacing(theme.typography.letter_spacing),
                    )
                    .wrap(),
                );
            })
            .response
    }
}

#[derive(Debug)]
pub struct AgentComposer<'a> {
    text: &'a mut String,
    placeholder: String,
    send_label: String,
    secondary_label: Option<String>,
    rows: usize,
    enabled: bool,
    width: Option<f32>,
}

impl<'a> AgentComposer<'a> {
    #[must_use]
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            placeholder: "Ask the agent to do something...".to_owned(),
            send_label: "Send".to_owned(),
            secondary_label: None,
            rows: 3,
            enabled: true,
            width: None,
        }
    }

    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    #[must_use]
    pub fn send_label(mut self, label: impl Into<String>) -> Self {
        self.send_label = label.into();
        self
    }

    #[must_use]
    pub fn secondary_label(mut self, label: impl Into<String>) -> Self {
        self.secondary_label = Some(label.into());
        self
    }

    #[must_use]
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows.max(2);
        self
    }

    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn show(self, ui: &mut Ui) -> InnerResponse<AgentComposerResponse> {
        let theme = theme_for_ui(ui);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(260.0));
        let enabled = self.enabled;

        egui::Frame::new()
            .fill(theme.components.input.fill)
            .stroke(egui::Stroke::new(
                theme.components.input.border_width,
                theme.components.input.border,
            ))
            .corner_radius(egui::CornerRadius::same(
                theme.components.input.radius as u8,
            ))
            .inner_margin(egui::Margin::same(theme.spacing.sm as i8))
            .show(ui, |ui| {
                let inner_width = frame_inner_width(width, theme.spacing.sm);
                ui.set_width(inner_width);
                ui.set_max_width(inner_width);
                let edit = ui.add_enabled(
                    enabled,
                    TextArea::new(self.text)
                        .hint_text(self.placeholder)
                        .variant(Variant::Ghost)
                        .rows(self.rows)
                        .width(inner_width),
                );
                let has_text = !self.text.trim().is_empty();
                let submit_shortcut = enabled
                    && has_text
                    && edit.has_focus()
                    && ui.input(|input| {
                        input.key_pressed(egui::Key::Enter)
                            && (input.modifiers.command || input.modifiers.ctrl)
                    });

                ui.add_space(theme.spacing.xs);
                let buttons = ui.horizontal(|ui| {
                    if let Some(label) = self.secondary_label {
                        ui.add(Button::new(label).variant(Variant::Ghost).size(Size::Small));
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(
                            Button::new(self.send_label)
                                .leading_icon("[>]")
                                .enabled(enabled && has_text)
                                .size(Size::Small),
                        )
                    })
                    .inner
                });

                let button_response = buttons.inner;
                let submitted = submit_shortcut || button_response.clicked();

                AgentComposerResponse {
                    response: edit.union(button_response),
                    submitted,
                }
            })
    }
}

#[derive(Debug)]
pub struct AgentComposerResponse {
    pub response: Response,
    pub submitted: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ChatMessageColors {
    fill: Color32,
    border: Color32,
}

fn chat_message_frame(theme: &CastTheme, colors: ChatMessageColors) -> egui::Frame {
    egui::Frame::new()
        .fill(colors.fill)
        .stroke(egui::Stroke::new(theme.stroke.sm, colors.border))
        .corner_radius(egui::CornerRadius::same(theme.radius.lg as u8))
        .inner_margin(egui::Margin::same(theme.spacing.md as i8))
}

fn chat_message_colors(theme: &CastTheme, role: ChatRole, intent: Intent) -> ChatMessageColors {
    match role {
        ChatRole::User => ChatMessageColors {
            fill: mix_with_transparent(theme.colors.primary_family.base, 0.06),
            border: mix_with_transparent(theme.colors.primary_family.base, 0.28),
        },
        ChatRole::Assistant => ChatMessageColors {
            fill: theme.colors.surface,
            border: theme.colors.border,
        },
        ChatRole::System => ChatMessageColors {
            fill: theme.colors.surface_muted,
            border: theme.colors.border,
        },
        ChatRole::Tool => ChatMessageColors {
            fill: mix_with_transparent(intent_color(theme, intent), 0.04),
            border: mix_with_transparent(intent_color(theme, intent), 0.22),
        },
    }
}

fn paint_role_dot(ui: &mut Ui, theme: &CastTheme, intent: Intent) {
    let size = 9.0;
    let (rect, _response) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
    ui.painter()
        .circle_filled(rect.center(), size / 2.0, intent_color(theme, intent));
}

fn chat_role_label(role: ChatRole) -> &'static str {
    match role {
        ChatRole::User => "You",
        ChatRole::Assistant => "Assistant",
        ChatRole::System => "System",
        ChatRole::Tool => "Tool",
    }
}

fn chat_role_intent(role: ChatRole) -> Intent {
    match role {
        ChatRole::User => Intent::Primary,
        ChatRole::Assistant => Intent::Secondary,
        ChatRole::System => Intent::Neutral,
        ChatRole::Tool => Intent::Info,
    }
}

fn tool_call_intent(status: ToolCallStatus) -> Intent {
    match status {
        ToolCallStatus::Queued => Intent::Neutral,
        ToolCallStatus::Running => Intent::Info,
        ToolCallStatus::Succeeded => Intent::Success,
        ToolCallStatus::Failed => Intent::Danger,
    }
}

fn tool_call_status_label(status: ToolCallStatus) -> &'static str {
    match status {
        ToolCallStatus::Queued => "Queued",
        ToolCallStatus::Running => "Running",
        ToolCallStatus::Succeeded => "Done",
        ToolCallStatus::Failed => "Failed",
    }
}

fn frame_inner_width(outer_width: f32, horizontal_padding: f32) -> f32 {
    (outer_width - horizontal_padding * 2.0).max(80.0)
}

fn tool_output_intent(kind: ToolOutputKind) -> Intent {
    match kind {
        ToolOutputKind::Text | ToolOutputKind::Code | ToolOutputKind::Log => Intent::Neutral,
        ToolOutputKind::Json => Intent::Info,
        ToolOutputKind::Error => Intent::Danger,
    }
}

fn tool_output_kind_label(kind: ToolOutputKind) -> &'static str {
    match kind {
        ToolOutputKind::Text => "Text",
        ToolOutputKind::Code => "Code",
        ToolOutputKind::Json => "JSON",
        ToolOutputKind::Log => "Log",
        ToolOutputKind::Error => "Error",
    }
}

fn tool_output_fill(theme: &CastTheme, kind: ToolOutputKind) -> Color32 {
    if kind == ToolOutputKind::Error {
        return mix_with_transparent(theme.colors.danger_family.base, 0.04);
    }

    match theme.mode {
        ThemeMode::Light => theme.colors.surface_muted,
        ThemeMode::Dark => mix_with_transparent(theme.colors.text, 0.03),
    }
}

fn tool_output_font(theme: &CastTheme, kind: ToolOutputKind) -> egui::FontId {
    match kind {
        ToolOutputKind::Code
        | ToolOutputKind::Json
        | ToolOutputKind::Log
        | ToolOutputKind::Error => theme.typography.code.clone(),
        ToolOutputKind::Text => theme.typography.small.clone(),
    }
}

fn tool_output_text_color(theme: &CastTheme, kind: ToolOutputKind) -> Color32 {
    match kind {
        ToolOutputKind::Error => theme.colors.danger_family.emphasis,
        ToolOutputKind::Text => theme.colors.text_muted,
        ToolOutputKind::Code | ToolOutputKind::Json | ToolOutputKind::Log => theme.colors.text,
    }
}

fn tool_call_fill(theme: &CastTheme) -> Color32 {
    match theme.mode {
        ThemeMode::Light => theme.colors.surface,
        ThemeMode::Dark => mix_with_transparent(theme.colors.text, 0.02),
    }
}

fn tool_call_border(theme: &CastTheme, intent: Intent) -> Color32 {
    mix_with_transparent(intent_color(theme, intent), 0.24)
}

fn intent_color(theme: &CastTheme, intent: Intent) -> Color32 {
    match intent {
        Intent::Neutral => theme.colors.text_muted,
        Intent::Primary => theme.colors.primary_family.base,
        Intent::Secondary => theme.colors.secondary_family.base,
        Intent::Success => theme.colors.success_family.base,
        Intent::Warning => theme.colors.warning_family.base,
        Intent::Danger => theme.colors.danger_family.base,
        Intent::Info => theme.colors.info_family.base,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_message_defaults_follow_role() {
        let message = ChatMessage::user("Run the checks");

        assert_eq!(message.role, ChatRole::User);
        assert_eq!(message.title, "You");
        assert_eq!(message.intent, Intent::Primary);
        assert!(!message.streaming);
    }

    #[test]
    fn tool_call_status_maps_to_intent_and_label() {
        assert_eq!(tool_call_intent(ToolCallStatus::Running), Intent::Info);
        assert_eq!(tool_call_intent(ToolCallStatus::Failed), Intent::Danger);
        assert_eq!(tool_call_status_label(ToolCallStatus::Succeeded), "Done");
    }

    #[test]
    fn composer_requires_text_to_submit() {
        let mut text = String::from("  ");
        let composer = AgentComposer::new(&mut text).send_label("Run").rows(1);

        assert_eq!(composer.send_label, "Run");
        assert_eq!(composer.rows, 2);
        assert!(composer.text.trim().is_empty());
    }

    #[test]
    fn chat_tool_message_uses_tinted_tool_chrome() {
        let theme = CastTheme::light();
        let colors = chat_message_colors(&theme, ChatRole::Tool, Intent::Warning);

        assert_eq!(
            colors.fill,
            mix_with_transparent(theme.colors.warning_family.base, 0.04)
        );
        assert_eq!(
            colors.border,
            mix_with_transparent(theme.colors.warning_family.base, 0.22)
        );
    }

    #[test]
    fn tool_output_kind_sets_intent_and_label() {
        assert_eq!(tool_output_intent(ToolOutputKind::Json), Intent::Info);
        assert_eq!(tool_output_intent(ToolOutputKind::Error), Intent::Danger);
        assert_eq!(tool_output_kind_label(ToolOutputKind::Code), "Code");
    }

    #[test]
    fn streaming_message_records_state() {
        let message = ChatMessage::assistant("Working").streaming(true);

        assert!(message.streaming);
    }

    #[test]
    fn framed_width_accounts_for_inner_padding() {
        assert_eq!(frame_inner_width(320.0, 12.0), 296.0);
        assert_eq!(frame_inner_width(64.0, 12.0), 80.0);
    }
}
