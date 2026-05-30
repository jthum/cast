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
    style: SpinnerStyle,
}

impl Spinner {
    #[must_use]
    pub fn new() -> Self {
        Self {
            intent: Intent::Primary,
            size: Size::Medium,
            style: SpinnerStyle::Ticks,
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
    pub fn style(mut self, style: SpinnerStyle) -> Self {
        self.style = style;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SpinnerStyle {
    #[default]
    Ticks,
    Signal,
    PixelSnake,
    SquareSnake,
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
            match self.style {
                SpinnerStyle::Ticks => paint_tick_spinner(ui, &theme, rect, self.intent),
                SpinnerStyle::Signal => paint_signal_spinner(ui, &theme, rect, self.intent),
                SpinnerStyle::PixelSnake | SpinnerStyle::SquareSnake => {
                    paint_pixel_snake_spinner(ui, &theme, rect, self.intent);
                }
            }
        }

        response
    }
}

fn paint_tick_spinner(ui: &Ui, theme: &CastTheme, rect: egui::Rect, intent: Intent) {
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

fn paint_signal_spinner(ui: &Ui, theme: &CastTheme, rect: egui::Rect, intent: Intent) {
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

fn paint_pixel_snake_spinner(ui: &Ui, theme: &CastTheme, rect: egui::Rect, intent: Intent) {
    let side = rect.width().min(rect.height());
    let accent = progress_accent(theme, intent);
    let time = ui.input(|input| input.time) as f32;
    let cells = pixel_snake_cells(rect, 5);
    let step = (time * 12.0).floor() as usize;
    let tail_len = 6;

    for tail_index in 0..tail_len {
        let index = (step + cells.len() - tail_index) % cells.len();
        let fade = 1.0 - tail_index as f32 / tail_len as f32;
        let cell = cells[index].shrink(side * 0.012);
        ui.painter().rect_filled(
            cell,
            egui::CornerRadius::same((theme.radius.sm * 0.35).round() as u8),
            color_with_scaled_alpha(accent, 0.18 + fade * 0.76),
        );
    }
}

fn pixel_snake_cells(rect: egui::Rect, grid: usize) -> Vec<egui::Rect> {
    let grid = grid.max(3);
    let side = rect.width().min(rect.height());
    let cell = side / grid as f32;
    let origin = egui::pos2(rect.center().x - side / 2.0, rect.center().y - side / 2.0);
    let mut cells = Vec::with_capacity((grid - 1) * 4);

    for column in 0..grid {
        cells.push(pixel_cell(origin, cell, column, 0));
    }
    for row in 1..grid {
        cells.push(pixel_cell(origin, cell, grid - 1, row));
    }
    for column in (0..grid - 1).rev() {
        cells.push(pixel_cell(origin, cell, column, grid - 1));
    }
    for row in (1..grid - 1).rev() {
        cells.push(pixel_cell(origin, cell, 0, row));
    }

    cells
}

fn pixel_cell(origin: egui::Pos2, size: f32, column: usize, row: usize) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(
            origin.x + column as f32 * size,
            origin.y + row as f32 * size,
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

    #[test]
    fn spinner_defaults_to_tick_style() {
        assert_eq!(Spinner::new().style, SpinnerStyle::Ticks);
    }

    #[test]
    fn spinner_style_can_be_changed() {
        assert_eq!(
            Spinner::new().style(SpinnerStyle::PixelSnake).style,
            SpinnerStyle::PixelSnake
        );
    }

    #[test]
    fn pixel_snake_cells_trace_square_perimeter() {
        let rect = egui::Rect::from_min_max(egui::pos2(2.0, 4.0), egui::pos2(12.0, 14.0));
        let cells = pixel_snake_cells(rect, 3);

        assert_eq!(cells.len(), 8);
        assert_eq!(cells[0].min, egui::pos2(2.0, 4.0));
        assert_eq!(cells[2].min.x, cells[3].min.x);
        assert_eq!(cells[4].min.y, cells[5].min.y);
    }
}
