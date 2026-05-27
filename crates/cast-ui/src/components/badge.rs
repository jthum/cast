use egui::{Response, RichText, Ui, Widget};

use crate::{
    foundation::{Intent, Size, Variant},
    style::{resolve_badge_metrics, resolve_intent_colors},
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
        let colors = resolve_intent_colors(&theme, self.intent, self.variant);
        let metrics = resolve_badge_metrics(&theme, self.size);

        ui.add(
            egui::Button::new(
                RichText::new(self.label)
                    .color(colors.fg)
                    .size(metrics.text_size),
            )
            .fill(colors.fill)
            .stroke(egui::Stroke::new(
                theme.components.badge.border_width,
                colors.border,
            ))
            .min_size(egui::vec2(
                metrics.padding.x * 2.0,
                metrics.min_height.max(metrics.padding.y * 2.0),
            ))
            .small(),
        )
    }
}
