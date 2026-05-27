use egui::{Align2, Color32, Response, Sense, StrokeKind, Ui, Widget};

use crate::{
    color::with_alpha,
    foundation::{Intent, Size, Variant},
    style::resolve_component_style,
    theme::{ThemeMode, theme_for_ui},
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
        let text = self.display_label();
        let mut font_id = theme.typography.button.clone();
        font_id.size = style.metrics.text_size;
        let galley = ui
            .painter()
            .layout_no_wrap(text.clone(), font_id.clone(), style.colors.fg);
        let desired_size = egui::vec2(
            (galley.size().x + style.metrics.padding.x * 2.0).max(style.metrics.padding.x * 2.0),
            (galley.size().y + style.metrics.padding.y * 2.0)
                .max(style.metrics.min_height.max(style.metrics.padding.y * 2.0)),
        );
        let sense = if enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, response) = ui.allocate_exact_size(desired_size, sense);

        if ui.is_rect_visible(rect) {
            let pressed = enabled && response.is_pointer_button_down_on();
            let hovered = enabled && response.hovered();
            let radius = egui::CornerRadius::same(theme.components.button.radius as u8);
            let depth = if pressed { 1.0 } else { 0.0 };
            let paint_rect = rect.translate(egui::vec2(0.0, depth));
            let fill = button_fill(style.colors.fill, &theme, self.variant, hovered, pressed);
            let fg = if enabled {
                style.colors.fg
            } else {
                theme.colors.text_subtle
            };
            let border = if pressed || hovered {
                theme.colors.border_strong
            } else if enabled {
                style.colors.border
            } else {
                theme.colors.border
            };

            if enabled && style.colors.fill != Color32::TRANSPARENT && !pressed {
                let shadow_rect = rect.translate(egui::vec2(0.0, 1.5));
                ui.painter().rect_filled(
                    shadow_rect,
                    radius,
                    with_alpha(Color32::BLACK, theme.elevation.shadow_alpha / 2),
                );
            }

            ui.painter().rect(
                paint_rect,
                radius,
                fill,
                egui::Stroke::new(style.stroke.width, border),
                StrokeKind::Outside,
            );
            ui.painter().text(
                paint_rect.center(),
                Align2::CENTER_CENTER,
                text,
                font_id,
                fg,
            );
        }

        response
    }
}

fn button_fill(
    fill: Color32,
    theme: &crate::CastTheme,
    variant: Variant,
    hovered: bool,
    pressed: bool,
) -> Color32 {
    if matches!(variant, Variant::Ghost | Variant::Outline) && fill == Color32::TRANSPARENT {
        return if pressed {
            theme.colors.surface_raised
        } else if hovered {
            theme.colors.surface_muted
        } else {
            Color32::TRANSPARENT
        };
    }

    let anchor = match theme.mode {
        ThemeMode::Light => Color32::BLACK,
        ThemeMode::Dark => Color32::WHITE,
    };

    if pressed {
        fill.lerp_to_gamma(anchor, 0.14)
    } else if hovered {
        fill.lerp_to_gamma(anchor, 0.07)
    } else {
        fill
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

    #[test]
    fn ghost_button_gets_interactive_fill() {
        let theme = crate::CastTheme::light();

        assert_eq!(
            button_fill(Color32::TRANSPARENT, &theme, Variant::Ghost, false, false),
            Color32::TRANSPARENT
        );
        assert_eq!(
            button_fill(Color32::TRANSPARENT, &theme, Variant::Ghost, true, false),
            theme.colors.surface_muted
        );
        assert_eq!(
            button_fill(Color32::TRANSPARENT, &theme, Variant::Ghost, true, true),
            theme.colors.surface_raised
        );
    }
}
