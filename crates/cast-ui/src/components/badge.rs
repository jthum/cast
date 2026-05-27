use egui::{Response, RichText, Ui, Widget};

use crate::theme::{CastTheme, current_theme};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Intent {
    Neutral,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

#[derive(Clone, Debug)]
pub struct Badge {
    label: String,
    intent: Intent,
}

impl Badge {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            intent: Intent::Neutral,
        }
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }
}

impl Widget for Badge {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = current_theme(ui.ctx()).unwrap_or_else(CastTheme::light);
        let (fill, text) = match self.intent {
            Intent::Neutral => (theme.colors.surface_muted, theme.colors.text_muted),
            Intent::Primary => (theme.colors.primary, theme.colors.primary_fg),
            Intent::Success => (theme.colors.success, theme.colors.success_fg),
            Intent::Warning => (theme.colors.warning, theme.colors.warning_fg),
            Intent::Danger => (theme.colors.danger, theme.colors.danger_fg),
            Intent::Info => (theme.colors.info, theme.colors.info_fg),
        };

        ui.add(
            egui::Button::new(RichText::new(self.label).color(text).size(12.0))
                .fill(fill)
                .stroke(egui::Stroke::new(theme.stroke.sm, fill))
                .small(),
        )
    }
}
