use egui::{Color32, Response, Sense, Ui, Vec2, Widget};

use crate::{color::mix_with_transparent, foundation::Size, theme::theme_for_ui};

#[derive(Clone, Debug)]
pub struct Skeleton {
    width: Option<f32>,
    height: Option<f32>,
    size: Size,
    animated: bool,
}

impl Skeleton {
    #[must_use]
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            size: Size::Medium,
            animated: true,
        }
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(8.0));
        self
    }

    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height.max(4.0));
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Skeleton {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let height = self.height.unwrap_or_else(|| skeleton_height(self.size));
        let width = self.width.unwrap_or_else(|| ui.available_width().max(48.0));
        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

        if ui.is_rect_visible(rect) {
            let radius = egui::CornerRadius::same((height / 2.0).min(theme.radius.md) as u8);
            let fill = skeleton_fill(&theme);
            ui.painter().rect_filled(rect, radius, fill);

            if self.animated && !theme.animation.reduced_motion {
                ui.ctx()
                    .request_repaint_after(std::time::Duration::from_millis(24));
                paint_skeleton_shimmer(ui, rect, radius);
            }
        }

        response
    }
}

fn paint_skeleton_shimmer(ui: &Ui, rect: egui::Rect, radius: egui::CornerRadius) {
    let time = ui.input(|input| input.time) as f32;
    let sweep = rect.width() * 0.28;
    let travel = rect.width() + sweep * 2.0;
    let offset = ((time * 0.78).fract() * travel) - sweep;
    let shimmer_rect = egui::Rect::from_min_size(
        egui::pos2(rect.min.x + offset, rect.min.y),
        egui::vec2(sweep, rect.height()),
    )
    .intersect(rect);

    if shimmer_rect.is_positive() {
        ui.painter().rect_filled(
            shimmer_rect,
            radius,
            Color32::from_rgba_unmultiplied(255, 255, 255, 28),
        );
    }
}

fn skeleton_height(size: Size) -> f32 {
    match size {
        Size::Small => 10.0,
        Size::Medium => 14.0,
        Size::Large => 18.0,
    }
}

fn skeleton_fill(theme: &crate::CastTheme) -> Color32 {
    mix_with_transparent(theme.colors.text_muted, 0.14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skeleton_sizes_scale() {
        assert!(skeleton_height(Size::Small) < skeleton_height(Size::Medium));
        assert!(skeleton_height(Size::Medium) < skeleton_height(Size::Large));
    }

    #[test]
    fn skeleton_dimensions_have_floor() {
        let skeleton = Skeleton::new().width(1.0).height(1.0);

        assert_eq!(skeleton.width, Some(8.0));
        assert_eq!(skeleton.height, Some(4.0));
    }
}
