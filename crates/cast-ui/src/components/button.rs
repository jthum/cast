use egui::{
    Color32, Response, Sense, StrokeKind, Ui, Widget,
    text::{LayoutJob, TextFormat},
};

use crate::{
    color::{accessible_foreground, mix_with_transparent, with_alpha},
    foundation::{Intent, Size, Variant},
    style::{IntentColors, resolve_component_style},
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
        let colors = button_colors(&theme, self.intent, self.variant);
        let enabled = self.enabled && !self.loading;
        let text = self.display_label();
        let mut font_id = theme.typography.button.clone();
        font_id.size = style.metrics.text_size;
        let galley = ui.painter().layout_job(button_layout_job(
            text.clone(),
            font_id.clone(),
            theme.typography.letter_spacing,
        ));
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
            let accent = button_accent(&theme, self.intent).base;
            let active_fill =
                button_fill(colors.fill, accent, &theme, self.variant, hovered, pressed);
            let fill = if enabled {
                active_fill
            } else {
                disabled_button_fill(active_fill)
            };
            let fg = if enabled {
                colors.fg
            } else {
                disabled_button_fg(active_fill, &theme)
            };
            let border = button_border(&theme, accent, self.variant, enabled, hovered, pressed);

            ui.painter().rect(
                paint_rect,
                radius,
                fill,
                egui::Stroke::new(
                    button_border_width(style.stroke.width, self.variant),
                    border,
                ),
                StrokeKind::Outside,
            );
            let text_pos = paint_rect.center() - galley.size() / 2.0;
            ui.painter().galley(text_pos, galley, fg);
        }

        response
    }
}

fn button_colors(theme: &crate::CastTheme, intent: Intent, variant: Variant) -> IntentColors {
    let accent = button_accent(theme, intent);

    match variant {
        Variant::Solid if intent == Intent::Neutral => IntentColors {
            fill: theme.colors.surface_muted,
            fg: theme.colors.text,
            border: Color32::TRANSPARENT,
        },
        Variant::Solid => IntentColors {
            fill: accent.base,
            fg: accent.solid_fg,
            border: Color32::TRANSPARENT,
        },
        Variant::Subtle => IntentColors {
            fill: mix_with_transparent(accent.base, theme.tone.subtle_fill_alpha),
            fg: accent.text,
            border: mix_with_transparent(accent.base, theme.tone.subtle_border_alpha),
        },
        Variant::Outline => IntentColors {
            fill: Color32::TRANSPARENT,
            fg: accent.text,
            border: mix_with_transparent(accent.base, theme.tone.subtle_border_alpha),
        },
        Variant::Ghost => IntentColors {
            fill: Color32::TRANSPARENT,
            fg: accent.text,
            border: Color32::TRANSPARENT,
        },
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ButtonAccent {
    base: Color32,
    solid_fg: Color32,
    text: Color32,
}

fn button_accent(theme: &crate::CastTheme, intent: Intent) -> ButtonAccent {
    match intent {
        Intent::Neutral => ButtonAccent {
            base: theme.colors.text,
            solid_fg: theme.colors.text,
            text: theme.colors.text,
        },
        Intent::Primary => semantic_button_accent(theme.colors.primary_family),
        Intent::Secondary => semantic_button_accent(theme.colors.secondary_family),
        Intent::Success => semantic_button_accent(theme.colors.success_family),
        Intent::Warning => semantic_button_accent(theme.colors.warning_family),
        Intent::Danger => semantic_button_accent(theme.colors.danger_family),
        Intent::Info => semantic_button_accent(theme.colors.info_family),
    }
}

fn semantic_button_accent(family: crate::SemanticColorTokens) -> ButtonAccent {
    ButtonAccent {
        base: family.base,
        solid_fg: family.fg,
        text: family.emphasis,
    }
}

fn disabled_button_fg(fill: Color32, theme: &crate::CastTheme) -> Color32 {
    if fill == Color32::TRANSPARENT {
        theme.colors.text_subtle
    } else {
        with_alpha(accessible_foreground(opaque_color(fill)), 190)
    }
}

fn disabled_button_fill(fill: Color32) -> Color32 {
    scale_alpha(fill, 0.62)
}

fn opaque_color(color: Color32) -> Color32 {
    let [r, g, b, _] = color.to_srgba_unmultiplied();
    Color32::from_rgb(r, g, b)
}

fn scale_alpha(color: Color32, factor: f32) -> Color32 {
    if color == Color32::TRANSPARENT {
        return Color32::TRANSPARENT;
    }

    let [r, g, b, a] = color.to_srgba_unmultiplied();
    let alpha = (f32::from(a) * factor.clamp(0.0, 1.0)).round() as u8;
    Color32::from_rgba_unmultiplied(r, g, b, alpha)
}

fn button_layout_job(text: String, font_id: egui::FontId, letter_spacing: f32) -> LayoutJob {
    LayoutJob::single_section(
        text,
        TextFormat {
            font_id,
            extra_letter_spacing: letter_spacing,
            color: Color32::PLACEHOLDER,
            ..Default::default()
        },
    )
}

fn button_fill(
    fill: Color32,
    accent: Color32,
    theme: &crate::CastTheme,
    variant: Variant,
    hovered: bool,
    pressed: bool,
) -> Color32 {
    if matches!(variant, Variant::Ghost | Variant::Outline) && fill == Color32::TRANSPARENT {
        return if pressed {
            mix_with_transparent(accent, theme.tone.subtle_active_fill_alpha)
        } else if hovered {
            mix_with_transparent(accent, theme.tone.subtle_fill_alpha)
        } else {
            Color32::TRANSPARENT
        };
    }

    if matches!(variant, Variant::Subtle) {
        return if pressed {
            mix_with_transparent(accent, theme.tone.subtle_active_fill_alpha)
        } else if hovered {
            mix_with_transparent(accent, theme.tone.subtle_hover_fill_alpha)
        } else {
            fill
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

fn button_border(
    theme: &crate::CastTheme,
    accent: Color32,
    variant: Variant,
    enabled: bool,
    hovered: bool,
    pressed: bool,
) -> Color32 {
    match variant {
        Variant::Solid | Variant::Ghost => Color32::TRANSPARENT,
        Variant::Subtle | Variant::Outline if !enabled => {
            mix_with_transparent(accent, theme.tone.disabled_border_alpha)
        }
        Variant::Subtle | Variant::Outline if pressed => {
            mix_with_transparent(accent, theme.tone.subtle_active_border_alpha)
        }
        Variant::Subtle | Variant::Outline if hovered => {
            mix_with_transparent(accent, theme.tone.subtle_hover_border_alpha)
        }
        Variant::Subtle | Variant::Outline => {
            mix_with_transparent(accent, theme.tone.subtle_border_alpha)
        }
    }
}

fn button_border_width(width: f32, variant: Variant) -> f32 {
    match variant {
        Variant::Solid | Variant::Ghost => 0.0,
        Variant::Subtle | Variant::Outline => width,
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
        let accent = theme.colors.primary_family.base;

        assert_eq!(
            button_fill(
                Color32::TRANSPARENT,
                accent,
                &theme,
                Variant::Ghost,
                false,
                false
            ),
            Color32::TRANSPARENT
        );
        assert_eq!(
            button_fill(
                Color32::TRANSPARENT,
                accent,
                &theme,
                Variant::Ghost,
                true,
                false
            ),
            mix_with_transparent(accent, 0.05)
        );
        assert_eq!(
            button_fill(
                Color32::TRANSPARENT,
                accent,
                &theme,
                Variant::Ghost,
                true,
                true
            ),
            mix_with_transparent(accent, 0.12)
        );
    }

    #[test]
    fn disabled_solid_button_text_contrasts_with_fill() {
        let theme = crate::CastTheme::light();
        let fg = disabled_button_fg(theme.colors.primary_family.base, &theme);

        let [r, g, b, a] = fg.to_srgba_unmultiplied();
        let expected = accessible_foreground(theme.colors.primary_family.base);

        assert_eq!([r, g, b], [expected.r(), expected.g(), expected.b()]);
        assert_eq!(a, 190);
    }

    #[test]
    fn disabled_button_fill_is_translucent() {
        let theme = crate::CastTheme::light();
        let fill = disabled_button_fill(theme.colors.primary_family.base);
        let [r, _, _, alpha] = fill.to_srgba_unmultiplied();

        assert!((i16::from(r) - i16::from(theme.colors.primary_family.base.r())).abs() <= 2);
        assert_eq!(alpha, 158);
    }

    #[test]
    fn solid_buttons_are_borderless() {
        let theme = crate::CastTheme::light();
        let colors = button_colors(&theme, Intent::Primary, Variant::Solid);

        assert_eq!(colors.border, Color32::TRANSPARENT);
        assert_eq!(
            button_border(
                &theme,
                theme.colors.primary_family.base,
                Variant::Solid,
                true,
                true,
                true
            ),
            Color32::TRANSPARENT
        );
        assert_eq!(button_border_width(1.0, Variant::Solid), 0.0);
    }

    #[test]
    fn subtle_buttons_use_transparent_accent_border() {
        let theme = crate::CastTheme::light();
        let colors = button_colors(&theme, Intent::Success, Variant::Subtle);
        let [_, _, _, fill_alpha] = colors.fill.to_srgba_unmultiplied();
        let [_, _, _, border_alpha] = colors.border.to_srgba_unmultiplied();

        assert_eq!(fill_alpha, 13);
        assert_eq!(border_alpha, 77);
        assert_eq!(colors.fg, theme.colors.success_family.emphasis);
    }
}
