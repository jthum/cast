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
pub struct Loader {
    intent: Intent,
    size: Size,
    style: LoaderStyle,
}

impl Loader {
    #[must_use]
    pub fn new() -> Self {
        Self {
            intent: Intent::Primary,
            size: Size::Medium,
            style: LoaderStyle::Ticks,
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
    pub fn style(mut self, style: LoaderStyle) -> Self {
        self.style = style;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LoaderStyle {
    #[default]
    Ticks,
    Signal,
    PixelEqualizer,
    PulseGrid,
}

pub type Spinner = Loader;
pub type SpinnerStyle = LoaderStyle;

impl Default for Loader {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Loader {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let side = loader_side(self.size);
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(side), Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(16));
            match self.style {
                LoaderStyle::Ticks => paint_tick_loader(ui, &theme, rect, self.intent),
                LoaderStyle::Signal => paint_signal_loader(ui, &theme, rect, self.intent),
                LoaderStyle::PixelEqualizer => paint_pixel_equalizer_loader(ui, rect, self.intent),
                LoaderStyle::PulseGrid => paint_pulse_grid_loader(ui, rect, self.intent),
            }
        }

        response
    }
}

fn paint_tick_loader(ui: &Ui, theme: &CastTheme, rect: egui::Rect, intent: Intent) {
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

fn paint_signal_loader(ui: &Ui, theme: &CastTheme, rect: egui::Rect, intent: Intent) {
    let center = rect.center();
    let side = rect.width().min(rect.height());
    let accent = progress_accent(theme, intent);
    let time = ui.input(|input| input.time) as f32;
    let pulse = ((time * std::f32::consts::TAU * 1.2).sin() + 1.0) * 0.5;
    let orbit = time * std::f32::consts::TAU * 0.72;
    let orbit_radius = side * (0.28 + pulse * 0.035);

    ui.painter().circle_filled(
        center,
        side * (0.26 + pulse * 0.035),
        color_with_scaled_alpha(accent, 0.12 + pulse * 0.08),
    );
    ui.painter().circle_stroke(
        center,
        side * 0.36,
        Stroke::new(theme.stroke.sm, color_with_scaled_alpha(accent, 0.28)),
    );

    for index in 0..3 {
        let angle = orbit + (index as f32 * std::f32::consts::TAU / 3.0);
        let dot_center = center + egui::vec2(angle.cos(), angle.sin()) * orbit_radius;
        let phase = ((time * std::f32::consts::TAU * 1.6) + index as f32 * 1.3).sin();
        let radius = side * (0.075 + (phase + 1.0) * 0.012);
        ui.painter().circle_filled(
            dot_center,
            radius,
            color_with_scaled_alpha(accent, 0.62 + index as f32 * 0.12),
        );
    }
}

fn paint_pixel_equalizer_loader(ui: &Ui, rect: egui::Rect, intent: Intent) {
    let theme = theme_for_ui(ui);
    let accent = progress_accent(&theme, intent);
    let time = ui.input(|input| input.time) as f32;
    let side = rect.width().min(rect.height());
    let columns = 4;
    let rows = 5;
    let outer_width = side * 0.76;
    let outer_height = side * 0.82;
    let pitch = (outer_width / columns as f32).min(outer_height / rows as f32);
    let cell = pitch * 0.66;
    let origin = egui::pos2(
        rect.center().x - (pitch * columns as f32) / 2.0 + (pitch - cell) / 2.0,
        rect.center().y - (pitch * rows as f32) / 2.0 + (pitch - cell) / 2.0,
    );

    for column in 0..columns {
        let phase = time * 5.2 + column as f32 * 1.18;
        let wave = (phase.sin() + 1.0) * 0.5;
        let active_rows = 1 + (wave * (rows - 1) as f32).round() as usize;

        for level in 0..active_rows {
            let row = rows - 1 - level;
            let cell_rect = pixel_cell(origin, pitch, cell, column, row);
            let level_alpha = level as f32 / rows as f32;
            let alpha = 0.22 + level_alpha * 0.38 + wave * 0.28;
            ui.painter().rect_filled(
                cell_rect,
                egui::CornerRadius::ZERO,
                color_with_scaled_alpha(accent, alpha),
            );
        }
    }
}

fn paint_pulse_grid_loader(ui: &Ui, rect: egui::Rect, intent: Intent) {
    let theme = theme_for_ui(ui);
    let accent = progress_accent(&theme, intent);
    let time = ui.input(|input| input.time) as f32;
    let grid = 3;
    let side = rect.width().min(rect.height()) * 0.74;
    let pitch = side / grid as f32;
    let cell = pitch * 0.68;
    let origin = egui::pos2(
        rect.center().x - side / 2.0 + (pitch - cell) / 2.0,
        rect.center().y - side / 2.0 + (pitch - cell) / 2.0,
    );

    for row in 0..grid {
        for column in 0..grid {
            let diagonal = row + column;
            let phase = time * 5.0 - diagonal as f32 * 0.52;
            let wave = (phase.sin() + 1.0) * 0.5;
            let center_bias = if row == 1 && column == 1 { 0.16 } else { 0.0 };
            let alpha = 0.16 + center_bias + wave * 0.68;
            ui.painter().rect_filled(
                pixel_cell(origin, pitch, cell, column, row),
                egui::CornerRadius::ZERO,
                color_with_scaled_alpha(accent, alpha),
            );
        }
    }
}

fn pixel_cell(origin: egui::Pos2, pitch: f32, size: f32, column: usize, row: usize) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(
            origin.x + column as f32 * pitch,
            origin.y + row as f32 * pitch,
        ),
        Vec2::splat(size),
    )
}

fn progress_height(size: Size) -> f32 {
    match size {
        Size::Small => 6.0,
        Size::Medium => 8.0,
        Size::Large => 10.0,
    }
}

fn loader_side(size: Size) -> f32 {
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
    fn loader_sizes_scale() {
        assert!(loader_side(Size::Small) < loader_side(Size::Medium));
        assert!(loader_side(Size::Medium) < loader_side(Size::Large));
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

    #[test]
    fn loader_defaults_to_tick_style() {
        assert_eq!(Loader::new().style, LoaderStyle::Ticks);
    }

    #[test]
    fn loader_style_can_be_changed() {
        assert_eq!(
            Loader::new().style(LoaderStyle::PulseGrid).style,
            LoaderStyle::PulseGrid
        );
    }

    #[test]
    fn spinner_aliases_remain_available() {
        assert_eq!(
            Spinner::new().style(SpinnerStyle::PixelEqualizer).style,
            LoaderStyle::PixelEqualizer
        );
    }
}
