use egui::{Response, RichText, Ui, Widget};

use crate::{
    foundation::{Intent, Size, Variant},
    style::resolve_component_style,
    theme::theme_for_ui,
};

#[derive(Clone, Debug)]
pub struct Button {
    label: String,
    leading_icon: Option<String>,
    trailing_icon: Option<String>,
    intent: Intent,
    variant: Variant,
    size: Size,
    enabled: bool,
    loading: bool,
}

impl Button {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            leading_icon: None,
            trailing_icon: None,
            intent: Intent::Primary,
            variant: Variant::Solid,
            size: Size::Medium,
            enabled: true,
            loading: false,
        }
    }

    #[must_use]
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    #[must_use]
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
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

    fn display_label(&self) -> String {
        let mut parts = Vec::new();
        if self.loading {
            parts.push("...".to_owned());
        }
        if let Some(icon) = &self.leading_icon {
            parts.push(icon.clone());
        }
        parts.push(self.label.clone());
        if let Some(icon) = &self.trailing_icon {
            parts.push(icon.clone());
        }
        parts.join(" ")
    }
}

impl Widget for Button {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let style = resolve_component_style(&theme, self.intent, self.variant, self.size);
        let enabled = self.enabled && !self.loading;
        let text = RichText::new(self.display_label())
            .color(style.colors.fg)
            .size(style.metrics.text_size);

        ui.add_enabled(
            enabled,
            egui::Button::new(text)
                .fill(style.colors.fill)
                .stroke(style.stroke)
                .corner_radius(egui::CornerRadius::same(
                    theme.components.button.radius as u8,
                ))
                .min_size(egui::vec2(
                    style.metrics.padding.x * 2.0,
                    style.metrics.min_height.max(style.metrics.padding.y * 2.0),
                )),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_display_label_includes_loading_and_icon_slots() {
        let label = Button::new("Save")
            .leading_icon("[+]")
            .trailing_icon("[>]")
            .loading(true)
            .display_label();

        assert_eq!(label, "... [+] Save [>]");
    }
}
