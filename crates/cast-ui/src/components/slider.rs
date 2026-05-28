use std::ops::RangeInclusive;

use egui::{Color32, Response, Sense, StrokeKind, Ui, Widget};

use crate::{
    color::{mix_with_transparent, with_alpha},
    foundation::Size,
    theme::{CastTheme, ThemeMode, theme_for_ui},
};

#[derive(Debug)]
pub struct Slider<'a> {
    value: &'a mut f32,
    range: RangeInclusive<f32>,
    label: Option<String>,
    show_value: bool,
    width: Option<f32>,
    size: Size,
    enabled: bool,
}

impl<'a> Slider<'a> {
    #[must_use]
    pub fn new(value: &'a mut f32, range: RangeInclusive<f32>) -> Self {
        Self {
            value,
            range,
            label: None,
            show_value: true,
            width: None,
            size: Size::Medium,
            enabled: true,
        }
    }

    #[must_use]
    pub fn text(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    #[must_use]
    pub fn show_value(mut self, show_value: bool) -> Self {
        self.show_value = show_value;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
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
}

impl Widget for Slider<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        if self.label.is_some() || self.show_value {
            ui.vertical(|ui| {
                slider_header(ui, self.label.as_deref(), *self.value, self.show_value);
                slider_track(
                    ui,
                    self.value,
                    self.range,
                    self.width,
                    self.size,
                    self.enabled,
                )
            })
            .inner
        } else {
            slider_track(
                ui,
                self.value,
                self.range,
                self.width,
                self.size,
                self.enabled,
            )
        }
    }
}

fn slider_header(ui: &mut Ui, label: Option<&str>, value: f32, show_value: bool) {
    let theme = theme_for_ui(ui);
    ui.horizontal(|ui| {
        if let Some(label) = label {
            ui.label(
                egui::RichText::new(label)
                    .font(theme.typography.label.clone())
                    .color(theme.colors.text)
                    .extra_letter_spacing(theme.typography.letter_spacing),
            );
        }
        if show_value {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(format_slider_value(value))
                        .font(theme.typography.caption.clone())
                        .color(theme.colors.text_muted)
                        .extra_letter_spacing(theme.typography.letter_spacing),
                );
            });
        }
    });
}

fn slider_track(
    ui: &mut Ui,
    value: &mut f32,
    range: RangeInclusive<f32>,
    width: Option<f32>,
    size: Size,
    enabled: bool,
) -> Response {
    let available_width = ui.available_width().max(96.0);
    let width = width
        .unwrap_or(available_width)
        .min(available_width)
        .max(72.0);
    let height = match size {
        Size::Small => 22.0,
        Size::Medium => 26.0,
        Size::Large => 30.0,
    };
    let (rect, mut response) = ui.allocate_exact_size(
        egui::vec2(width, height),
        if enabled {
            Sense::click_and_drag()
        } else {
            Sense::hover()
        },
    );

    if enabled
        && (response.clicked() || response.dragged())
        && let Some(pointer) = response.interact_pointer_pos()
    {
        let next = value_from_pointer(pointer.x, rect, &range);
        if (*value - next).abs() > f32::EPSILON {
            *value = next;
            response.mark_changed();
        }
    }

    if ui.is_rect_visible(rect) {
        paint_slider(ui, rect, *value, &range, enabled, &response);
    }

    response
}

fn paint_slider(
    ui: &Ui,
    rect: egui::Rect,
    value: f32,
    range: &RangeInclusive<f32>,
    enabled: bool,
    response: &Response,
) {
    let theme = theme_for_ui(ui);
    let track_height = 4.0;
    let thumb_radius = 7.0;
    let track_rect = egui::Rect::from_center_size(
        rect.center(),
        egui::vec2(rect.width() - thumb_radius * 2.0, track_height),
    );
    let amount = normalized_value(value, range);
    let thumb_x = egui::lerp(track_rect.x_range(), amount);
    let active_rect =
        egui::Rect::from_min_max(track_rect.min, egui::pos2(thumb_x, track_rect.max.y));
    let radius = egui::CornerRadius::same((track_height / 2.0) as u8);
    let primary = theme.colors.primary_family.base;
    let inactive_fill = if enabled {
        theme.colors.surface_muted
    } else {
        with_alpha(theme.colors.surface_muted, 150)
    };
    let active_fill = if enabled {
        primary
    } else {
        mix_with_transparent(primary, 0.40)
    };

    ui.painter().rect(
        track_rect,
        radius,
        inactive_fill,
        egui::Stroke::new(theme.stroke.sm, theme.colors.border),
        StrokeKind::Outside,
    );
    ui.painter().rect_filled(active_rect, radius, active_fill);

    let center = egui::pos2(thumb_x, rect.center().y);
    let hovered = enabled && response.hovered();
    let dragged = enabled && response.dragged();
    let halo_alpha = if dragged {
        0.24
    } else if hovered {
        0.16
    } else {
        0.0
    };

    if halo_alpha > 0.0 {
        ui.painter().circle_filled(
            center,
            thumb_radius + if dragged { 7.0 } else { 5.0 },
            mix_with_transparent(primary, halo_alpha),
        );
    }

    ui.painter().circle_filled(
        center + egui::vec2(0.0, 1.0),
        thumb_radius,
        with_alpha(Color32::BLACK, theme.elevation.shadow_alpha / 2),
    );
    ui.painter()
        .circle_filled(center, thumb_radius, slider_thumb_fill(&theme));
    ui.painter().circle_stroke(
        center,
        thumb_radius,
        egui::Stroke::new(
            theme.stroke.sm,
            if hovered || dragged {
                mix_with_transparent(primary, 0.44)
            } else {
                mix_with_transparent(primary, 0.30)
            },
        ),
    );
}

fn slider_thumb_fill(theme: &CastTheme) -> Color32 {
    match theme.mode {
        ThemeMode::Light => theme.colors.surface,
        ThemeMode::Dark => theme.colors.text,
    }
}

fn normalized_value(value: f32, range: &RangeInclusive<f32>) -> f32 {
    let start = *range.start();
    let end = *range.end();
    if (end - start).abs() <= f32::EPSILON {
        0.0
    } else {
        ((value - start) / (end - start)).clamp(0.0, 1.0)
    }
}

fn value_from_pointer(x: f32, rect: egui::Rect, range: &RangeInclusive<f32>) -> f32 {
    let start = *range.start();
    let end = *range.end();
    let amount = ((x - rect.min.x) / rect.width()).clamp(0.0, 1.0);
    start + (end - start) * amount
}

fn format_slider_value(value: f32) -> String {
    if value.abs() >= 10.0 {
        format!("{value:.0}")
    } else {
        format!("{value:.2}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalized_value_clamps_to_range() {
        assert_eq!(normalized_value(5.0, &(0.0..=10.0)), 0.5);
        assert_eq!(normalized_value(-1.0, &(0.0..=10.0)), 0.0);
        assert_eq!(normalized_value(11.0, &(0.0..=10.0)), 1.0);
    }

    #[test]
    fn slider_value_format_is_compact() {
        assert_eq!(format_slider_value(14.2), "14");
        assert_eq!(format_slider_value(0.75), "0.75");
    }

    #[test]
    fn slider_thumb_uses_bright_fill_in_dark_mode() {
        let theme = CastTheme::dark();

        assert_eq!(slider_thumb_fill(&theme), theme.colors.text);
    }
}
