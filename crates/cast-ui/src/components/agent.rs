use egui::{Color32, InnerResponse, Response, RichText, TextEdit, Ui, Widget};

use crate::{
    color::mix_with_transparent,
    components::{Badge, Button, Loader, Markdown, Select},
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

    pub fn show_with_content(self, ui: &mut Ui, add_content: impl FnOnce(&mut Ui)) -> Response {
        show_chat_message_content(self, ui, Some(add_content))
    }
}

impl Widget for ChatMessage {
    fn ui(self, ui: &mut Ui) -> Response {
        show_chat_message_content(self, ui, None::<fn(&mut Ui)>)
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
                if self.kind == ToolOutputKind::Text {
                    ui.add(Markdown::new(self.body).width(ui.available_width()));
                } else {
                    ui.add(
                        egui::Label::new(
                            RichText::new(self.body)
                                .font(tool_output_font(&theme, self.kind))
                                .color(tool_output_text_color(&theme, self.kind))
                                .extra_letter_spacing(theme.typography.letter_spacing),
                        )
                        .wrap()
                        .selectable(true),
                    );
                }
            })
            .response
    }
}

#[derive(Debug)]
pub struct AgentComposer<'a> {
    text: &'a mut String,
    placeholder: String,
    send_label: String,
    stop_label: String,
    secondary_label: Option<String>,
    tool_label: Option<String>,
    model_label: Option<String>,
    model_options: Vec<String>,
    selected_model: Option<&'a mut usize>,
    rows: usize,
    enabled: bool,
    loading: bool,
    width: Option<f32>,
}

impl<'a> AgentComposer<'a> {
    #[must_use]
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            placeholder: "Ask the agent to do something...".to_owned(),
            send_label: "Send".to_owned(),
            stop_label: "Stop".to_owned(),
            secondary_label: None,
            tool_label: None,
            model_label: None,
            model_options: Vec::new(),
            selected_model: None,
            rows: 3,
            enabled: true,
            loading: false,
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
    pub fn stop_label(mut self, label: impl Into<String>) -> Self {
        self.stop_label = label.into();
        self
    }

    #[must_use]
    pub fn secondary_label(mut self, label: impl Into<String>) -> Self {
        self.secondary_label = Some(label.into());
        self
    }

    #[must_use]
    pub fn attachment_label(self, label: impl Into<String>) -> Self {
        self.secondary_label(label)
    }

    #[must_use]
    pub fn tool_label(mut self, label: impl Into<String>) -> Self {
        self.tool_label = Some(label.into());
        self
    }

    #[must_use]
    pub fn model_selector<I, O>(mut self, selected_model: &'a mut usize, options: I) -> Self
    where
        I: IntoIterator<Item = O>,
        O: Into<String>,
    {
        self.selected_model = Some(selected_model);
        self.model_options = options.into_iter().map(Into::into).collect();
        self
    }

    #[must_use]
    pub fn model_label(mut self, label: impl Into<String>) -> Self {
        self.model_label = Some(label.into());
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
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
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
        let loading = self.loading;
        let send_label = self.send_label;
        let stop_label = self.stop_label;
        let secondary_label = self.secondary_label;
        let tool_label = self.tool_label;
        let model_label = self.model_label.unwrap_or_else(|| "Model".to_owned());
        let model_options = self.model_options;
        let selected_model = self.selected_model;

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
                let edit_widget = TextEdit::multiline(self.text)
                    .frame(crate::style::input_frame(&theme, Variant::Ghost))
                    .font(theme.typography.body.clone())
                    .desired_rows(self.rows)
                    .desired_width(inner_width)
                    .lock_focus(true)
                    .hint_text(
                        RichText::new(self.placeholder)
                            .font(theme.typography.body.clone())
                            .color(theme.components.input.placeholder)
                            .extra_letter_spacing(theme.typography.letter_spacing),
                    );
                let edit = ui.add_enabled(enabled, edit_widget);
                let has_text = !self.text.trim().is_empty();
                let submit_shortcut = enabled
                    && has_text
                    && !loading
                    && edit.has_focus()
                    && ui.input(|input| {
                        input.key_pressed(egui::Key::Enter)
                            && !input.modifiers.shift
                            && !input.modifiers.command
                            && !input.modifiers.ctrl
                    });
                if submit_shortcut {
                    trim_submitted_newline(self.text);
                }

                ui.add_space(theme.spacing.xs);
                let buttons = ui.horizontal_centered(|ui| {
                    ui.spacing_mut().item_spacing.x = theme.spacing.sm;
                    let mut attachment_clicked = false;
                    let mut tool_clicked = false;
                    let mut stopped = false;
                    let mut model_changed = false;

                    if let Some(label) = secondary_label {
                        attachment_clicked = ui
                            .add(Button::new(label).variant(Variant::Ghost).size(Size::Small))
                            .clicked();
                    }
                    if let Some(label) = tool_label {
                        tool_clicked = ui
                            .add(Button::new(label).variant(Variant::Ghost).size(Size::Small))
                            .clicked();
                    }
                    if let Some(selected) = selected_model {
                        if !model_options.is_empty() {
                            ui.label(
                                RichText::new(model_label)
                                    .font(theme.typography.caption.clone())
                                    .color(theme.colors.text_muted)
                                    .extra_letter_spacing(theme.typography.letter_spacing),
                            );
                            let response = ui.add(
                                Select::new(selected, model_options)
                                    .size(Size::Small)
                                    .width((inner_width * 0.34).clamp(132.0, 188.0)),
                            );
                            model_changed = response.changed();
                        }
                    }
                    let response = ui
                        .with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if loading {
                                let response = ui.add(
                                    Button::new(stop_label)
                                        .leading_icon("■")
                                        .intent(Intent::Danger)
                                        .variant(Variant::Outline)
                                        .size(Size::Small),
                                );
                                stopped = response.clicked();
                                response
                            } else {
                                ui.add(
                                    Button::new(send_label)
                                        .leading_icon("▶")
                                        .enabled(enabled && has_text)
                                        .size(Size::Small),
                                )
                            }
                        })
                        .inner;

                    AgentComposerActionState {
                        response,
                        attachment_clicked,
                        tool_clicked,
                        stopped,
                        model_changed,
                    }
                });

                let actions = buttons.inner;
                let submitted = submit_shortcut || actions.response.clicked();

                AgentComposerResponse {
                    response: edit.union(actions.response),
                    submitted,
                    stopped: actions.stopped,
                    attachment_clicked: actions.attachment_clicked,
                    tool_clicked: actions.tool_clicked,
                    model_changed: actions.model_changed,
                }
            })
    }
}

#[derive(Debug)]
struct AgentComposerActionState {
    response: Response,
    attachment_clicked: bool,
    tool_clicked: bool,
    stopped: bool,
    model_changed: bool,
}

#[derive(Debug)]
pub struct AgentComposerResponse {
    pub response: Response,
    pub submitted: bool,
    pub stopped: bool,
    pub attachment_clicked: bool,
    pub tool_clicked: bool,
    pub model_changed: bool,
}

#[derive(Clone, Debug)]
pub struct MessageThread {
    width: Option<f32>,
    compact: bool,
}

impl MessageThread {
    #[must_use]
    pub fn new() -> Self {
        Self {
            width: None,
            compact: false,
        }
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    #[must_use]
    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_messages: impl FnOnce(&mut MessageThreadUi<'_>) -> R,
    ) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(260.0));
        egui::Frame::new()
            .fill(theme.colors.surface)
            .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
            .corner_radius(egui::CornerRadius::same(theme.radius.lg as u8))
            .inner_margin(egui::Margin::same(theme.spacing.md as i8))
            .show(ui, |ui| {
                ui.set_width(frame_inner_width(width, theme.spacing.md));
                let mut thread = MessageThreadUi {
                    ui,
                    spacing: if self.compact {
                        theme.spacing.sm
                    } else {
                        theme.spacing.md
                    },
                    count: 0,
                };
                add_messages(&mut thread)
            })
    }
}

impl Default for MessageThread {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MessageThreadUi<'a> {
    ui: &'a mut Ui,
    spacing: f32,
    count: usize,
}

impl MessageThreadUi<'_> {
    pub fn message(&mut self, message: ChatMessage) -> Response {
        self.add_gap();
        let width = self.ui.available_width();
        let response = self.ui.add(message.width(width));
        self.count += 1;
        response
    }

    pub fn rich_message(
        &mut self,
        message: ChatMessage,
        add_content: impl FnOnce(&mut Ui),
    ) -> Response {
        self.add_gap();
        let width = self.ui.available_width();
        let response = message.width(width).show_with_content(self.ui, add_content);
        self.count += 1;
        response
    }

    fn add_gap(&mut self) {
        if self.count > 0 {
            self.ui.add_space(self.spacing);
        }
    }
}

#[derive(Debug)]
pub struct ToolCallBlock<'a> {
    name: String,
    status: ToolCallStatus,
    arguments: Option<String>,
    elapsed: Option<String>,
    preview: Option<String>,
    open: &'a mut bool,
    width: Option<f32>,
}

impl<'a> ToolCallBlock<'a> {
    #[must_use]
    pub fn new(name: impl Into<String>, open: &'a mut bool) -> Self {
        Self {
            name: name.into(),
            status: ToolCallStatus::Queued,
            arguments: None,
            elapsed: None,
            preview: None,
            open,
            width: None,
        }
    }

    #[must_use]
    pub fn status(mut self, status: ToolCallStatus) -> Self {
        self.status = status;
        self
    }

    #[must_use]
    pub fn arguments(mut self, arguments: impl Into<String>) -> Self {
        self.arguments = Some(arguments.into());
        self
    }

    #[must_use]
    pub fn elapsed(mut self, elapsed: impl Into<String>) -> Self {
        self.elapsed = Some(elapsed.into());
        self
    }

    #[must_use]
    pub fn preview(mut self, preview: impl Into<String>) -> Self {
        self.preview = Some(preview.into());
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
}

impl Widget for ToolCallBlock<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let intent = tool_call_intent(self.status);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(260.0));

        egui::Frame::new()
            .fill(tool_call_fill(&theme))
            .stroke(egui::Stroke::new(
                theme.stroke.sm,
                tool_call_border(&theme, intent),
            ))
            .corner_radius(egui::CornerRadius::same(theme.radius.md as u8))
            .inner_margin(egui::Margin::same(theme.spacing.sm as i8))
            .show(ui, |ui| {
                ui.set_width(frame_inner_width(width, theme.spacing.sm));
                let header = ui
                    .horizontal(|ui| {
                        let caret = if *self.open { "v" } else { ">" };
                        ui.label(
                            RichText::new(caret)
                                .font(theme.typography.button.clone())
                                .color(theme.colors.text_muted),
                        );
                        ui.label(
                            RichText::new(self.name.clone())
                                .font(theme.typography.button.clone())
                                .color(theme.colors.text)
                                .extra_letter_spacing(theme.typography.letter_spacing),
                        );
                        if let Some(elapsed) = &self.elapsed {
                            ui.label(
                                RichText::new(elapsed)
                                    .font(theme.typography.caption.clone())
                                    .color(theme.colors.text_subtle),
                            );
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add(
                                Badge::new(tool_call_status_label(self.status))
                                    .intent(intent)
                                    .status_dot(),
                            );
                        });
                    })
                    .response
                    .interact(egui::Sense::click());
                if header.clicked() {
                    *self.open = !*self.open;
                }
                if let Some(arguments) = &self.arguments {
                    ui.add_space(theme.spacing.xs);
                    ui.label(
                        RichText::new(arguments)
                            .font(theme.typography.small.clone())
                            .color(theme.colors.text_muted),
                    );
                }
                if *self.open {
                    if let Some(preview) = &self.preview {
                        ui.add_space(theme.spacing.sm);
                        paint_code_region(ui, &theme, preview, ToolOutputKind::Log, true, 128.0);
                    }
                }
                header
            })
            .inner
    }
}

#[derive(Clone, Debug)]
pub struct RunTimeline {
    items: Vec<RunTimelineItem>,
    width: Option<f32>,
}

impl RunTimeline {
    #[must_use]
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            width: None,
        }
    }

    #[must_use]
    pub fn item(mut self, item: RunTimelineItem) -> Self {
        self.items.push(item);
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
}

impl Default for RunTimeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for RunTimeline {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(260.0));
        egui::Frame::new()
            .fill(theme.colors.surface)
            .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
            .corner_radius(egui::CornerRadius::same(theme.radius.lg as u8))
            .inner_margin(egui::Margin::same(theme.spacing.md as i8))
            .show(ui, |ui| {
                ui.set_width(frame_inner_width(width, theme.spacing.md));
                let mut combined: Option<Response> = None;
                let mut rows = Vec::with_capacity(self.items.len());

                for item in &self.items {
                    let row = allocate_timeline_item(ui, item);
                    combined = Some(match combined {
                        Some(existing) => existing.union(row.response.clone()),
                        None => row.response.clone(),
                    });
                    rows.push(row);
                }

                paint_timeline_guide(ui, &theme, &rows);
                for (row, item) in rows.iter().zip(&self.items) {
                    paint_timeline_item(ui, &theme, row.rect, item);
                }

                combined
                    .unwrap_or_else(|| ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover()))
            })
            .inner
    }
}

#[derive(Clone, Debug)]
pub struct RunTimelineItem {
    title: String,
    detail: Option<String>,
    phase: RunPhase,
    status: ToolCallStatus,
    metadata: Option<String>,
}

impl RunTimelineItem {
    #[must_use]
    pub fn new(phase: RunPhase, title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            detail: None,
            phase,
            status: ToolCallStatus::Succeeded,
            metadata: None,
        }
    }

    #[must_use]
    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    #[must_use]
    pub fn status(mut self, status: ToolCallStatus) -> Self {
        self.status = status;
        self
    }

    #[must_use]
    pub fn metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RunPhase {
    #[default]
    Planning,
    ToolCall,
    Patch,
    Test,
    Review,
    FinalResponse,
}

#[derive(Clone, Debug)]
pub struct CodeOutputPanel {
    title: String,
    body: String,
    kind: ToolOutputKind,
    metadata: Option<String>,
    wrap: bool,
    copy: bool,
    width: Option<f32>,
    height: f32,
}

impl CodeOutputPanel {
    #[must_use]
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            kind: ToolOutputKind::Log,
            metadata: None,
            wrap: true,
            copy: true,
            width: None,
            height: 180.0,
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
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    #[must_use]
    pub fn copy(mut self, copy: bool) -> Self {
        self.copy = copy;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(64.0);
        self
    }
}

impl Widget for CodeOutputPanel {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(260.0));
        egui::Frame::new()
            .fill(tool_output_fill(&theme, self.kind))
            .stroke(egui::Stroke::new(
                theme.stroke.sm,
                mix_with_transparent(intent_color(&theme, tool_output_intent(self.kind)), 0.24),
            ))
            .corner_radius(egui::CornerRadius::same(theme.radius.md as u8))
            .inner_margin(egui::Margin::same(theme.spacing.md as i8))
            .show(ui, |ui| {
                ui.set_width(frame_inner_width(width, theme.spacing.md));
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(self.title.clone())
                            .font(theme.typography.button.clone())
                            .color(theme.colors.text),
                    );
                    if let Some(metadata) = &self.metadata {
                        ui.label(
                            RichText::new(metadata)
                                .font(theme.typography.caption.clone())
                                .color(theme.colors.text_subtle),
                        );
                    }
                    if self.copy {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .add(
                                    Button::new("Copy")
                                        .variant(Variant::Ghost)
                                        .size(Size::Small),
                                )
                                .clicked()
                            {
                                ui.ctx().copy_text(self.body.clone());
                            }
                        });
                    }
                });
                ui.add_space(theme.spacing.xs);
                paint_code_region(ui, &theme, &self.body, self.kind, self.wrap, self.height)
            })
            .inner
    }
}

#[derive(Clone, Debug)]
pub struct ContextPanel {
    title: String,
    used: usize,
    capacity: usize,
    window_count: Option<usize>,
    auto_compact_at: Option<usize>,
    items: Vec<ContextItem>,
    width: Option<f32>,
}

impl ContextPanel {
    #[must_use]
    pub fn new(used: usize, capacity: usize) -> Self {
        Self {
            title: "Context".to_owned(),
            used,
            capacity: capacity.max(1),
            window_count: None,
            auto_compact_at: None,
            items: Vec::new(),
            width: None,
        }
    }

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    #[must_use]
    pub fn window_count(mut self, count: usize) -> Self {
        self.window_count = Some(count);
        self
    }

    #[must_use]
    pub fn auto_compact_at(mut self, count: usize) -> Self {
        self.auto_compact_at = Some(count);
        self
    }

    #[must_use]
    pub fn item(mut self, item: ContextItem) -> Self {
        self.items.push(item);
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(260.0));

        egui::Frame::new()
            .fill(theme.colors.surface)
            .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
            .corner_radius(egui::CornerRadius::same(theme.radius.lg as u8))
            .inner_margin(egui::Margin::same(theme.spacing.md as i8))
            .show(ui, |ui| {
                ui.set_width(frame_inner_width(width, theme.spacing.md));
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = theme.spacing.xs;
                    ui.label(
                        RichText::new(self.title)
                            .font(theme.typography.heading_sm.clone())
                            .color(theme.colors.text),
                    );
                    ui.add(
                        Badge::new(format!(
                            "{} / {}",
                            compact_number(self.used),
                            compact_number(self.capacity)
                        ))
                        .intent(context_usage_intent(self.used, self.capacity))
                        .status_dot(),
                    );
                });
                ui.add_space(theme.spacing.sm);
                paint_context_meter(ui, &theme, self.used, self.capacity);
                if !self.items.is_empty() {
                    ui.add_space(theme.spacing.md);
                    for item in self.items {
                        context_item_ui(ui, &theme, item);
                    }
                }
                ui.add_space(theme.spacing.sm);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = theme.spacing.sm;
                    if let Some(count) = self.window_count {
                        ui.label(
                            RichText::new(format!("{count} in window"))
                                .font(theme.typography.small.clone())
                                .color(theme.colors.text_muted),
                        );
                    }
                    if let Some(count) = self.auto_compact_at {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("Auto-compacts at {count}"))
                                    .font(theme.typography.small.clone())
                                    .color(theme.colors.text_subtle),
                            );
                        });
                    }
                });
            })
            .response
    }
}

#[derive(Clone, Debug)]
pub struct ContextItem {
    label: String,
    detail: String,
    tokens: Option<usize>,
    intent: Intent,
}

impl ContextItem {
    #[must_use]
    pub fn new(label: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            detail: detail.into(),
            tokens: None,
            intent: Intent::Neutral,
        }
    }

    #[must_use]
    pub fn tokens(mut self, tokens: usize) -> Self {
        self.tokens = Some(tokens);
        self
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }
}

#[derive(Clone, Debug)]
pub struct ArtifactCard {
    title: String,
    description: Option<String>,
    kind: String,
    metadata: Option<String>,
    intent: Intent,
    width: Option<f32>,
}

impl ArtifactCard {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            kind: "Artifact".to_owned(),
            metadata: None,
            intent: Intent::Neutral,
            width: None,
        }
    }

    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn kind(mut self, kind: impl Into<String>) -> Self {
        self.kind = kind.into();
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
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn show(self, ui: &mut Ui) -> InnerResponse<ArtifactCardResponse> {
        let theme = theme_for_ui(ui);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(260.0));
        egui::Frame::new()
            .fill(theme.colors.surface)
            .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
            .corner_radius(egui::CornerRadius::same(theme.radius.lg as u8))
            .inner_margin(egui::Margin::same(theme.spacing.md as i8))
            .show(ui, |ui| {
                ui.set_width(frame_inner_width(width, theme.spacing.md));
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = theme.spacing.xs;
                    ui.add(Badge::new(self.kind).intent(self.intent).status_dot());
                    if let Some(metadata) = &self.metadata {
                        ui.label(
                            RichText::new(metadata)
                                .font(theme.typography.caption.clone())
                                .color(theme.colors.text_subtle),
                        );
                    }
                });
                ui.add_space(theme.spacing.xs);
                ui.label(
                    RichText::new(self.title)
                        .font(theme.typography.body_strong.clone())
                        .color(theme.colors.text),
                );
                if let Some(description) = self.description {
                    ui.label(
                        RichText::new(description)
                            .font(theme.typography.small.clone())
                            .color(theme.colors.text_muted),
                    );
                }
                ui.add_space(theme.spacing.sm);
                let mut result = ArtifactCardResponse::default();
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(theme.spacing.sm, theme.spacing.xs);
                    result.opened = ui.add(Button::new("Open").size(Size::Small)).clicked();
                    result.copied = ui
                        .add(
                            Button::new("Copy")
                                .variant(Variant::Outline)
                                .size(Size::Small),
                        )
                        .clicked();
                    result.approved = ui
                        .add(
                            Button::new("Approve")
                                .intent(Intent::Success)
                                .size(Size::Small),
                        )
                        .clicked();
                    result.rejected = ui
                        .add(
                            Button::new("Reject")
                                .intent(Intent::Danger)
                                .variant(Variant::Outline)
                                .size(Size::Small),
                        )
                        .clicked();
                });
                result
            })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ArtifactCardResponse {
    pub opened: bool,
    pub copied: bool,
    pub approved: bool,
    pub rejected: bool,
}

#[derive(Clone, Debug)]
pub struct ApprovalPanel {
    title: String,
    impact: String,
    risk: Option<String>,
    primary_label: String,
    secondary_label: String,
    intent: Intent,
    width: Option<f32>,
}

impl ApprovalPanel {
    #[must_use]
    pub fn new(title: impl Into<String>, impact: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            impact: impact.into(),
            risk: None,
            primary_label: "Approve".to_owned(),
            secondary_label: "Cancel".to_owned(),
            intent: Intent::Primary,
            width: None,
        }
    }

    #[must_use]
    pub fn risk(mut self, risk: impl Into<String>) -> Self {
        self.risk = Some(risk.into());
        self
    }

    #[must_use]
    pub fn primary_label(mut self, label: impl Into<String>) -> Self {
        self.primary_label = label.into();
        self
    }

    #[must_use]
    pub fn secondary_label(mut self, label: impl Into<String>) -> Self {
        self.secondary_label = label.into();
        self
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn show(self, ui: &mut Ui) -> InnerResponse<ApprovalPanelResponse> {
        let theme = theme_for_ui(ui);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(260.0));
        egui::Frame::new()
            .fill(mix_with_transparent(
                intent_color(&theme, self.intent),
                0.04,
            ))
            .stroke(egui::Stroke::new(
                theme.stroke.sm,
                mix_with_transparent(intent_color(&theme, self.intent), 0.25),
            ))
            .corner_radius(egui::CornerRadius::same(theme.radius.lg as u8))
            .inner_margin(egui::Margin::same(theme.spacing.md as i8))
            .show(ui, |ui| {
                ui.set_width(frame_inner_width(width, theme.spacing.md));
                ui.add(
                    Badge::new("Approval required")
                        .intent(self.intent)
                        .status_dot(),
                );
                ui.add_space(theme.spacing.xs);
                ui.label(
                    RichText::new(self.title)
                        .font(theme.typography.body_strong.clone())
                        .color(theme.colors.text),
                );
                ui.label(
                    RichText::new(self.impact)
                        .font(theme.typography.small.clone())
                        .color(theme.colors.text_muted),
                );
                if let Some(risk) = self.risk {
                    ui.add_space(theme.spacing.xs);
                    ui.label(
                        RichText::new(risk)
                            .font(theme.typography.small.clone())
                            .color(intent_color(&theme, Intent::Warning)),
                    );
                }
                ui.add_space(theme.spacing.sm);
                let mut result = ApprovalPanelResponse::default();
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(theme.spacing.sm, theme.spacing.xs);
                    result.approved = ui
                        .add(
                            Button::new(self.primary_label)
                                .intent(self.intent)
                                .size(Size::Small),
                        )
                        .clicked();
                    result.cancelled = ui
                        .add(
                            Button::new(self.secondary_label)
                                .variant(Variant::Outline)
                                .size(Size::Small),
                        )
                        .clicked();
                });
                result
            })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ApprovalPanelResponse {
    pub approved: bool,
    pub cancelled: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PlanStepStatus {
    #[default]
    Pending,
    Active,
    Done,
    Blocked,
}

#[derive(Clone, Debug)]
pub struct PlanStep {
    title: String,
    detail: Option<String>,
    status: PlanStepStatus,
    depth: usize,
}

impl PlanStep {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            detail: None,
            status: PlanStepStatus::Pending,
            depth: 0,
        }
    }

    #[must_use]
    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    #[must_use]
    pub fn status(mut self, status: PlanStepStatus) -> Self {
        self.status = status;
        self
    }

    #[must_use]
    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = depth.min(3);
        self
    }
}

#[derive(Clone, Debug)]
pub struct PlanList {
    title: String,
    summary: Option<String>,
    steps: Vec<PlanStep>,
    width: Option<f32>,
}

impl PlanList {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            summary: None,
            steps: Vec::new(),
            width: None,
        }
    }

    #[must_use]
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    #[must_use]
    pub fn step(mut self, step: PlanStep) -> Self {
        self.steps.push(step);
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
}

impl Widget for PlanList {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(260.0));

        egui::Frame::new()
            .fill(theme.colors.surface)
            .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
            .corner_radius(egui::CornerRadius::same(theme.radius.lg as u8))
            .inner_margin(egui::Margin::same(theme.spacing.md as i8))
            .show(ui, |ui| {
                ui.set_width(frame_inner_width(width, theme.spacing.md));
                ui.label(
                    RichText::new(self.title)
                        .font(theme.typography.body_strong.clone())
                        .color(theme.colors.text),
                );
                if let Some(summary) = self.summary {
                    ui.label(
                        RichText::new(summary)
                            .font(theme.typography.small.clone())
                            .color(theme.colors.text_muted),
                    );
                }
                ui.add_space(theme.spacing.sm);
                let mut combined: Option<Response> = None;
                for (index, step) in self.steps.iter().enumerate() {
                    let response = plan_step_ui(ui, &theme, step, index + 1);
                    combined = Some(match combined {
                        Some(existing) => existing.union(response),
                        None => response,
                    });
                }
                combined
                    .unwrap_or_else(|| ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover()))
            })
            .inner
    }
}

#[derive(Clone, Debug)]
pub struct PatchFile {
    path: String,
    additions: usize,
    deletions: usize,
    status: ToolCallStatus,
}

impl PatchFile {
    #[must_use]
    pub fn new(path: impl Into<String>, additions: usize, deletions: usize) -> Self {
        Self {
            path: path.into(),
            additions,
            deletions,
            status: ToolCallStatus::Succeeded,
        }
    }

    #[must_use]
    pub fn status(mut self, status: ToolCallStatus) -> Self {
        self.status = status;
        self
    }
}

#[derive(Clone, Debug)]
pub struct PatchReviewPanel {
    title: String,
    summary: String,
    files: Vec<PatchFile>,
    tests: Option<String>,
    risk: Option<String>,
    width: Option<f32>,
}

impl PatchReviewPanel {
    #[must_use]
    pub fn new(title: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            summary: summary.into(),
            files: Vec::new(),
            tests: None,
            risk: None,
            width: None,
        }
    }

    #[must_use]
    pub fn file(mut self, file: PatchFile) -> Self {
        self.files.push(file);
        self
    }

    #[must_use]
    pub fn tests(mut self, tests: impl Into<String>) -> Self {
        self.tests = Some(tests.into());
        self
    }

    #[must_use]
    pub fn risk(mut self, risk: impl Into<String>) -> Self {
        self.risk = Some(risk.into());
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn show(self, ui: &mut Ui) -> InnerResponse<PatchReviewResponse> {
        let theme = theme_for_ui(ui);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(260.0));

        egui::Frame::new()
            .fill(theme.colors.surface)
            .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
            .corner_radius(egui::CornerRadius::same(theme.radius.lg as u8))
            .inner_margin(egui::Margin::same(theme.spacing.md as i8))
            .show(ui, |ui| {
                ui.set_width(frame_inner_width(width, theme.spacing.md));
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(self.title)
                            .font(theme.typography.body_strong.clone())
                            .color(theme.colors.text),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(
                            Badge::new(format!("{} files", self.files.len()))
                                .intent(Intent::Info)
                                .status_dot(),
                        );
                    });
                });
                ui.label(
                    RichText::new(self.summary)
                        .font(theme.typography.small.clone())
                        .color(theme.colors.text_muted),
                );
                ui.add_space(theme.spacing.sm);
                for file in self.files {
                    patch_file_ui(ui, &theme, file);
                }
                if let Some(tests) = self.tests {
                    ui.add_space(theme.spacing.sm);
                    ui.add(Badge::new(tests).intent(Intent::Success).status_dot());
                }
                if let Some(risk) = self.risk {
                    ui.add_space(theme.spacing.xs);
                    ui.label(
                        RichText::new(risk)
                            .font(theme.typography.small.clone())
                            .color(intent_color(&theme, Intent::Warning)),
                    );
                }
                ui.add_space(theme.spacing.sm);
                let mut result = PatchReviewResponse::default();
                ui.horizontal_wrapped(|ui| {
                    result.approved = ui
                        .add(Button::new("Approve patch").size(Size::Small))
                        .clicked();
                    result.rejected = ui
                        .add(
                            Button::new("Request changes")
                                .intent(Intent::Danger)
                                .variant(Variant::Outline)
                                .size(Size::Small),
                        )
                        .clicked();
                    result.opened = ui
                        .add(
                            Button::new("Open diff")
                                .variant(Variant::Ghost)
                                .size(Size::Small),
                        )
                        .clicked();
                });
                result
            })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PatchReviewResponse {
    pub approved: bool,
    pub rejected: bool,
    pub opened: bool,
}

fn paint_context_meter(ui: &mut Ui, theme: &CastTheme, used: usize, capacity: usize) {
    let width = ui.available_width();
    let height = theme.spacing.sm.max(8.0);
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }

    let radius = egui::CornerRadius::same((height / 2.0).round() as u8);
    let percent = (used as f32 / capacity.max(1) as f32).clamp(0.0, 1.0);
    let fill_rect = egui::Rect::from_min_max(
        rect.min,
        egui::pos2(rect.min.x + rect.width() * percent, rect.max.y),
    );
    let intent = context_usage_intent(used, capacity);

    ui.painter()
        .rect_filled(rect, radius, theme.colors.surface_muted);
    ui.painter()
        .rect_filled(fill_rect, radius, intent_color(theme, intent));
}

fn context_usage_intent(used: usize, capacity: usize) -> Intent {
    let ratio = used as f32 / capacity.max(1) as f32;
    if ratio >= 0.86 {
        Intent::Danger
    } else if ratio >= 0.68 {
        Intent::Warning
    } else {
        Intent::Info
    }
}

fn context_item_ui(ui: &mut Ui, theme: &CastTheme, item: ContextItem) {
    let response = egui::Frame::new()
        .fill(theme.colors.surface_raised)
        .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
        .corner_radius(egui::CornerRadius::same(theme.radius.md as u8))
        .inner_margin(egui::Margin::symmetric(
            theme.spacing.sm as i8,
            theme.spacing.xs as i8,
        ))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = theme.spacing.xs;
                ui.add(Badge::new(item.label).intent(item.intent).status_dot());
                ui.label(
                    RichText::new(item.detail)
                        .font(theme.typography.small.clone())
                        .color(theme.colors.text_muted),
                );
                if let Some(tokens) = item.tokens {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(compact_number(tokens))
                                .font(theme.typography.caption.clone())
                                .color(theme.colors.text_subtle),
                        );
                    });
                }
            });
        });
    ui.add_space(theme.spacing.xs);
    response.response.on_hover_cursor(egui::CursorIcon::Default);
}

fn compact_number(value: usize) -> String {
    if value >= 1_000_000 {
        format!("{}m", value / 1_000_000)
    } else if value >= 1_000 {
        format!("{}k", value / 1_000)
    } else {
        value.to_string()
    }
}

fn plan_step_ui(ui: &mut Ui, theme: &CastTheme, step: &PlanStep, index: usize) -> Response {
    let row_height = if step.detail.is_some() { 52.0 } else { 36.0 };
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::hover(),
    );

    if ui.is_rect_visible(rect) {
        let indent = step.depth as f32 * theme.spacing.lg;
        let content_x = rect.min.x + indent;
        let badge_rect = egui::Rect::from_min_size(
            egui::pos2(content_x, rect.min.y + 6.0),
            egui::vec2(24.0, 22.0),
        );
        ui.painter().rect(
            badge_rect,
            egui::CornerRadius::same(theme.radius.sm as u8),
            mix_with_transparent(intent_color(theme, plan_status_intent(step.status)), 0.08),
            egui::Stroke::new(
                theme.stroke.sm,
                mix_with_transparent(intent_color(theme, plan_status_intent(step.status)), 0.28),
            ),
            egui::StrokeKind::Outside,
        );
        ui.painter().text(
            badge_rect.center(),
            egui::Align2::CENTER_CENTER,
            index.to_string(),
            theme.typography.caption.clone(),
            intent_color(theme, plan_status_intent(step.status)),
        );
        let text_x = badge_rect.max.x + theme.spacing.sm;
        ui.painter().text(
            egui::pos2(text_x, rect.min.y + 6.0),
            egui::Align2::LEFT_TOP,
            step.title.as_str(),
            theme.typography.button.clone(),
            theme.colors.text,
        );
        ui.painter().text(
            egui::pos2(rect.max.x, rect.min.y + 7.0),
            egui::Align2::RIGHT_TOP,
            plan_status_label(step.status),
            theme.typography.caption.clone(),
            intent_color(theme, plan_status_intent(step.status)),
        );
        if let Some(detail) = &step.detail {
            ui.painter().text(
                egui::pos2(text_x, rect.min.y + 28.0),
                egui::Align2::LEFT_TOP,
                detail,
                theme.typography.small.clone(),
                theme.colors.text_muted,
            );
        }
    }

    response
}

fn plan_status_label(status: PlanStepStatus) -> &'static str {
    match status {
        PlanStepStatus::Pending => "Pending",
        PlanStepStatus::Active => "Active",
        PlanStepStatus::Done => "Done",
        PlanStepStatus::Blocked => "Blocked",
    }
}

fn plan_status_intent(status: PlanStepStatus) -> Intent {
    match status {
        PlanStepStatus::Pending => Intent::Neutral,
        PlanStepStatus::Active => Intent::Info,
        PlanStepStatus::Done => Intent::Success,
        PlanStepStatus::Blocked => Intent::Warning,
    }
}

fn patch_file_ui(ui: &mut Ui, theme: &CastTheme, file: PatchFile) {
    egui::Frame::new()
        .fill(theme.colors.surface_raised)
        .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
        .corner_radius(egui::CornerRadius::same(theme.radius.md as u8))
        .inner_margin(egui::Margin::symmetric(
            theme.spacing.sm as i8,
            theme.spacing.xs as i8,
        ))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    Badge::new(tool_call_status_label(file.status))
                        .intent(tool_call_intent(file.status))
                        .status_dot(),
                );
                ui.label(
                    RichText::new(file.path)
                        .font(theme.typography.code.clone())
                        .color(theme.colors.text),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("-{}", file.deletions))
                            .font(theme.typography.caption.clone())
                            .color(intent_color(theme, Intent::Danger)),
                    );
                    ui.label(
                        RichText::new(format!("+{}", file.additions))
                            .font(theme.typography.caption.clone())
                            .color(intent_color(theme, Intent::Success)),
                    );
                });
            });
        });
    ui.add_space(theme.spacing.xs);
}

fn show_chat_message_content(
    message: ChatMessage,
    ui: &mut Ui,
    add_content: Option<impl FnOnce(&mut Ui)>,
) -> Response {
    let theme = theme_for_ui(ui);
    let colors = chat_message_colors(&theme, message.role, message.intent);
    let body = message.body;

    chat_message_frame(&theme, colors)
        .show(ui, |ui| {
            if let Some(width) = message.width {
                let inner_width = frame_inner_width(width, theme.spacing.md);
                ui.set_width(inner_width);
                ui.set_max_width(inner_width);
            }

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = theme.spacing.xs;
                paint_role_dot(ui, &theme, message.intent);
                ui.label(
                    RichText::new(message.title)
                        .font(theme.typography.strong.clone())
                        .color(theme.colors.text)
                        .extra_letter_spacing(theme.typography.letter_spacing),
                );
                ui.add_space(theme.spacing.xs);
                if let Some(metadata) = message.metadata {
                    ui.label(
                        RichText::new(metadata)
                            .font(theme.typography.caption.clone())
                            .color(theme.colors.text_subtle)
                            .extra_letter_spacing(theme.typography.letter_spacing),
                    );
                }
                if message.streaming {
                    ui.add_space(theme.spacing.xs);
                    ui.add(Loader::new().intent(Intent::Info).size(Size::Small));
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add(
                            Button::new("Copy")
                                .variant(Variant::Ghost)
                                .size(Size::Small),
                        )
                        .clicked()
                    {
                        ui.ctx().copy_text(body.clone());
                    }
                });
            });
            ui.add_space(theme.spacing.xs);
            if !body.is_empty() {
                ui.add(Markdown::new(body).width(ui.available_width()));
            }
            if let Some(add_content) = add_content {
                ui.add_space(theme.spacing.sm);
                add_content(ui);
            }
        })
        .response
}

fn trim_submitted_newline(text: &mut String) {
    if text.ends_with('\n') {
        text.pop();
        if text.ends_with('\r') {
            text.pop();
        }
    }
}

#[derive(Clone, Debug)]
struct TimelineRow {
    rect: egui::Rect,
    response: Response,
}

fn allocate_timeline_item(ui: &mut Ui, item: &RunTimelineItem) -> TimelineRow {
    let height = timeline_item_height(item);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    TimelineRow { rect, response }
}

fn timeline_item_height(item: &RunTimelineItem) -> f32 {
    if item.detail.is_some() { 58.0 } else { 42.0 }
}

fn paint_timeline_guide(ui: &Ui, theme: &CastTheme, rows: &[TimelineRow]) {
    let (Some(first), Some(last)) = (rows.first(), rows.last()) else {
        return;
    };
    if rows.len() < 2 {
        return;
    }

    let x = timeline_dot_center(first.rect).x;
    let start_y = timeline_dot_center(first.rect).y;
    let end_y = timeline_dot_center(last.rect).y;
    let line_rect = egui::Rect::from_min_max(egui::pos2(x, start_y), egui::pos2(x, end_y));
    if ui.is_rect_visible(line_rect.expand(theme.stroke.sm)) {
        ui.painter().vline(
            x,
            start_y..=end_y,
            egui::Stroke::new(theme.stroke.sm, theme.colors.border),
        );
    }
}

fn paint_timeline_item(ui: &Ui, theme: &CastTheme, rect: egui::Rect, item: &RunTimelineItem) {
    if ui.is_rect_visible(rect) {
        let dot_center = timeline_dot_center(rect);
        ui.painter().circle_filled(
            dot_center,
            5.0,
            intent_color(theme, tool_call_intent(item.status)),
        );
        let content_x = dot_center.x + 18.0;
        ui.painter().text(
            egui::pos2(content_x, rect.min.y + 5.0),
            egui::Align2::LEFT_TOP,
            format!("{} · {}", run_phase_label(item.phase), item.title),
            theme.typography.button.clone(),
            theme.colors.text,
        );
        if let Some(metadata) = &item.metadata {
            ui.painter().text(
                egui::pos2(rect.max.x, rect.min.y + 5.0),
                egui::Align2::RIGHT_TOP,
                metadata,
                theme.typography.caption.clone(),
                theme.colors.text_subtle,
            );
        }
        if let Some(detail) = &item.detail {
            ui.painter().text(
                egui::pos2(content_x, rect.min.y + 28.0),
                egui::Align2::LEFT_TOP,
                detail,
                theme.typography.small.clone(),
                theme.colors.text_muted,
            );
        }
    }
}

fn timeline_dot_center(rect: egui::Rect) -> egui::Pos2 {
    egui::pos2(rect.min.x + 10.0, rect.min.y + 14.0)
}

fn run_phase_label(phase: RunPhase) -> &'static str {
    match phase {
        RunPhase::Planning => "Plan",
        RunPhase::ToolCall => "Tool",
        RunPhase::Patch => "Patch",
        RunPhase::Test => "Test",
        RunPhase::Review => "Review",
        RunPhase::FinalResponse => "Final",
    }
}

fn paint_code_region(
    ui: &mut Ui,
    theme: &CastTheme,
    body: &str,
    kind: ToolOutputKind,
    wrap: bool,
    height: f32,
) -> Response {
    let output = egui::ScrollArea::vertical()
        .id_salt(ui.next_auto_id())
        .max_height(height)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if !wrap {
                ui.set_min_width(
                    (body.lines().map(str::len).max().unwrap_or(0) as f32 * 7.0)
                        .max(ui.available_width()),
                );
            }
            let label = egui::Label::new(
                RichText::new(body)
                    .font(tool_output_font(theme, kind))
                    .color(tool_output_text_color(theme, kind))
                    .extra_letter_spacing(theme.typography.letter_spacing),
            )
            .selectable(true);
            ui.add(if wrap { label.wrap() } else { label.extend() })
        });
    output.inner
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
        let mut selected_model = 0;
        let composer = AgentComposer::new(&mut text)
            .send_label("Run")
            .stop_label("Cancel")
            .attachment_label("Attach")
            .tool_label("Tools")
            .model_selector(&mut selected_model, ["Small", "Large"])
            .loading(true)
            .rows(1);

        assert_eq!(composer.send_label, "Run");
        assert_eq!(composer.stop_label, "Cancel");
        assert_eq!(composer.tool_label.as_deref(), Some("Tools"));
        assert_eq!(composer.model_options, ["Small", "Large"]);
        assert!(composer.loading);
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

    #[test]
    fn submitted_newline_is_trimmed_for_enter_send() {
        let mut text = String::from("Run tests\n");

        trim_submitted_newline(&mut text);

        assert_eq!(text, "Run tests");
    }

    #[test]
    fn workflow_components_store_state() {
        let mut open = false;
        let block = ToolCallBlock::new("cargo test", &mut open)
            .status(ToolCallStatus::Running)
            .arguments("package: cast-ui")
            .elapsed("1.2s")
            .preview("running 189 tests");
        let timeline = RunTimeline::new()
            .item(RunTimelineItem::new(RunPhase::Planning, "Plan").metadata("now"))
            .item(
                RunTimelineItem::new(RunPhase::ToolCall, "Run tests")
                    .status(ToolCallStatus::Running),
            );
        let output = CodeOutputPanel::new("stdout", "ok")
            .height(24.0)
            .wrap(false);

        assert_eq!(block.status, ToolCallStatus::Running);
        assert_eq!(block.arguments.as_deref(), Some("package: cast-ui"));
        assert_eq!(timeline.items.len(), 2);
        assert_eq!(timeline.items[0].phase, RunPhase::Planning);
        assert_eq!(output.height, 64.0);
        assert!(!output.wrap);
    }

    #[test]
    fn artifact_and_approval_defaults_are_workflow_oriented() {
        let artifact = ArtifactCard::new("Report").kind("Markdown");
        let approval = ApprovalPanel::new("Run command", "Executes cargo test")
            .risk("May take a few seconds")
            .intent(Intent::Warning);

        assert_eq!(artifact.kind, "Markdown");
        assert_eq!(approval.primary_label, "Approve");
        assert_eq!(approval.intent, Intent::Warning);
        assert!(approval.risk.is_some());
    }

    #[test]
    fn review_components_store_workflow_state() {
        let context = ContextPanel::new(86_000, 200_000)
            .window_count(3)
            .auto_compact_at(6)
            .item(ContextItem::new("File", "src/main.rs").tokens(480));
        let plan = PlanList::new("Plan")
            .summary("Review before patching")
            .step(PlanStep::new("Inspect").status(PlanStepStatus::Done))
            .step(
                PlanStep::new("Patch")
                    .status(PlanStepStatus::Active)
                    .depth(1),
            );
        let patch = PatchReviewPanel::new("Patch", "Two files changed")
            .file(PatchFile::new("src/main.rs", 12, 4))
            .tests("checks passed")
            .risk("Shared route touched");

        assert_eq!(context.used, 86_000);
        assert_eq!(context.items.len(), 1);
        assert_eq!(plan.steps[1].status, PlanStepStatus::Active);
        assert_eq!(plan.steps[1].depth, 1);
        assert_eq!(patch.files[0].additions, 12);
        assert_eq!(patch.risk.as_deref(), Some("Shared route touched"));
    }
}
