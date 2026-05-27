use egui::{Color32, Margin, Stroke, Vec2};

use crate::{
    foundation::{Intent, Size, Variant},
    theme::CastTheme,
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
        stroke: Stroke::new(theme.stroke.sm, colors.border),
    }
}

pub(crate) fn resolve_intent_colors(
    theme: &CastTheme,
    intent: Intent,
    variant: Variant,
) -> IntentColors {
    let solid = solid_intent_colors(theme, intent);

    match variant {
        Variant::Solid => solid,
        Variant::Subtle => IntentColors {
            fill: subtle_fill(theme, intent),
            fg: solid.fill,
            border: subtle_fill(theme, intent),
        },
        Variant::Outline => IntentColors {
            fill: Color32::TRANSPARENT,
            fg: solid.fill,
            border: solid.fill,
        },
        Variant::Ghost => IntentColors {
            fill: Color32::TRANSPARENT,
            fg: solid.fill,
            border: Color32::TRANSPARENT,
        },
    }
}

pub(crate) fn resolve_control_metrics(theme: &CastTheme, size: Size) -> ControlMetrics {
    match size {
        Size::Small => ControlMetrics {
            min_height: theme.controls.min_height - 6.0,
            padding: Vec2::new(theme.spacing.sm, theme.spacing.xs),
            text_size: theme.typography.small.size,
        },
        Size::Medium => ControlMetrics {
            min_height: theme.controls.min_height,
            padding: Vec2::new(theme.controls.padding_x, theme.controls.padding_y),
            text_size: theme.typography.body.size,
        },
        Size::Large => ControlMetrics {
            min_height: theme.controls.min_height + 8.0,
            padding: Vec2::new(theme.spacing.lg, theme.spacing.sm),
            text_size: theme.typography.body.size + 1.0,
        },
    }
}

pub(crate) fn card_frame(theme: &CastTheme) -> egui::Frame {
    egui::Frame::new()
        .fill(theme.colors.surface)
        .stroke(Stroke::new(theme.stroke.sm, theme.colors.border))
        .inner_margin(Margin::same(theme.spacing.lg as i8))
}

pub(crate) fn input_frame(theme: &CastTheme) -> egui::Frame {
    egui::Frame::new()
        .fill(theme.colors.surface)
        .stroke(Stroke::new(theme.stroke.sm, theme.colors.border))
        .inner_margin(Margin::symmetric(
            theme.controls.padding_x as i8,
            theme.controls.padding_y as i8,
        ))
}

fn solid_intent_colors(theme: &CastTheme, intent: Intent) -> IntentColors {
    match intent {
        Intent::Neutral => IntentColors {
            fill: theme.colors.surface_muted,
            fg: theme.colors.text,
            border: theme.colors.border,
        },
        Intent::Primary => IntentColors {
            fill: theme.colors.primary,
            fg: theme.colors.primary_fg,
            border: theme.colors.primary,
        },
        Intent::Success => IntentColors {
            fill: theme.colors.success,
            fg: theme.colors.success_fg,
            border: theme.colors.success,
        },
        Intent::Warning => IntentColors {
            fill: theme.colors.warning,
            fg: theme.colors.warning_fg,
            border: theme.colors.warning,
        },
        Intent::Danger => IntentColors {
            fill: theme.colors.danger,
            fg: theme.colors.danger_fg,
            border: theme.colors.danger,
        },
        Intent::Info => IntentColors {
            fill: theme.colors.info,
            fg: theme.colors.info_fg,
            border: theme.colors.info,
        },
    }
}

fn subtle_fill(theme: &CastTheme, intent: Intent) -> Color32 {
    match intent {
        Intent::Neutral => theme.colors.surface_muted,
        Intent::Primary | Intent::Info => theme.colors.selection,
        Intent::Success | Intent::Warning | Intent::Danger => theme.colors.surface_muted,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_solid_uses_theme_primary_pair() {
        let theme = CastTheme::light();
        let colors = resolve_intent_colors(&theme, Intent::Primary, Variant::Solid);

        assert_eq!(colors.fill, theme.colors.primary);
        assert_eq!(colors.fg, theme.colors.primary_fg);
        assert_eq!(colors.border, theme.colors.primary);
    }

    #[test]
    fn ghost_variant_has_no_fill_or_border() {
        let theme = CastTheme::light();
        let colors = resolve_intent_colors(&theme, Intent::Danger, Variant::Ghost);

        assert_eq!(colors.fill, Color32::TRANSPARENT);
        assert_eq!(colors.fg, theme.colors.danger);
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
}
