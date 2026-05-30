use egui::{Align, InnerResponse, Layout, Response, RichText, StrokeKind, Ui, Widget};

use crate::{
    color::mix_with_transparent,
    foundation::Intent,
    style::{alert_frame, alert_intent_colors},
    theme::{CastTheme, theme_for_ui},
};

#[derive(Clone, Debug)]
pub struct FormField {
    label: String,
    description: Option<String>,
    message: Option<FormFieldMessage>,
    required: bool,
    width: Option<f32>,
}

#[derive(Clone, Debug)]
pub struct FormSection {
    title: String,
    description: Option<String>,
    width: Option<f32>,
}

impl FormSection {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            width: None,
        }
    }

    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(160.0));
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);

        ui.vertical(|ui| {
            if let Some(width) = self.width {
                ui.set_min_width(width);
                ui.set_max_width(width);
            }

            ui.spacing_mut().item_spacing.y = theme.spacing.xs;
            ui.label(
                RichText::new(self.title)
                    .font(theme.typography.heading_sm.clone())
                    .color(theme.colors.text)
                    .extra_letter_spacing(theme.typography.letter_spacing),
            );

            if let Some(description) = self.description {
                paint_field_support_text(ui, &theme, &description, theme.colors.text_muted);
            }

            ui.add_space(theme.spacing.sm);
            add_contents(ui)
        })
    }
}

#[derive(Clone, Debug)]
pub struct FormActions {
    separator: bool,
    align_end: bool,
}

impl FormActions {
    #[must_use]
    pub fn new() -> Self {
        Self {
            separator: true,
            align_end: true,
        }
    }

    #[must_use]
    pub fn separator(mut self, separator: bool) -> Self {
        self.separator = separator;
        self
    }

    #[must_use]
    pub fn align_end(mut self, align_end: bool) -> Self {
        self.align_end = align_end;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_actions: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);

        ui.vertical(|ui| {
            if self.separator {
                ui.add_space(theme.spacing.sm);
                let rect = ui
                    .allocate_exact_size(
                        egui::vec2(ui.available_width(), theme.stroke.sm.max(1.0)),
                        egui::Sense::hover(),
                    )
                    .0;
                ui.painter().rect_filled(rect, 0.0, theme.colors.border);
                ui.add_space(theme.spacing.sm);
            }

            if self.align_end {
                ui.with_layout(Layout::right_to_left(Align::Center), add_actions)
            } else {
                ui.horizontal(add_actions)
            }
        })
        .inner
    }
}

impl Default for FormActions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationIssue {
    field: Option<String>,
    message: String,
}

impl ValidationIssue {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            field: None,
            message: message.into(),
        }
    }

    #[must_use]
    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }
}

#[derive(Clone, Debug)]
pub struct ValidationSummary {
    title: String,
    issues: Vec<ValidationIssue>,
    intent: Intent,
    width: Option<f32>,
    attention: bool,
    scroll_to: bool,
}

impl ValidationSummary {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            issues: Vec::new(),
            intent: Intent::Danger,
            width: None,
            attention: false,
            scroll_to: false,
        }
    }

    #[must_use]
    pub fn issue(mut self, issue: ValidationIssue) -> Self {
        self.issues.push(issue);
        self
    }

    #[must_use]
    pub fn issues(mut self, issues: impl IntoIterator<Item = ValidationIssue>) -> Self {
        self.issues.extend(issues);
        self
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(180.0));
        self
    }

    #[must_use]
    pub fn attention(mut self, attention: bool) -> Self {
        self.attention = attention;
        self
    }

    #[must_use]
    pub fn scroll_to(mut self, scroll_to: bool) -> Self {
        self.scroll_to = scroll_to;
        self
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.issues.is_empty()
    }
}

impl Widget for ValidationSummary {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let colors = alert_intent_colors(&theme, self.intent);
        let attention = self.attention;
        let scroll_to = self.scroll_to;

        let response = alert_frame(&theme, colors.border)
            .fill(colors.fill)
            .show(ui, |ui| {
                if let Some(width) = self.width {
                    ui.set_min_width(width);
                    ui.set_max_width(width);
                }

                ui.spacing_mut().item_spacing.y = theme.spacing.xs;
                ui.label(
                    RichText::new(self.title)
                        .font(theme.typography.strong.clone())
                        .color(colors.fg)
                        .extra_letter_spacing(theme.typography.letter_spacing),
                );

                for issue in self.issues {
                    ui.horizontal_top(|ui| {
                        ui.spacing_mut().item_spacing.x = theme.spacing.sm;
                        ui.label(
                            RichText::new("•")
                                .font(theme.typography.small.clone())
                                .color(colors.fg)
                                .extra_letter_spacing(theme.typography.letter_spacing),
                        );
                        let text = validation_issue_text(&issue);
                        ui.label(
                            RichText::new(text)
                                .font(theme.typography.small.clone())
                                .color(theme.colors.text_muted)
                                .extra_letter_spacing(theme.typography.letter_spacing),
                        );
                    });
                }
            })
            .response;

        if attention {
            ui.painter().rect_stroke(
                response.rect.expand(3.0),
                egui::CornerRadius::same(theme.components.alert.radius as u8),
                egui::Stroke::new(2.0, mix_with_transparent(colors.fg, 0.35)),
                StrokeKind::Outside,
            );
        }

        if scroll_to {
            response.scroll_to_me(Some(Align::Center));
        }

        response
    }
}

impl FormField {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: None,
            message: None,
            required: false,
            width: None,
        }
    }

    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn message(mut self, intent: Intent, message: impl Into<String>) -> Self {
        self.message = Some(FormFieldMessage {
            intent,
            text: message.into(),
        });
        self
    }

    #[must_use]
    pub fn success(self, message: impl Into<String>) -> Self {
        self.message(Intent::Success, message)
    }

    #[must_use]
    pub fn warning(self, message: impl Into<String>) -> Self {
        self.message(Intent::Warning, message)
    }

    #[must_use]
    pub fn error(self, message: impl Into<String>) -> Self {
        self.message(Intent::Danger, message)
    }

    #[must_use]
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(120.0));
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_control: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);

        ui.vertical(|ui| {
            if let Some(width) = self.width {
                ui.set_min_width(width);
                ui.set_max_width(width);
            }

            ui.spacing_mut().item_spacing.y = theme.spacing.xs;
            paint_field_label(ui, &theme, &self.label, self.required);
            let inner = add_control(ui);

            if let Some(description) = self.description {
                paint_field_support_text(ui, &theme, &description, theme.colors.text_muted);
            }

            if let Some(message) = self.message {
                let color = form_message_color(&theme, message.intent);
                paint_field_support_text(ui, &theme, &message.text, color);
            }

            inner
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FormFieldMessage {
    intent: Intent,
    text: String,
}

fn paint_field_label(ui: &mut Ui, theme: &CastTheme, label: &str, required: bool) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = theme.spacing.xs * 0.5;
        ui.label(
            RichText::new(label)
                .font(theme.typography.label.clone())
                .color(theme.colors.text)
                .extra_letter_spacing(theme.typography.letter_spacing),
        );

        if required {
            ui.label(
                RichText::new("*")
                    .font(theme.typography.label.clone())
                    .color(theme.colors.danger_family.emphasis)
                    .extra_letter_spacing(theme.typography.letter_spacing),
            );
        }
    });
}

fn paint_field_support_text(ui: &mut Ui, theme: &CastTheme, text: &str, color: egui::Color32) {
    ui.label(
        RichText::new(text)
            .font(theme.typography.small.clone())
            .color(color)
            .extra_letter_spacing(theme.typography.letter_spacing),
    );
}

fn form_message_color(theme: &CastTheme, intent: Intent) -> egui::Color32 {
    if intent == Intent::Neutral {
        theme.colors.text_muted
    } else {
        alert_intent_colors(theme, intent).fg
    }
}

fn validation_issue_text(issue: &ValidationIssue) -> String {
    issue.field.as_ref().map_or_else(
        || issue.message.clone(),
        |field| format!("{field}: {}", issue.message),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_field_defaults_to_plain_label() {
        let field = FormField::new("Project name");

        assert_eq!(field.label, "Project name");
        assert!(!field.required);
        assert!(field.description.is_none());
        assert!(field.message.is_none());
    }

    #[test]
    fn form_section_defaults_to_plain_group() {
        let section = FormSection::new("Basics");

        assert_eq!(section.title, "Basics");
        assert!(section.description.is_none());
        assert!(section.width.is_none());
    }

    #[test]
    fn form_section_width_has_floor() {
        let section = FormSection::new("Basics").width(80.0);

        assert_eq!(section.width, Some(160.0));
    }

    #[test]
    fn form_actions_default_to_separated_end_alignment() {
        let actions = FormActions::new();

        assert!(actions.separator);
        assert!(actions.align_end);
    }

    #[test]
    fn validation_summary_collects_issues() {
        let summary = ValidationSummary::new("Review fields")
            .issue(ValidationIssue::new("Required").field("Handle"))
            .issues([ValidationIssue::new("Choose a preset")])
            .intent(Intent::Warning)
            .width(80.0)
            .attention(true)
            .scroll_to(true);

        assert_eq!(summary.issues.len(), 2);
        assert_eq!(summary.intent, Intent::Warning);
        assert_eq!(summary.width, Some(180.0));
        assert!(summary.attention);
        assert!(summary.scroll_to);
        assert!(!summary.is_empty());
    }

    #[test]
    fn validation_issue_text_includes_field_when_present() {
        let issue = ValidationIssue::new("Required").field("Handle");

        assert_eq!(validation_issue_text(&issue), "Handle: Required");
        assert_eq!(
            validation_issue_text(&ValidationIssue::new("Choose a preset")),
            "Choose a preset"
        );
    }

    #[test]
    fn form_field_width_has_floor() {
        let field = FormField::new("Project name").width(80.0);

        assert_eq!(field.width, Some(120.0));
    }

    #[test]
    fn form_field_status_helpers_set_message_intent() {
        let field = FormField::new("Handle").required(true).error("Required");

        assert!(field.required);
        assert_eq!(
            field.message,
            Some(FormFieldMessage {
                intent: Intent::Danger,
                text: "Required".to_owned(),
            })
        );
    }

    #[test]
    fn neutral_form_message_uses_muted_text() {
        let theme = CastTheme::light();

        assert_eq!(
            form_message_color(&theme, Intent::Neutral),
            theme.colors.text_muted
        );
    }
}
