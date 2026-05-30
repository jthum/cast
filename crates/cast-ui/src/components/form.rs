use egui::{InnerResponse, RichText, Ui};

use crate::{
    foundation::Intent,
    style::alert_intent_colors,
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
