use egui::{Response, RichText, Ui, Widget};

use crate::{
    foundation::{Intent, Size, Variant},
    style::resolve_component_style,
    theme::theme_for_ui,
};

#[derive(Clone, Debug)]
pub struct Badge {
    label: String,
    intent: Intent,
    variant: Variant,
    size: Size,
}

impl Badge {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            intent: Intent::Neutral,
            variant: Variant::Subtle,
            size: Size::Small,
        }
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    #[must_use]
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Widget for Badge {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let style = resolve_component_style(&theme, self.intent, self.variant, self.size);

        ui.add(
            egui::Button::new(
                RichText::new(self.label)
                    .color(style.colors.fg)
                    .size(style.metrics.text_size),
            )
            .fill(style.colors.fill)
            .stroke(style.stroke)
            .min_size(egui::vec2(
                style.metrics.padding.x * 2.0,
                style.metrics.min_height.max(style.metrics.padding.y * 2.0),
            ))
            .small(),
        )
    }
}
