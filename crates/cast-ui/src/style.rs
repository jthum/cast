use egui::{Color32, Margin, Stroke, Vec2};

use crate::{
    color::{mix_with_transparent, with_alpha},
    foundation::{Intent, Size, Variant},
    theme::{ButtonTokens, CastTheme, SemanticColorTokens},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct IntentColors {
    pub(crate) fill: Color32,
    pub(crate) fg: Color32,
    pub(crate) border: Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ControlMetrics {
    pub(crate) min_height: f32,
    pub(crate) padding: Vec2,
    pub(crate) text_size: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ComponentStyle {
    pub(crate) colors: IntentColors,
    pub(crate) metrics: ControlMetrics,
    pub(crate) stroke: Stroke,
}

pub(crate) fn resolve_component_style(
    theme: &CastTheme,
    intent: Intent,
    variant: Variant,
    size: Size,
) -> ComponentStyle {
    let colors = resolve_intent_colors(theme, intent, variant);
    let metrics = resolve_control_metrics(theme, size);

    ComponentStyle {
        colors,
        metrics,
        stroke: Stroke::new(theme.components.button.border_width, colors.border),
    }
}

pub(crate) fn resolve_intent_colors(
    theme: &CastTheme,
    intent: Intent,
    variant: Variant,
) -> IntentColors {
    if intent == Intent::Neutral {
        return neutral_intent_colors(theme, variant);
    }

    semantic_intent_colors(semantic_family(theme, intent), variant)
}

fn semantic_intent_colors(family: SemanticColorTokens, variant: Variant) -> IntentColors {
    match variant {
        Variant::Solid => IntentColors {
            fill: family.base,
            fg: family.fg,
            border: family.base,
        },
        Variant::Subtle => IntentColors {
            fill: family.subtle,
            fg: family.base,
            border: family.border,
        },
        Variant::Outline => IntentColors {
            fill: Color32::TRANSPARENT,
            fg: family.base,
            border: family.border,
        },
        Variant::Ghost => IntentColors {
            fill: Color32::TRANSPARENT,
            fg: family.base,
            border: Color32::TRANSPARENT,
        },
    }
}

pub(crate) fn resolve_control_metrics(theme: &CastTheme, size: Size) -> ControlMetrics {
    button_metrics(theme.components.button, theme, size)
}

pub(crate) fn resolve_badge_metrics(theme: &CastTheme, size: Size) -> ControlMetrics {
    let base = theme.components.badge;
    let metrics = match size {
        Size::Small => (
            base.min_height,
            Vec2::new(base.padding_x, base.padding_y),
            theme.typography.small.size,
        ),
        Size::Medium => (
            base.min_height + 6.0,
            Vec2::new(base.padding_x + theme.spacing.xs, base.padding_y + 3.0),
            theme.typography.body.size,
        ),
        Size::Large => (
            base.min_height + 14.0,
            Vec2::new(
                base.padding_x + theme.spacing.sm,
                base.padding_y + theme.spacing.xs,
            ),
            theme.typography.body.size + 1.0,
        ),
    };

    ControlMetrics {
        min_height: metrics.0,
        padding: metrics.1,
        text_size: metrics.2,
    }
}

fn button_metrics(tokens: ButtonTokens, theme: &CastTheme, size: Size) -> ControlMetrics {
    match size {
        Size::Small => ControlMetrics {
            min_height: tokens.min_height - 6.0,
            padding: Vec2::new(theme.spacing.sm, theme.spacing.xs),
            text_size: theme.typography.small.size,
        },
        Size::Medium => ControlMetrics {
            min_height: tokens.min_height,
            padding: Vec2::new(tokens.padding_x, tokens.padding_y),
            text_size: theme.typography.body.size,
        },
        Size::Large => ControlMetrics {
            min_height: tokens.min_height + 8.0,
            padding: Vec2::new(theme.spacing.lg, theme.spacing.sm),
            text_size: theme.typography.body.size + 1.0,
        },
    }
}

pub(crate) fn card_frame(theme: &CastTheme) -> egui::Frame {
    let tokens = theme.components.card;
    egui::Frame::new()
        .fill(tokens.fill)
        .stroke(Stroke::new(tokens.border_width, tokens.border))
        .corner_radius(egui::CornerRadius::same(tokens.radius as u8))
        .shadow(surface_shadow(theme, 0.55, 10, 0, 2))
        .outer_margin(Margin::symmetric(1, 2))
        .inner_margin(Margin::same(tokens.padding as i8))
}

pub(crate) fn panel_frame(theme: &CastTheme) -> egui::Frame {
    let tokens = theme.components.panel;
    egui::Frame::new()
        .fill(tokens.fill)
        .stroke(Stroke::new(tokens.border_width, tokens.border))
        .corner_radius(egui::CornerRadius::same(tokens.radius as u8))
        .shadow(surface_shadow(theme, 0.28, 6, 0, 1))
        .outer_margin(Margin::symmetric(0, 1))
        .inner_margin(Margin::same(tokens.padding as i8))
}

pub(crate) fn input_frame(theme: &CastTheme, variant: Variant) -> egui::Frame {
    let tokens = theme.components.input;
    let (fill, border) = match variant {
        Variant::Solid => (tokens.fill, tokens.border),
        Variant::Subtle => (theme.colors.surface_muted, tokens.border),
        Variant::Outline => (Color32::TRANSPARENT, tokens.border),
        Variant::Ghost => (Color32::TRANSPARENT, Color32::TRANSPARENT),
    };

    egui::Frame::new()
        .fill(fill)
        .stroke(Stroke::new(tokens.border_width, border))
        .corner_radius(egui::CornerRadius::same(tokens.radius as u8))
        .inner_margin(Margin::symmetric(
            tokens.padding_x as i8,
            tokens.padding_y as i8,
        ))
}

pub(crate) fn alert_frame(theme: &CastTheme, border: Color32) -> egui::Frame {
    let tokens = theme.components.alert;
    egui::Frame::new()
        .stroke(Stroke::new(tokens.border_width, border))
        .corner_radius(egui::CornerRadius::same(tokens.radius as u8))
        .inner_margin(Margin::same(tokens.padding as i8))
}

pub(crate) fn alert_intent_colors(theme: &CastTheme, intent: Intent) -> IntentColors {
    if intent == Intent::Neutral {
        return IntentColors {
            fill: theme.colors.surface_muted,
            fg: theme.colors.text,
            border: theme.colors.border,
        };
    }

    let family = semantic_family(theme, intent);
    IntentColors {
        fill: mix_with_transparent(family.base, 0.05),
        fg: family.base,
        border: mix_with_transparent(family.base, 0.30),
    }
}

fn surface_shadow(
    theme: &CastTheme,
    opacity: f32,
    blur: u8,
    spread: u8,
    offset_y: i8,
) -> egui::epaint::Shadow {
    egui::epaint::Shadow {
        offset: [0, offset_y],
        blur,
        spread,
        color: with_alpha(
            Color32::BLACK,
            (f32::from(theme.elevation.shadow_alpha) * opacity).round() as u8,
        ),
    }
}

fn semantic_family(theme: &CastTheme, intent: Intent) -> SemanticColorTokens {
    match intent {
        Intent::Neutral => unreachable!("neutral intent has no semantic family"),
        Intent::Primary => theme.colors.primary_family,
        Intent::Secondary => theme.colors.secondary_family,
        Intent::Success => theme.colors.success_family,
        Intent::Warning => theme.colors.warning_family,
        Intent::Danger => theme.colors.danger_family,
        Intent::Info => theme.colors.info_family,
    }
}

fn neutral_intent_colors(theme: &CastTheme, variant: Variant) -> IntentColors {
    match variant {
        Variant::Solid => IntentColors {
            fill: theme.colors.surface_muted,
            fg: theme.colors.text,
            border: theme.colors.border,
        },
        Variant::Subtle => IntentColors {
            fill: theme.colors.surface_muted,
            fg: theme.colors.text_muted,
            border: theme.colors.border,
        },
        Variant::Outline => IntentColors {
            fill: Color32::TRANSPARENT,
            fg: theme.colors.text,
            border: theme.colors.border,
        },
        Variant::Ghost => IntentColors {
            fill: Color32::TRANSPARENT,
            fg: theme.colors.text,
            border: Color32::TRANSPARENT,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_solid_uses_theme_primary_pair() {
        let theme = CastTheme::light();
        let colors = resolve_intent_colors(&theme, Intent::Primary, Variant::Solid);

        assert_eq!(colors.fill, theme.colors.primary_family.base);
        assert_eq!(colors.fg, theme.colors.primary_family.fg);
        assert_eq!(colors.border, theme.colors.primary_family.base);
    }

    #[test]
    fn ghost_variant_has_no_fill_or_border() {
        let theme = CastTheme::light();
        let colors = resolve_intent_colors(&theme, Intent::Danger, Variant::Ghost);

        assert_eq!(colors.fill, Color32::TRANSPARENT);
        assert_eq!(colors.fg, theme.colors.danger_family.base);
        assert_eq!(colors.border, Color32::TRANSPARENT);
    }

    #[test]
    fn control_metrics_scale_by_size() {
        let theme = CastTheme::light();
        let small = resolve_control_metrics(&theme, Size::Small);
        let medium = resolve_control_metrics(&theme, Size::Medium);
        let large = resolve_control_metrics(&theme, Size::Large);

        assert!(small.min_height < medium.min_height);
        assert!(medium.min_height < large.min_height);
    }

    #[test]
    fn badge_metrics_use_badge_tokens_for_small_size() {
        let theme = CastTheme::light();
        let metrics = resolve_badge_metrics(&theme, Size::Small);

        assert_eq!(metrics.min_height, theme.components.badge.min_height);
        assert_eq!(metrics.padding.x, theme.components.badge.padding_x);
        assert_eq!(metrics.padding.y, theme.components.badge.padding_y);
    }

    #[test]
    fn badge_metrics_scale_from_badge_tokens() {
        let mut seed = crate::ThemeSeed::for_mode(crate::ThemeMode::Light);
        seed.component_overrides.badge.min_height = Some(20.0);
        seed.component_overrides.badge.padding_x = Some(10.0);
        let theme = seed.resolve();

        let small = resolve_badge_metrics(&theme, Size::Small);
        let medium = resolve_badge_metrics(&theme, Size::Medium);
        let large = resolve_badge_metrics(&theme, Size::Large);

        assert_eq!(small.min_height, 20.0);
        assert_eq!(small.padding.x, 10.0);
        assert_eq!(medium.min_height, 26.0);
        assert_eq!(large.min_height, 34.0);
    }

    #[test]
    fn ghost_input_frame_has_no_fill_or_border() {
        let theme = CastTheme::light();
        let frame = input_frame(&theme, Variant::Ghost);

        assert_eq!(frame.fill, Color32::TRANSPARENT);
        assert_eq!(frame.stroke.color, Color32::TRANSPARENT);
    }

    #[test]
    fn surface_frames_have_subtle_elevation() {
        let theme = CastTheme::light();
        let card = card_frame(&theme);
        let panel = panel_frame(&theme);

        assert!(card.shadow.blur > panel.shadow.blur);
        assert!(card.shadow.color.a() > panel.shadow.color.a());
    }

    #[test]
    fn alert_colors_use_transparent_semantic_tints() {
        let theme = CastTheme::light();
        let colors = alert_intent_colors(&theme, Intent::Success);
        let [_, _, _, fill_alpha] = colors.fill.to_srgba_unmultiplied();
        let [_, _, _, border_alpha] = colors.border.to_srgba_unmultiplied();

        assert_eq!(fill_alpha, 13);
        assert_eq!(border_alpha, 77);
        assert_eq!(colors.fg, theme.colors.success_family.base);
    }
}
