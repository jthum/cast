use egui::{Color32, Response, Sense, Stroke, Ui, Vec2, Widget};

use crate::{
    color::mix_with_transparent,
    foundation::{Intent, Size},
    theme::{CastTheme, theme_for_ui},
};

#[derive(Clone, Debug)]
pub struct ProgressBar {
    value: f32,
    intent: Intent,
    size: Size,
    width: Option<f32>,
}

impl ProgressBar {
    #[must_use]
    pub fn new(value: f32) -> Self {
        Self {
            value,
            intent: Intent::Primary,
            size: Size::Medium,
            width: None,
        }
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(48.0));
        self
    }
}

impl Widget for ProgressBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let height = progress_height(self.size);
        let width = self.width.unwrap_or_else(|| ui.available_width().max(96.0));
        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

        if ui.is_rect_visible(rect) {
            let radius = egui::CornerRadius::same((height / 2.0).round() as u8);
            let track = progress_track_color(&theme);
            let accent = progress_accent(&theme, self.intent);
            ui.painter().rect_filled(rect, radius, track);

            let fill_width = rect.width() * self.value.clamp(0.0, 1.0);
            if fill_width > 0.0 {
                let fill_rect = egui::Rect::from_min_size(
                    rect.min,
                    egui::vec2(fill_width.max(height), rect.height()),
                )
                .intersect(rect);
                ui.painter().rect_filled(fill_rect, radius, accent);
            }
        }

        response
    }
}

#[derive(Clone, Debug)]
pub struct Spinner {
    intent: Intent,
    size: Size,
}

impl Spinner {
    #[must_use]
    pub fn new() -> Self {
        Self {
            intent: Intent::Primary,
            size: Size::Medium,
        }
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Spinner {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let side = spinner_side(self.size);
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(side), Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(16));
            paint_spinner(ui, &theme, rect, self.intent);
        }

        response
    }
}

fn paint_spinner(ui: &Ui, theme: &CastTheme, rect: egui::Rect, intent: Intent) {
    let center = rect.center();
    let radius = rect.width().min(rect.height()) * 0.38;
    let accent = progress_accent(theme, intent);
    let time = ui.input(|input| input.time) as f32;
    let phase = (time * 10.0).round() as usize;

    for index in 0..12 {
        let angle = (index as f32 / 12.0) * std::f32::consts::TAU;
        let alpha_step = ((index + 12 - (phase % 12)) % 12) as f32 / 11.0;
        let alpha = (0.20 + alpha_step * 0.72).clamp(0.0, 1.0);
        let inner = center + egui::vec2(angle.cos(), angle.sin()) * (radius * 0.58);
        let outer = center + egui::vec2(angle.cos(), angle.sin()) * radius;
        ui.painter().line_segment(
            [inner, outer],
            Stroke::new(
                theme.stroke.md.max(1.6),
                color_with_scaled_alpha(accent, alpha),
            ),
        );
    }
}

fn progress_height(size: Size) -> f32 {
    match size {
        Size::Small => 6.0,
        Size::Medium => 8.0,
        Size::Large => 10.0,
    }
}

fn spinner_side(size: Size) -> f32 {
    match size {
        Size::Small => 16.0,
        Size::Medium => 20.0,
        Size::Large => 24.0,
    }
}

fn progress_track_color(theme: &CastTheme) -> Color32 {
    mix_with_transparent(theme.colors.text_muted, 0.18)
}

fn progress_accent(theme: &CastTheme, intent: Intent) -> Color32 {
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

fn color_with_scaled_alpha(color: Color32, alpha: f32) -> Color32 {
    let [r, g, b, a] = color.to_srgba_unmultiplied();
    Color32::from_rgba_unmultiplied(r, g, b, (f32::from(a) * alpha).round() as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_sizes_scale() {
        assert!(progress_height(Size::Small) < progress_height(Size::Medium));
        assert!(progress_height(Size::Medium) < progress_height(Size::Large));
    }

    #[test]
    fn spinner_sizes_scale() {
        assert!(spinner_side(Size::Small) < spinner_side(Size::Medium));
        assert!(spinner_side(Size::Medium) < spinner_side(Size::Large));
    }

    #[test]
    fn progress_accent_follows_intent() {
        let theme = CastTheme::light();

        assert_eq!(
            progress_accent(&theme, Intent::Success),
            theme.colors.success_family.base
        );
    }

    #[test]
    fn progress_width_has_floor() {
        assert_eq!(ProgressBar::new(0.5).width(20.0).width, Some(48.0));
    }
}
