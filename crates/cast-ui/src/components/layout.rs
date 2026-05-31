use std::hash::Hash;

use egui::{InnerResponse, Sense, StrokeKind, Ui, UiBuilder};

use crate::{foundation::Orientation, theme::theme_for_ui};

#[derive(Clone, Copy, Debug)]
pub struct ControlGroup {
    gap: Option<f32>,
    padding: Option<f32>,
    width: Option<f32>,
    wrap: bool,
}

impl ControlGroup {
    #[must_use]
    pub fn new() -> Self {
        Self {
            gap: None,
            padding: None,
            width: None,
            wrap: true,
        }
    }

    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = Some(gap.max(0.0));
        self
    }

    #[must_use]
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding.max(0.0));
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(120.0));
        self
    }

    #[must_use]
    pub fn nowrap(mut self) -> Self {
        self.wrap = false;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);
        let gap = self.gap.unwrap_or(theme.spacing.xs);
        let padding = self.padding.unwrap_or(theme.spacing.xs);

        egui::Frame::new()
            .fill(theme.colors.surface)
            .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
            .corner_radius(egui::CornerRadius::same(theme.radius.md.round() as u8))
            .inner_margin(egui::Margin::same(padding as i8))
            .show(ui, |ui| {
                if let Some(width) = self.width {
                    ui.set_width(width);
                    ui.set_max_width(width);
                }
                let previous_spacing = ui.spacing().item_spacing;
                ui.spacing_mut().item_spacing = egui::vec2(gap, gap);
                let inner = if self.wrap {
                    ui.horizontal_wrapped(add_contents)
                } else {
                    ui.horizontal(add_contents)
                };
                ui.spacing_mut().item_spacing = previous_spacing;
                inner.inner
            })
    }
}

impl Default for ControlGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct ResizablePanels<'a> {
    ratio: &'a mut f32,
    orientation: Orientation,
    min_first: f32,
    min_second: f32,
    width: Option<f32>,
    height: Option<f32>,
    id_salt: egui::Id,
}

impl<'a> ResizablePanels<'a> {
    #[must_use]
    pub fn new(ratio: &'a mut f32) -> Self {
        Self {
            ratio,
            orientation: Orientation::Horizontal,
            min_first: 120.0,
            min_second: 120.0,
            width: None,
            height: None,
            id_salt: egui::Id::new("cast_resizable_panels"),
        }
    }

    #[must_use]
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    #[must_use]
    pub fn min_sizes(mut self, first: f32, second: f32) -> Self {
        self.min_first = first.max(48.0);
        self.min_second = second.max(48.0);
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(self.min_first + self.min_second + 12.0));
        self
    }

    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        let floor = match self.orientation {
            Orientation::Horizontal => 96.0,
            Orientation::Vertical => self.min_first + self.min_second + 12.0,
        };
        self.height = Some(height.max(floor));
        self
    }

    #[must_use]
    pub fn id_salt(mut self, id_salt: impl Hash) -> Self {
        self.id_salt = egui::Id::new(id_salt);
        self
    }

    pub fn show(
        self,
        ui: &mut Ui,
        add_first: impl FnOnce(&mut Ui),
        add_second: impl FnOnce(&mut Ui),
    ) -> InnerResponse<()> {
        let theme = theme_for_ui(ui);
        let available = ui.available_size_before_wrap();
        let width = self.width.unwrap_or(available.x.max(320.0));
        let height = self.height.unwrap_or(match self.orientation {
            Orientation::Horizontal => 220.0,
            Orientation::Vertical => available.y.clamp(260.0, 420.0),
        });
        let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), Sense::hover());
        let handle_size = 6.0;
        let split = resizable_split_lengths(
            primary_len(rect, self.orientation),
            handle_size,
            *self.ratio,
            self.min_first,
            self.min_second,
        );
        debug_assert!(split.second >= self.min_second);
        *self.ratio = split.ratio;

        if ui.is_rect_visible(rect) {
            ui.painter().rect(
                rect,
                egui::CornerRadius::same(theme.radius.lg.round() as u8),
                theme.colors.surface,
                egui::Stroke::new(theme.stroke.sm, theme.colors.border),
                StrokeKind::Outside,
            );
        }

        let (first_rect, handle_rect, second_rect) =
            resizable_rects(rect, self.orientation, split.first, handle_size);
        let handle_response = ui.interact(handle_rect, self.id_salt.with("handle"), Sense::drag());
        if handle_response.dragged() {
            let delta = match self.orientation {
                Orientation::Horizontal => handle_response.drag_delta().x,
                Orientation::Vertical => handle_response.drag_delta().y,
            };
            let updated = resizable_split_lengths(
                primary_len(rect, self.orientation),
                handle_size,
                (split.first + delta) / (primary_len(rect, self.orientation) - handle_size),
                self.min_first,
                self.min_second,
            );
            *self.ratio = updated.ratio;
            ui.ctx().request_repaint();
        }

        paint_resizable_handle(ui, handle_rect, self.orientation);
        show_resizable_child(ui, first_rect, self.id_salt.with("first"), add_first);
        show_resizable_child(ui, second_rect, self.id_salt.with("second"), add_second);

        InnerResponse {
            inner: (),
            response: response.union(handle_response),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ResizableSplit {
    first: f32,
    second: f32,
    ratio: f32,
}

fn resizable_split_lengths(
    total: f32,
    handle: f32,
    ratio: f32,
    min_first: f32,
    min_second: f32,
) -> ResizableSplit {
    let content = (total - handle).max(min_first + min_second);
    let first = (content * ratio.clamp(0.0, 1.0)).clamp(min_first, content - min_second);
    let second = content - first;

    ResizableSplit {
        first,
        second,
        ratio: if content <= 0.0 { 0.5 } else { first / content },
    }
}

fn primary_len(rect: egui::Rect, orientation: Orientation) -> f32 {
    match orientation {
        Orientation::Horizontal => rect.width(),
        Orientation::Vertical => rect.height(),
    }
}

fn resizable_rects(
    rect: egui::Rect,
    orientation: Orientation,
    first: f32,
    handle: f32,
) -> (egui::Rect, egui::Rect, egui::Rect) {
    match orientation {
        Orientation::Horizontal => {
            let first_rect =
                egui::Rect::from_min_max(rect.min, egui::pos2(rect.min.x + first, rect.max.y));
            let handle_rect = egui::Rect::from_min_max(
                egui::pos2(first_rect.max.x, rect.min.y),
                egui::pos2(first_rect.max.x + handle, rect.max.y),
            );
            let second_rect =
                egui::Rect::from_min_max(egui::pos2(handle_rect.max.x, rect.min.y), rect.max);
            (first_rect, handle_rect, second_rect)
        }
        Orientation::Vertical => {
            let first_rect =
                egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.min.y + first));
            let handle_rect = egui::Rect::from_min_max(
                egui::pos2(rect.min.x, first_rect.max.y),
                egui::pos2(rect.max.x, first_rect.max.y + handle),
            );
            let second_rect =
                egui::Rect::from_min_max(egui::pos2(rect.min.x, handle_rect.max.y), rect.max);
            (first_rect, handle_rect, second_rect)
        }
    }
}

fn show_resizable_child(
    ui: &mut Ui,
    rect: egui::Rect,
    id: egui::Id,
    add_contents: impl FnOnce(&mut Ui),
) {
    let theme = theme_for_ui(ui);
    let content_rect = rect.shrink(theme.spacing.md);
    let mut child = ui.new_child(
        UiBuilder::new()
            .max_rect(content_rect)
            .layout(egui::Layout::top_down(egui::Align::Min))
            .id_salt(id),
    );
    child.set_clip_rect(rect.intersect(ui.clip_rect()));
    add_contents(&mut child);
}

fn paint_resizable_handle(ui: &Ui, rect: egui::Rect, orientation: Orientation) {
    let theme = theme_for_ui(ui);
    ui.painter()
        .rect_filled(rect, 0.0, theme.colors.surface_muted);
    let center = rect.center();
    let stroke = egui::Stroke::new(theme.stroke.md.max(1.0), theme.colors.border_strong);
    match orientation {
        Orientation::Horizontal => ui.painter().line_segment(
            [
                egui::pos2(center.x, rect.min.y + theme.spacing.lg),
                egui::pos2(center.x, rect.max.y - theme.spacing.lg),
            ],
            stroke,
        ),
        Orientation::Vertical => ui.painter().line_segment(
            [
                egui::pos2(rect.min.x + theme.spacing.lg, center.y),
                egui::pos2(rect.max.x - theme.spacing.lg, center.y),
            ],
            stroke,
        ),
    };
}

#[derive(Clone, Copy, Debug)]
pub struct ResponsiveColumns {
    breakpoint: f32,
    min_column_width: f32,
    gap: Option<f32>,
}

impl ResponsiveColumns {
    #[must_use]
    pub fn new() -> Self {
        Self {
            breakpoint: 720.0,
            min_column_width: 260.0,
            gap: None,
        }
    }

    #[must_use]
    pub fn breakpoint(mut self, breakpoint: f32) -> Self {
        self.breakpoint = breakpoint.max(240.0);
        self
    }

    #[must_use]
    pub fn min_column_width(mut self, width: f32) -> Self {
        self.min_column_width = width.max(120.0);
        self
    }

    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = Some(gap.max(0.0));
        self
    }

    pub fn show<L, R>(
        self,
        ui: &mut Ui,
        left: L,
        right: R,
    ) -> InnerResponse<(InnerResponse<()>, InnerResponse<()>)>
    where
        L: FnOnce(&mut Ui),
        R: FnOnce(&mut Ui),
    {
        let theme = theme_for_ui(ui);
        let gap = self.gap.unwrap_or(theme.spacing.md);
        let available = ui.available_width();

        ui.vertical(|ui| {
            if available < self.breakpoint {
                let left_response = ui.vertical(left);
                ui.add_space(gap);
                let right_response = ui.vertical(right);
                (left_response, right_response)
            } else {
                let column_width = ((available - gap) / 2.0).max(self.min_column_width);
                let mut left_response = None;
                let mut right_response = None;

                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    left_response = Some(ui.vertical(|ui| {
                        ui.set_width(column_width);
                        ui.set_max_width(column_width);
                        left(ui);
                    }));
                    ui.add_space(gap);
                    right_response = Some(ui.vertical(|ui| {
                        ui.set_width(column_width);
                        ui.set_max_width(column_width);
                        right(ui);
                    }));
                });

                (
                    left_response.expect("left column is always rendered"),
                    right_response.expect("right column is always rendered"),
                )
            }
        })
    }
}

impl Default for ResponsiveColumns {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn responsive_columns_have_reasonable_defaults() {
        let columns = ResponsiveColumns::new()
            .breakpoint(120.0)
            .min_column_width(10.0);

        assert_eq!(columns.breakpoint, 240.0);
        assert_eq!(columns.min_column_width, 120.0);
    }

    #[test]
    fn control_group_defaults_to_wrapped_contents() {
        let group = ControlGroup::new().width(80.0).gap(2.0).padding(3.0);

        assert_eq!(group.width, Some(120.0));
        assert_eq!(group.gap, Some(2.0));
        assert_eq!(group.padding, Some(3.0));
        assert!(group.wrap);
    }

    #[test]
    fn resizable_split_respects_minimum_sizes() {
        let split = resizable_split_lengths(300.0, 6.0, 0.95, 120.0, 90.0);

        assert_eq!(split.first, 204.0);
        assert_eq!(split.second, 90.0);
        assert!((split.ratio - (204.0 / 294.0)).abs() < f32::EPSILON);
    }
}
