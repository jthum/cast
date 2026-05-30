use egui::{Color32, Response, RichText, StrokeKind, TextEdit, Ui, Widget};

use crate::{
    color::mix_with_transparent,
    foundation::{Intent, Size, Variant},
    style::{input_frame, resolve_control_metrics},
    theme::{CastTheme, theme_for_ui},
};

#[derive(Debug)]
pub struct TextInput<'a> {
    text: &'a mut String,
    label: Option<String>,
    hint_text: Option<String>,
    help_text: Option<String>,
    status_text: Option<String>,
    status_intent: Option<Intent>,
    width: Option<f32>,
    size: Option<Size>,
    variant: Variant,
    enabled: bool,
}

impl<'a> TextInput<'a> {
    #[must_use]
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            label: None,
            hint_text: None,
            help_text: None,
            status_text: None,
            status_intent: None,
            width: None,
            size: None,
            variant: Variant::Solid,
            enabled: true,
        }
    }

    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    #[must_use]
    pub fn hint_text(mut self, hint_text: impl Into<String>) -> Self {
        self.hint_text = Some(hint_text.into());
        self
    }

    #[must_use]
    pub fn help_text(mut self, help_text: impl Into<String>) -> Self {
        self.help_text = Some(help_text.into());
        self
    }

    #[must_use]
    pub fn status_text(mut self, intent: Intent, status_text: impl Into<String>) -> Self {
        self.status_intent = Some(intent);
        self.status_text = Some(status_text.into());
        self
    }

    #[must_use]
    pub fn success_text(self, status_text: impl Into<String>) -> Self {
        self.status_text(Intent::Success, status_text)
    }

    #[must_use]
    pub fn warning_text(self, status_text: impl Into<String>) -> Self {
        self.status_text(Intent::Warning, status_text)
    }

    #[must_use]
    pub fn error_text(self, status_text: impl Into<String>) -> Self {
        self.status_text(Intent::Danger, status_text)
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    #[must_use]
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
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
}

impl Widget for TextInput<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let size = self
            .size
            .or_else(|| crate::style::contextual_control_size(ui))
            .unwrap_or(Size::Medium);
        let metrics = resolve_control_metrics(&theme, size);
        let mut font = theme.typography.body.clone();
        font.size = metrics.text_size;
        let label = self.label;
        let help_text = self.help_text;
        let status_text = self.status_text;
        let status_intent = self.status_intent;
        let enabled = self.enabled;
        let input_radius = egui::CornerRadius::same(theme.components.input.radius as u8);
        let mut edit = TextEdit::singleline(self.text)
            .frame(input_frame(&theme, self.variant))
            .font(font.clone())
            .min_size(egui::vec2(0.0, input_min_height(&theme, size, metrics)))
            .text_color(if enabled {
                theme.components.input.fg
            } else {
                theme.colors.text_subtle
            });

        if let Some(hint_text) = self.hint_text {
            edit = edit.hint_text(
                RichText::new(hint_text)
                    .font(font.clone())
                    .color(theme.components.input.placeholder)
                    .extra_letter_spacing(theme.typography.letter_spacing),
            );
        }

        if let Some(width) = self.width {
            edit = edit.desired_width(width);
        }

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = theme.spacing.xs;

            if let Some(label) = label {
                ui.label(
                    RichText::new(label)
                        .font(theme.typography.label.clone())
                        .color(if enabled {
                            theme.colors.text
                        } else {
                            theme.colors.text_subtle
                        })
                        .extra_letter_spacing(theme.typography.letter_spacing),
                );
            }

            let response = ui.add_enabled(enabled, edit);
            paint_input_state(ui, &response, input_radius, enabled, status_intent);

            if let Some(message) = status_text.or(help_text) {
                let color = status_intent.map_or(theme.colors.text_muted, |intent| {
                    status_color(&theme, intent)
                });
                ui.label(
                    RichText::new(message)
                        .font(theme.typography.small.clone())
                        .color(color)
                        .extra_letter_spacing(theme.typography.letter_spacing),
                );
            }

            response
        })
        .inner
    }
}

#[derive(Debug)]
pub struct TextArea<'a> {
    text: &'a mut String,
    hint_text: Option<String>,
    width: Option<f32>,
    rows: usize,
    size: Option<Size>,
    variant: Variant,
    enabled: bool,
    status_intent: Option<Intent>,
}

impl<'a> TextArea<'a> {
    #[must_use]
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            hint_text: None,
            width: None,
            rows: 4,
            size: None,
            variant: Variant::Solid,
            enabled: true,
            status_intent: None,
        }
    }

    #[must_use]
    pub fn hint_text(mut self, hint_text: impl Into<String>) -> Self {
        self.hint_text = Some(hint_text.into());
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    #[must_use]
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows.max(2);
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    #[must_use]
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    #[must_use]
    pub fn status(mut self, intent: Intent) -> Self {
        self.status_intent = Some(intent);
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
}

impl Widget for TextArea<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let size = self
            .size
            .or_else(|| crate::style::contextual_control_size(ui))
            .unwrap_or(Size::Medium);
        let metrics = resolve_control_metrics(&theme, size);
        let mut font = theme.typography.body.clone();
        font.size = metrics.text_size;
        let enabled = self.enabled;
        let input_radius = egui::CornerRadius::same(theme.components.input.radius as u8);
        let mut edit = TextEdit::multiline(self.text)
            .frame(input_frame(&theme, self.variant))
            .font(font.clone())
            .desired_rows(self.rows)
            .min_size(egui::vec2(
                0.0,
                text_area_min_height(&theme, size, metrics, self.rows),
            ))
            .text_color(if enabled {
                theme.components.input.fg
            } else {
                theme.colors.text_subtle
            });

        if let Some(hint_text) = self.hint_text {
            edit = edit.hint_text(
                RichText::new(hint_text)
                    .font(font)
                    .color(theme.components.input.placeholder)
                    .extra_letter_spacing(theme.typography.letter_spacing),
            );
        }

        if let Some(width) = self.width {
            edit = edit.desired_width(width);
        }

        let response = ui.add_enabled(enabled, edit);
        paint_input_state(ui, &response, input_radius, enabled, self.status_intent);

        response
    }
}

fn paint_input_state(
    ui: &Ui,
    response: &Response,
    radius: egui::CornerRadius,
    enabled: bool,
    status: Option<Intent>,
) {
    let theme = theme_for_ui(ui);
    let focused = enabled && response.has_focus();
    let hovered = enabled && response.hovered();

    if let Some(halo) = input_interaction_halo(&theme, status, focused, hovered) {
        ui.painter()
            .rect_stroke(response.rect, radius, halo, StrokeKind::Outside);
    }

    let stroke = status
        .map(|status| egui::Stroke::new(input_border_width(&theme), status_color(&theme, status)))
        .or_else(|| input_interaction_border(&theme, focused, hovered));

    if let Some(stroke) = stroke {
        ui.painter()
            .rect_stroke(response.rect, radius, stroke, StrokeKind::Outside);
    }
}

fn input_interaction_halo(
    theme: &CastTheme,
    status: Option<Intent>,
    focused: bool,
    hovered: bool,
) -> Option<egui::Stroke> {
    let color = status.map_or(theme.colors.primary_family.base, |status| {
        status_color(theme, status)
    });

    if focused {
        Some(egui::Stroke::new(3.0, mix_with_transparent(color, 0.14)))
    } else if hovered || status.is_some() {
        Some(egui::Stroke::new(2.0, mix_with_transparent(color, 0.09)))
    } else {
        None
    }
}

fn input_interaction_border(
    theme: &CastTheme,
    focused: bool,
    hovered: bool,
) -> Option<egui::Stroke> {
    if focused || hovered {
        Some(egui::Stroke::new(
            input_border_width(theme),
            theme.colors.border_strong,
        ))
    } else {
        None
    }
}

fn input_border_width(theme: &CastTheme) -> f32 {
    theme.components.input.border_width.max(1.0)
}

fn input_min_height(theme: &CastTheme, size: Size, metrics: crate::style::ControlMetrics) -> f32 {
    if size == Size::Small {
        (metrics.min_height - theme.components.input.padding_y * 2.0).max(metrics.text_size)
    } else {
        metrics.min_height.max(theme.components.input.min_height)
    }
}

fn text_area_min_height(
    theme: &CastTheme,
    size: Size,
    metrics: crate::style::ControlMetrics,
    rows: usize,
) -> f32 {
    let row_height = metrics.text_size * 1.35;
    let padding = theme.components.input.padding_y * 2.0;
    let min_height = row_height * rows.max(2) as f32 + padding;

    min_height.max(input_min_height(theme, size, metrics))
}

fn status_color(theme: &CastTheme, intent: Intent) -> Color32 {
    match intent {
        Intent::Neutral => theme.colors.border_strong,
        Intent::Primary => theme.colors.primary_family.base,
        Intent::Secondary => theme.colors.secondary_family.base,
        Intent::Success => theme.colors.success_family.base,
        Intent::Warning => theme.colors.warning_family.base,
        Intent::Danger => theme.colors.danger_family.base,
        Intent::Info => theme.colors.info_family.base,
    }
}

#[derive(Debug)]
pub struct SearchInput<'a> {
    inner: TextInput<'a>,
}

impl<'a> SearchInput<'a> {
    #[must_use]
    pub fn new(text: &'a mut String) -> Self {
        Self {
            inner: TextInput::new(text).hint_text("Search"),
        }
    }

    #[must_use]
    pub fn hint_text(mut self, hint_text: impl Into<String>) -> Self {
        self.inner = self.inner.hint_text(hint_text);
        self
    }

    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.inner = self.inner.label(label);
        self
    }

    #[must_use]
    pub fn help_text(mut self, help_text: impl Into<String>) -> Self {
        self.inner = self.inner.help_text(help_text);
        self
    }

    #[must_use]
    pub fn status_text(mut self, intent: Intent, status_text: impl Into<String>) -> Self {
        self.inner = self.inner.status_text(intent, status_text);
        self
    }

    #[must_use]
    pub fn success_text(mut self, status_text: impl Into<String>) -> Self {
        self.inner = self.inner.success_text(status_text);
        self
    }

    #[must_use]
    pub fn warning_text(mut self, status_text: impl Into<String>) -> Self {
        self.inner = self.inner.warning_text(status_text);
        self
    }

    #[must_use]
    pub fn error_text(mut self, status_text: impl Into<String>) -> Self {
        self.inner = self.inner.error_text(status_text);
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.inner = self.inner.width(width);
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.inner = self.inner.size(size);
        self
    }

    #[must_use]
    pub fn variant(mut self, variant: Variant) -> Self {
        self.inner = self.inner.variant(variant);
        self
    }

    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.inner = self.inner.enabled(enabled);
        self
    }

    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.inner = self.inner.disabled();
        self
    }
}

impl Widget for SearchInput<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.inner.ui(ui)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_input_status_helpers_set_intent_and_message() {
        let mut value = String::new();
        let input = TextInput::new(&mut value).error_text("Required field");

        assert_eq!(input.status_intent, Some(Intent::Danger));
        assert_eq!(input.status_text.as_deref(), Some("Required field"));
    }

    #[test]
    fn search_input_can_carry_field_metadata() {
        let mut value = String::new();
        let search = SearchInput::new(&mut value)
            .label("Search")
            .help_text("Filters the current view");

        assert_eq!(search.inner.label.as_deref(), Some("Search"));
        assert_eq!(
            search.inner.help_text.as_deref(),
            Some("Filters the current view")
        );
    }

    #[test]
    fn text_input_size_is_optional_for_contextual_sizing() {
        let mut value = String::new();
        let input = TextInput::new(&mut value);

        assert_eq!(input.size, None);
    }

    #[test]
    fn text_area_rows_have_floor() {
        let mut value = String::new();
        let area = TextArea::new(&mut value).rows(1);

        assert_eq!(area.rows, 2);
    }

    #[test]
    fn text_area_min_height_scales_with_rows() {
        let theme = CastTheme::light();
        let metrics = resolve_control_metrics(&theme, Size::Medium);

        assert!(
            text_area_min_height(&theme, Size::Medium, metrics, 5)
                > text_area_min_height(&theme, Size::Medium, metrics, 2)
        );
    }

    #[test]
    fn small_text_inputs_match_small_control_height() {
        let theme = CastTheme::light();
        let metrics = resolve_control_metrics(&theme, Size::Small);

        assert_eq!(
            input_min_height(&theme, Size::Small, metrics),
            (metrics.min_height - theme.components.input.padding_y * 2.0).max(metrics.text_size)
        );
    }

    #[test]
    fn input_hover_and_focus_use_muted_chrome() {
        let theme = CastTheme::light();
        let hover = input_interaction_border(&theme, false, true).unwrap();
        let focus = input_interaction_border(&theme, true, true).unwrap();

        assert_eq!(hover.color, theme.colors.border_strong);
        assert_eq!(focus.color, theme.colors.border_strong);
        assert_eq!(hover.width, input_border_width(&theme));
        assert_eq!(focus.width, input_border_width(&theme));
    }

    #[test]
    fn input_halo_uses_subtle_primary_tints() {
        let theme = CastTheme::light();
        let hover = input_interaction_halo(&theme, None, false, true).unwrap();
        let focus = input_interaction_halo(&theme, None, true, false).unwrap();
        let [_, _, _, hover_alpha] = hover.color.to_srgba_unmultiplied();
        let [_, _, _, focus_alpha] = focus.color.to_srgba_unmultiplied();

        assert_eq!(
            hover.color,
            mix_with_transparent(theme.colors.primary_family.base, 0.09)
        );
        assert_eq!(
            focus.color,
            mix_with_transparent(theme.colors.primary_family.base, 0.14)
        );
        assert_eq!(hover_alpha, 23);
        assert_eq!(focus_alpha, 36);
    }

    #[test]
    fn status_inputs_use_semantic_halo_and_stable_border_width() {
        let theme = CastTheme::light();
        let halo = input_interaction_halo(&theme, Some(Intent::Danger), false, false).unwrap();
        let border = egui::Stroke::new(
            input_border_width(&theme),
            status_color(&theme, Intent::Danger),
        );
        let [halo_r, _, _, halo_alpha] = halo.color.to_srgba_unmultiplied();

        assert!((i16::from(halo_r) - i16::from(theme.colors.danger_family.base.r())).abs() <= 3);
        assert_eq!(halo_alpha, 23);
        assert_eq!(border.width, input_border_width(&theme));
    }
}
