use egui::{Response, Sense, StrokeKind, Ui, Widget};

use crate::{foundation::Size, theme::theme_for_ui};

#[derive(Debug)]
pub struct Switch<'a> {
    checked: &'a mut bool,
    size: Size,
}

impl<'a> Switch<'a> {
    #[must_use]
    pub fn new(checked: &'a mut bool) -> Self {
        Self {
            checked,
            size: Size::Medium,
        }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Widget for Switch<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let desired_size = switch_size(self.size);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        if response.clicked() {
            *self.checked = !*self.checked;
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let on = *self.checked;
            let track_fill = if on {
                theme.colors.primary
            } else {
                theme.colors.surface_muted
            };
            let track_stroke = if response.hovered() {
                theme.colors.border_strong
            } else {
                theme.colors.border
            };
            let radius = (rect.height() / 2.0).round() as u8;

            ui.painter().rect(
                rect,
                egui::CornerRadius::same(radius),
                track_fill,
                egui::Stroke::new(theme.stroke.sm, track_stroke),
                StrokeKind::Outside,
            );

            let knob_radius = (rect.height() - 6.0) / 2.0;
            let knob_x = if on {
                rect.right() - 3.0 - knob_radius
            } else {
                rect.left() + 3.0 + knob_radius
            };
            let knob_fill = if on {
                theme.colors.primary_fg
            } else {
                theme.colors.surface
            };

            ui.painter()
                .circle_filled(egui::pos2(knob_x, rect.center().y), knob_radius, knob_fill);
        }

        response
    }
}

fn switch_size(size: Size) -> egui::Vec2 {
    match size {
        Size::Small => egui::vec2(34.0, 20.0),
        Size::Medium => egui::vec2(42.0, 24.0),
        Size::Large => egui::vec2(50.0, 28.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switch_sizes_scale() {
        let small = switch_size(Size::Small);
        let medium = switch_size(Size::Medium);
        let large = switch_size(Size::Large);

        assert!(small.x < medium.x);
        assert!(medium.x < large.x);
        assert!(small.y < medium.y);
        assert!(medium.y < large.y);
    }
}
