use egui::{Response, RichText, Ui, Widget};

use crate::{foundation::Size, style::resolve_control_metrics, theme::theme_for_ui};

#[derive(Clone, Debug)]
pub struct Label {
    text: String,
    size: Size,
    muted: bool,
}

impl Label {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            size: Size::Medium,
            muted: false,
        }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }
}

impl Widget for Label {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let metrics = resolve_control_metrics(&theme, self.size);
        let color = if self.muted {
            theme.colors.text_muted
        } else {
            theme.colors.text
        };

        ui.label(
            RichText::new(self.text)
                .color(color)
                .family(theme.typography.body.family.clone())
                .size(metrics.text_size),
        )
    }
}
