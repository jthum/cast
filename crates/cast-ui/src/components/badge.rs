use egui::{Response, RichText, Ui, Widget};

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
}

impl Badge {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            intent: Intent::Neutral,
            variant: None,
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
        self.variant = Some(variant);
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
        let colors = resolve_badge_colors(&theme, self.intent, self.variant);
        let metrics = resolve_badge_metrics(&theme, self.size);

        ui.add(
            egui::Button::new(
                RichText::new(self.label)
                    .color(colors.fg)
                    .family(theme.typography.button.family.clone())
                    .size(metrics.text_size)
                    .extra_letter_spacing(theme.typography.letter_spacing),
            )
            .fill(colors.fill)
            .stroke(egui::Stroke::new(
                theme.components.badge.border_width,
                colors.border,
            ))
            .corner_radius(egui::CornerRadius::same(
                theme.components.badge.radius as u8,
            ))
            .min_size(egui::vec2(
                metrics.padding.x * 2.0,
                metrics.min_height.max(metrics.padding.y * 2.0),
            ))
            .small(),
        )
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
            fg: family.base,
            border: mix_with_transparent(family.base, 0.30),
        };
    }

    if variant != Variant::Subtle || matches!(intent, Intent::Neutral) {
        return resolve_intent_colors(theme, intent, variant);
    }

    let family = badge_semantic_family(theme, intent);
    IntentColors {
        fill: mix_with_transparent(family.base, 0.05),
        fg: family.base,
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
        assert_eq!(colors.fg, theme.colors.success_family.base);
    }

    #[test]
    fn explicit_badge_variant_is_respected() {
        let theme = CastTheme::light();
        let colors = resolve_badge_colors(&theme, Intent::Primary, Some(Variant::Subtle));
        let [_, _, _, fill_alpha] = colors.fill.to_srgba_unmultiplied();

        assert_eq!(fill_alpha, 13);
        assert_eq!(colors.fg, theme.colors.primary_family.base);
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
    }
}
