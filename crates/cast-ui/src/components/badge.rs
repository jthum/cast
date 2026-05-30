use egui::{
    Color32, Response, Sense, StrokeKind, Ui, Widget,
    text::{LayoutJob, TextFormat},
};

use crate::{
    color::{contrast_ratio, mix_oklch, mix_with_transparent},
    foundation::{Intent, Size, Variant},
    style::{IntentColors, resolve_badge_metrics, resolve_intent_colors},
    theme::{CastTheme, SemanticColorTokens, ThemeMode, theme_for_ui},
};

#[derive(Clone, Debug)]
pub struct Badge {
    label: String,
    intent: Intent,
    variant: Option<Variant>,
    size: Size,
    status_dot: bool,
}

impl Badge {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            intent: Intent::Neutral,
            variant: None,
            size: Size::Small,
            status_dot: false,
        }
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    #[must_use]
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = Some(variant);
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn status_dot(mut self) -> Self {
        self.status_dot = true;
        self.variant = Some(Variant::Outline);
        self
    }
}

impl Widget for Badge {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let colors = if self.status_dot {
            resolve_status_dot_badge_colors(&theme)
        } else {
            resolve_badge_colors(&theme, self.intent, self.variant)
        };
        let metrics = resolve_badge_metrics(&theme, self.size);
        let dot_size = if self.status_dot {
            badge_dot_size(self.size)
        } else {
            0.0
        };
        let dot_gap = if self.status_dot {
            theme.spacing.xs + 1.0
        } else {
            0.0
        };
        let text = ui.painter().layout_job(badge_layout_job(
            self.label,
            egui::FontId::new(metrics.text_size, theme.typography.button.family.clone()),
            theme.typography.letter_spacing,
        ));
        let desired_size = egui::vec2(
            metrics.padding.x * 2.0 + dot_size + dot_gap + text.size().x,
            metrics
                .min_height
                .max(metrics.padding.y * 2.0 + text.size().y),
        );
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.painter().rect(
                rect,
                egui::CornerRadius::same(theme.components.badge.radius as u8),
                colors.fill,
                egui::Stroke::new(theme.components.badge.border_width, colors.border),
                StrokeKind::Outside,
            );

            let mut x = rect.min.x + metrics.padding.x;
            if self.status_dot {
                let dot_radius = dot_size / 2.0;
                ui.painter().circle_filled(
                    egui::pos2(x + dot_radius, rect.center().y),
                    dot_radius,
                    badge_dot_color(&theme, self.intent),
                );
                x += dot_size + dot_gap;
            }

            ui.painter().galley(
                egui::pos2(x, rect.center().y - text.size().y / 2.0),
                text,
                colors.fg,
            );
        }

        response
    }
}

fn resolve_status_dot_badge_colors(theme: &CastTheme) -> IntentColors {
    IntentColors {
        fill: egui::Color32::TRANSPARENT,
        fg: theme.colors.text,
        border: match theme.mode {
            ThemeMode::Light => theme.colors.border,
            ThemeMode::Dark => mix_with_transparent(theme.colors.text_muted, 0.28),
        },
    }
}

fn badge_layout_job(text: String, font_id: egui::FontId, letter_spacing: f32) -> LayoutJob {
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

fn badge_dot_size(size: Size) -> f32 {
    match size {
        Size::Small => 7.0,
        Size::Medium => 8.0,
        Size::Large => 9.0,
    }
}

fn badge_dot_color(theme: &CastTheme, intent: Intent) -> egui::Color32 {
    match intent {
        Intent::Neutral => theme.colors.text_muted,
        Intent::Primary => theme.colors.primary_family.base,
        Intent::Secondary => theme.colors.secondary_family.base,
        Intent::Success => theme.colors.success_family.base,
        Intent::Warning => theme.colors.warning_family.base,
        Intent::Danger => theme.colors.danger_family.base,
        Intent::Info => theme.colors.info_family.base,
    }
}

fn resolve_badge_colors(
    theme: &CastTheme,
    intent: Intent,
    explicit_variant: Option<Variant>,
) -> IntentColors {
    let variant = explicit_variant.unwrap_or_else(|| default_badge_variant(intent));

    if matches!(
        (theme.mode, intent, variant),
        (ThemeMode::Dark, Intent::Primary, Variant::Solid)
    ) {
        return IntentColors {
            fill: theme.colors.text,
            fg: dark_primary_badge_fg(theme),
            border: theme.colors.text,
        };
    }

    if variant == Variant::Outline && intent != Intent::Neutral {
        let family = badge_semantic_family(theme, intent);
        return IntentColors {
            fill: egui::Color32::TRANSPARENT,
            fg: family.emphasis,
            border: mix_with_transparent(family.base, 0.30),
        };
    }

    if variant != Variant::Subtle || matches!(intent, Intent::Neutral) {
        return resolve_intent_colors(theme, intent, variant);
    }

    let family = badge_semantic_family(theme, intent);
    IntentColors {
        fill: mix_with_transparent(family.base, 0.05),
        fg: family.emphasis,
        border: mix_with_transparent(family.base, 0.30),
    }
}

fn default_badge_variant(intent: Intent) -> Variant {
    match intent {
        Intent::Primary | Intent::Secondary => Variant::Solid,
        Intent::Neutral | Intent::Success | Intent::Warning | Intent::Danger | Intent::Info => {
            Variant::Subtle
        }
    }
}

fn badge_semantic_family(theme: &CastTheme, intent: Intent) -> SemanticColorTokens {
    match intent {
        Intent::Neutral => unreachable!("neutral subtle badges use neutral intent colors"),
        Intent::Primary => theme.colors.primary_family,
        Intent::Secondary => theme.colors.secondary_family,
        Intent::Success => theme.colors.success_family,
        Intent::Warning => theme.colors.warning_family,
        Intent::Danger => theme.colors.danger_family,
        Intent::Info => theme.colors.info_family,
    }
}

fn dark_primary_badge_fg(theme: &CastTheme) -> egui::Color32 {
    let base = theme.colors.primary_family.base;
    if contrast_ratio(theme.colors.text, base) >= 4.5 {
        base
    } else {
        mix_oklch(base, egui::Color32::BLACK, 0.36)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_badge_defaults_to_solid_primary() {
        let theme = CastTheme::light();
        let colors = resolve_badge_colors(&theme, Intent::Primary, None);

        assert_eq!(colors.fill, theme.colors.primary_family.base);
        assert_eq!(colors.fg, theme.colors.primary_family.fg);
        assert_eq!(colors.border, theme.colors.primary_family.base);
    }

    #[test]
    fn success_badge_defaults_to_transparent_tinted_subtle() {
        let theme = CastTheme::light();
        let colors = resolve_badge_colors(&theme, Intent::Success, None);
        let [_, _, _, fill_alpha] = colors.fill.to_srgba_unmultiplied();
        let [_, _, _, border_alpha] = colors.border.to_srgba_unmultiplied();

        assert_eq!(fill_alpha, 13);
        assert_eq!(border_alpha, 77);
        assert_eq!(colors.fg, theme.colors.success_family.emphasis);
    }

    #[test]
    fn explicit_badge_variant_is_respected() {
        let theme = CastTheme::light();
        let colors = resolve_badge_colors(&theme, Intent::Primary, Some(Variant::Subtle));
        let [_, _, _, fill_alpha] = colors.fill.to_srgba_unmultiplied();

        assert_eq!(fill_alpha, 13);
        assert_eq!(colors.fg, theme.colors.primary_family.emphasis);
    }

    #[test]
    fn dark_primary_badge_uses_light_fill_with_contrast_adjusted_primary_text() {
        let theme = CastTheme::dark();
        let colors = resolve_badge_colors(&theme, Intent::Primary, None);

        assert_eq!(colors.fill, theme.colors.text);
        assert!(contrast_ratio(colors.fill, colors.fg) >= 4.5);
    }

    #[test]
    fn outline_badge_uses_transparent_semantic_border() {
        let theme = CastTheme::dark();
        let colors = resolve_badge_colors(&theme, Intent::Primary, Some(Variant::Outline));
        let [_, _, _, border_alpha] = colors.border.to_srgba_unmultiplied();

        assert_eq!(colors.fill, egui::Color32::TRANSPARENT);
        assert_eq!(border_alpha, 77);
        assert_eq!(colors.fg, theme.colors.primary_family.emphasis);
    }

    #[test]
    fn dark_outline_badge_text_uses_readable_emphasis() {
        let theme = CastTheme::from_palette(
            ThemeMode::Dark,
            crate::CastPaletteInput::from_primary(Color32::from_rgb(88, 28, 135)),
        );
        let colors = resolve_badge_colors(&theme, Intent::Primary, Some(Variant::Outline));

        assert_eq!(colors.fg, theme.colors.primary_family.emphasis);
        assert!(contrast_ratio(theme.colors.surface, colors.fg) >= 4.5);
        assert_ne!(colors.fg, theme.colors.primary_family.base);
    }

    #[test]
    fn status_dot_badge_uses_neutral_outline_with_semantic_dot() {
        let theme = CastTheme::light();
        let colors = resolve_status_dot_badge_colors(&theme);

        assert_eq!(colors.fill, egui::Color32::TRANSPARENT);
        assert_eq!(colors.fg, theme.colors.text);
        assert_eq!(colors.border, theme.colors.border);
        assert_eq!(
            badge_dot_color(&theme, Intent::Success),
            theme.colors.success_family.base
        );
        assert_eq!(badge_dot_size(Size::Small), 7.0);
    }
}
