use egui::{
    Color32, Direction, InnerResponse, Response, RichText, Sense, Shape, Stroke, Ui, Widget,
};

use crate::{
    color::{mix_oklch, mix_with_transparent, with_alpha},
    components::{Badge, ProgressBar},
    foundation::{Intent, Size},
    theme::{CastTheme, theme_for_ui},
};

#[derive(Clone, Debug)]
pub struct MetricCard {
    label: String,
    value: String,
    delta: Option<String>,
    detail: Option<String>,
    intent: Intent,
    width: Option<f32>,
}

impl MetricCard {
    #[must_use]
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            delta: None,
            detail: None,
            intent: Intent::Neutral,
            width: None,
        }
    }

    #[must_use]
    pub fn delta(mut self, delta: impl Into<String>, intent: Intent) -> Self {
        self.delta = Some(delta.into());
        self.intent = intent;
        self
    }

    #[must_use]
    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
}

impl Widget for MetricCard {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(180.0));

        egui::Frame::new()
            .fill(theme.colors.surface_raised)
            .stroke(Stroke::new(theme.stroke.sm, theme.colors.border))
            .corner_radius(egui::CornerRadius::same(theme.radius.lg as u8))
            .inner_margin(egui::Margin::same(theme.spacing.md as i8))
            .show(ui, |ui| {
                ui.set_width(frame_inner_width(width, theme.spacing.md));
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(self.label)
                            .font(theme.typography.small.clone())
                            .color(theme.colors.text_muted),
                    );
                    if let Some(delta) = self.delta {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add(Badge::new(delta).intent(self.intent).status_dot());
                        });
                    }
                });
                ui.add_space(theme.spacing.xs);
                ui.label(
                    RichText::new(self.value)
                        .font(theme.typography.heading.clone())
                        .color(theme.colors.text),
                );
                if let Some(detail) = self.detail {
                    ui.label(
                        RichText::new(detail)
                            .font(theme.typography.caption.clone())
                            .color(theme.colors.text_subtle),
                    );
                }
            })
            .response
    }
}

#[derive(Clone, Debug)]
pub struct Sparkline {
    values: Vec<f32>,
    intent: Intent,
    width: Option<f32>,
    height: f32,
}

impl Sparkline {
    #[must_use]
    pub fn new(values: impl IntoIterator<Item = f32>) -> Self {
        Self {
            values: values.into_iter().collect(),
            intent: Intent::Primary,
            width: None,
            height: 48.0,
        }
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(96.0));
        self
    }

    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(24.0);
        self
    }
}

impl Widget for Sparkline {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let width = self.width.unwrap_or_else(|| ui.available_width().max(96.0));
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(width, self.height), Sense::hover());

        if ui.is_rect_visible(rect) {
            paint_sparkline(ui, &theme, rect, &self.values, self.intent);
        }

        response
    }
}

#[derive(Clone, Debug)]
pub struct BarDatum {
    label: String,
    value: f32,
    intent: Intent,
}

impl BarDatum {
    #[must_use]
    pub fn new(label: impl Into<String>, value: f32) -> Self {
        Self {
            label: label.into(),
            value,
            intent: Intent::Primary,
        }
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }
}

#[derive(Clone, Debug)]
pub struct BarChart {
    data: Vec<BarDatum>,
    width: Option<f32>,
    height: f32,
}

impl BarChart {
    #[must_use]
    pub fn new(data: impl IntoIterator<Item = BarDatum>) -> Self {
        Self {
            data: data.into_iter().collect(),
            width: None,
            height: 160.0,
        }
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(160.0));
        self
    }

    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(96.0);
        self
    }
}

impl Widget for BarChart {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(160.0));
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(width, self.height), Sense::hover());

        if ui.is_rect_visible(rect) {
            paint_bar_chart(ui, &theme, rect, &self.data);
        }

        response
    }
}

#[derive(Clone, Debug)]
pub struct ProgressMetric {
    label: String,
    value: f32,
    detail: Option<String>,
    intent: Intent,
    width: Option<f32>,
}

impl ProgressMetric {
    #[must_use]
    pub fn new(label: impl Into<String>, value: f32) -> Self {
        Self {
            label: label.into(),
            value,
            detail: None,
            intent: Intent::Primary,
            width: None,
        }
    }

    #[must_use]
    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(120.0));
        self
    }
}

impl Widget for ProgressMetric {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(120.0));

        ui.vertical(|ui| {
            ui.set_width(width);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(self.label)
                        .font(theme.typography.small.clone())
                        .color(theme.colors.text),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("{}%", (self.value * 100.0).round()))
                            .font(theme.typography.caption.clone())
                            .color(theme.colors.text_muted),
                    );
                });
            });
            ui.add_space(theme.spacing.xs);
            ui.add(
                ProgressBar::new(self.value)
                    .intent(self.intent)
                    .size(Size::Small)
                    .width(width),
            );
            if let Some(detail) = self.detail {
                ui.add_space(theme.spacing.xs);
                ui.label(
                    RichText::new(detail)
                        .font(theme.typography.caption.clone())
                        .color(theme.colors.text_subtle),
                );
            }
        })
        .response
    }
}

#[derive(Clone, Debug)]
pub struct ReportSection {
    title: String,
    description: Option<String>,
    width: Option<f32>,
}

impl ReportSection {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            width: None,
        }
    }

    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(260.0));

        egui::Frame::new()
            .fill(theme.colors.surface_raised)
            .stroke(Stroke::new(theme.stroke.sm, theme.colors.border))
            .corner_radius(egui::CornerRadius::same(theme.radius.lg as u8))
            .inner_margin(egui::Margin::same(theme.spacing.md as i8))
            .show(ui, |ui| {
                ui.set_width(frame_inner_width(width, theme.spacing.md));
                ui.label(
                    RichText::new(self.title)
                        .font(theme.typography.body_strong.clone())
                        .color(theme.colors.text),
                );
                if let Some(description) = self.description {
                    ui.label(
                        RichText::new(description)
                            .font(theme.typography.small.clone())
                            .color(theme.colors.text_muted),
                    );
                }
                ui.add_space(theme.spacing.md);
                add_contents(ui)
            })
    }
}

fn paint_sparkline(ui: &Ui, theme: &CastTheme, rect: egui::Rect, values: &[f32], intent: Intent) {
    paint_plot_shell(ui, theme, rect, 3);

    if values.len() < 2 {
        return;
    }

    let (min, max) = value_range(values);
    let accent = intent_color(theme, intent);
    let inset = egui::vec2(theme.spacing.sm, theme.spacing.sm);
    let plot = rect.shrink2(inset);
    let points = values
        .iter()
        .enumerate()
        .map(|(index, value)| sparkline_point(plot, *value, index, values.len(), min, max))
        .collect::<Vec<_>>();

    ui.painter().add(Shape::gradient_rect(
        plot,
        Direction::TopDown,
        [
            mix_with_transparent(accent, 0.12),
            mix_with_transparent(accent, 0.02),
        ],
    ));
    ui.painter().add(Shape::line(
        points.clone(),
        Stroke::new(theme.stroke.lg, with_alpha(accent, 235)),
    ));

    if let Some(last) = points.last().copied() {
        ui.painter().circle_filled(last, 3.5, theme.colors.surface);
        ui.painter()
            .circle_stroke(last, 3.5, Stroke::new(theme.stroke.md, accent));
    }
}

fn paint_bar_chart(ui: &Ui, theme: &CastTheme, rect: egui::Rect, data: &[BarDatum]) {
    paint_plot_shell(ui, theme, rect, 4);

    if data.is_empty() {
        return;
    }

    let max = data
        .iter()
        .map(|datum| datum.value.max(0.0))
        .fold(0.0, f32::max)
        .max(1.0);
    let label_height = 24.0;
    let value_height = 18.0;
    let gap = theme.spacing.sm;
    let plot = egui::Rect::from_min_max(
        rect.min + egui::vec2(theme.spacing.sm, value_height),
        egui::pos2(rect.max.x - theme.spacing.sm, rect.max.y - label_height),
    );
    let bar_width =
        ((plot.width() - gap * (data.len().saturating_sub(1) as f32)) / data.len() as f32).max(4.0);

    for (index, datum) in data.iter().enumerate() {
        let x = plot.min.x + index as f32 * (bar_width + gap);
        let normalized = (datum.value.max(0.0) / max).clamp(0.0, 1.0);
        let height = plot.height() * normalized;
        let track_rect = egui::Rect::from_min_max(
            egui::pos2(x, plot.min.y),
            egui::pos2((x + bar_width).min(plot.max.x), plot.max.y),
        );
        let bar_rect = egui::Rect::from_min_max(
            egui::pos2(x, plot.max.y - height),
            egui::pos2((x + bar_width).min(plot.max.x), plot.max.y),
        );
        let accent = intent_color(theme, datum.intent);
        ui.painter().rect_filled(
            track_rect,
            egui::CornerRadius::same(theme.radius.sm as u8),
            mix_with_transparent(theme.colors.text_muted, 0.06),
        );
        ui.painter().rect_filled(
            bar_rect,
            egui::CornerRadius::same(theme.radius.sm as u8),
            mix_oklch(accent, theme.colors.surface, 0.05),
        );
        ui.painter().text(
            egui::pos2(x + bar_width / 2.0, bar_rect.min.y - 4.0),
            egui::Align2::CENTER_BOTTOM,
            compact_chart_value(datum.value).as_str(),
            theme.typography.caption.clone(),
            theme.colors.text_muted,
        );
        ui.painter().text(
            egui::pos2(x + bar_width / 2.0, rect.max.y - 2.0),
            egui::Align2::CENTER_BOTTOM,
            datum.label.as_str(),
            theme.typography.caption.clone(),
            theme.colors.text_subtle,
        );
    }
}

fn paint_plot_shell(ui: &Ui, theme: &CastTheme, rect: egui::Rect, grid_lines: usize) {
    let radius = egui::CornerRadius::same(theme.radius.md as u8);
    ui.painter()
        .rect_filled(rect, radius, plot_background(theme));
    ui.painter().rect_stroke(
        rect,
        radius,
        Stroke::new(
            theme.stroke.sm,
            mix_with_transparent(theme.colors.text_muted, 0.10),
        ),
        egui::StrokeKind::Inside,
    );

    if grid_lines == 0 {
        return;
    }

    let grid = Stroke::new(
        theme.stroke.sm,
        mix_with_transparent(theme.colors.text_muted, 0.10),
    );
    for index in 1..grid_lines {
        let y = rect.top() + rect.height() * index as f32 / grid_lines as f32;
        ui.painter().hline(rect.x_range(), y, grid);
    }
}

fn plot_background(theme: &CastTheme) -> Color32 {
    mix_oklch(theme.colors.surface_muted, theme.colors.surface, 0.35)
}

fn compact_chart_value(value: f32) -> String {
    if value >= 1000.0 {
        format!("{:.1}k", value / 1000.0)
    } else if value.fract().abs() < f32::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.1}")
    }
}

fn sparkline_point(
    rect: egui::Rect,
    value: f32,
    index: usize,
    len: usize,
    min: f32,
    max: f32,
) -> egui::Pos2 {
    let x = rect.left() + rect.width() * index as f32 / (len.saturating_sub(1).max(1) as f32);
    let normalized = if (max - min).abs() <= f32::EPSILON {
        0.5
    } else {
        (value - min) / (max - min)
    };
    let y = rect.bottom() - rect.height() * normalized.clamp(0.0, 1.0);
    egui::pos2(x, y)
}

fn value_range(values: &[f32]) -> (f32, f32) {
    values
        .iter()
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), value| {
            (min.min(*value), max.max(*value))
        })
}

fn intent_color(theme: &CastTheme, intent: Intent) -> Color32 {
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

fn frame_inner_width(outer_width: f32, horizontal_padding: f32) -> f32 {
    (outer_width - horizontal_padding * 2.0).max(80.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparkline_range_handles_flat_values() {
        assert_eq!(value_range(&[2.0, 2.0, 2.0]), (2.0, 2.0));
    }

    #[test]
    fn chart_components_store_metrics() {
        let metric = MetricCard::new("Revenue", "$42k").delta("+12%", Intent::Success);
        let sparkline = Sparkline::new([1.0, 2.0, 3.0]).height(4.0);
        let chart = BarChart::new([
            BarDatum::new("A", 4.0),
            BarDatum::new("B", 8.0).intent(Intent::Info),
        ]);
        let progress = ProgressMetric::new("Coverage", 0.82).detail("Target 80%");

        assert_eq!(metric.intent, Intent::Success);
        assert_eq!(sparkline.height, 24.0);
        assert_eq!(chart.data.len(), 2);
        assert_eq!(progress.detail.as_deref(), Some("Target 80%"));
    }
}
