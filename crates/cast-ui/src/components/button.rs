use egui::{Response, RichText, Ui, Widget};

use crate::theme::{CastTheme, current_theme};

#[derive(Clone, Debug)]
pub struct Button {
    label: String,
}

impl Button {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl Widget for Button {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = current_theme(ui.ctx()).unwrap_or_else(CastTheme::light);
        let text = RichText::new(self.label).color(theme.colors.primary_fg);

        ui.add(
            egui::Button::new(text)
                .fill(theme.colors.primary)
                .stroke(egui::Stroke::new(
                    theme.stroke.sm,
                    theme.colors.border_strong,
                ))
                .min_size(egui::vec2(0.0, theme.controls.min_height)),
        )
    }
}
