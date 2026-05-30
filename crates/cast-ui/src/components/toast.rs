use egui::{
    Align2, Color32, Context, Id, InnerResponse, Order, Response, RichText, Ui, Vec2, Widget,
};

use crate::{
    foundation::Intent,
    style::{alert_intent_colors, toast_frame},
    theme::{CastTheme, theme_for_ui},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Toast {
    title: String,
    body: Option<String>,
    intent: Intent,
    width: Option<f32>,
}

impl Toast {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: None,
            intent: Intent::Neutral,
            width: None,
        }
    }

    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(180.0));
        self
    }
}

impl Widget for Toast {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let colors = alert_intent_colors(&theme, self.intent);

        toast_frame(&theme, colors.border)
            .show(ui, |ui| {
                if let Some(width) = self.width {
                    ui.set_min_width(width);
                    ui.set_max_width(width);
                }

                ui.horizontal_top(|ui| {
                    paint_toast_marker(ui, &theme, colors.fg);
                    ui.add_space(theme.spacing.xs);
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(self.title)
                                .color(theme.colors.text)
                                .family(theme.typography.strong.family.clone())
                                .size(theme.typography.body.size)
                                .extra_letter_spacing(theme.typography.letter_spacing),
                        );
                        if let Some(body) = self.body {
                            ui.add_space(theme.spacing.xs * 0.5);
                            ui.label(
                                RichText::new(body)
                                    .color(theme.colors.text_muted)
                                    .family(theme.typography.small.family.clone())
                                    .size(theme.typography.small.size)
                                    .extra_letter_spacing(theme.typography.letter_spacing),
                            );
                        }
                    });
                });
            })
            .response
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToastPlacement {
    TopLeft,
    #[default]
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Debug)]
pub struct ToastStack<'a> {
    id: Id,
    toasts: &'a [Toast],
    placement: ToastPlacement,
    width: Option<f32>,
    margin: Vec2,
}

impl<'a> ToastStack<'a> {
    #[must_use]
    pub fn new(id_source: impl std::hash::Hash, toasts: &'a [Toast]) -> Self {
        Self {
            id: Id::new(id_source),
            toasts,
            placement: ToastPlacement::TopRight,
            width: None,
            margin: Vec2::splat(16.0),
        }
    }

    #[must_use]
    pub fn placement(mut self, placement: ToastPlacement) -> Self {
        self.placement = placement;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(240.0));
        self
    }

    #[must_use]
    pub fn margin(mut self, margin: Vec2) -> Self {
        self.margin = margin.max(Vec2::ZERO);
        self
    }

    pub fn show(self, ctx: &Context) -> Option<InnerResponse<()>> {
        if self.toasts.is_empty() {
            return None;
        }

        Some(
            egui::Area::new(self.id)
                .order(Order::Foreground)
                .anchor(
                    toast_anchor(self.placement),
                    toast_anchor_offset(self.placement, self.margin),
                )
                .show(ctx, |ui| {
                    let theme = theme_for_ui(ui);
                    let width = self.width.unwrap_or(320.0);
                    ui.set_width(width);
                    ui.spacing_mut().item_spacing.y = theme.spacing.sm;

                    for toast in self.toasts.iter().cloned() {
                        ui.add(toast.width(width));
                    }
                }),
        )
    }
}

fn paint_toast_marker(ui: &mut Ui, theme: &CastTheme, color: Color32) {
    let side = 10.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(side), egui::Sense::hover());
    let center = egui::pos2(
        rect.center().x,
        rect.min.y + theme.typography.body.size * 0.5,
    );

    ui.painter().circle_filled(center, 4.0, color);
}

fn toast_anchor(placement: ToastPlacement) -> Align2 {
    match placement {
        ToastPlacement::TopLeft => Align2::LEFT_TOP,
        ToastPlacement::TopRight => Align2::RIGHT_TOP,
        ToastPlacement::BottomLeft => Align2::LEFT_BOTTOM,
        ToastPlacement::BottomRight => Align2::RIGHT_BOTTOM,
    }
}

fn toast_anchor_offset(placement: ToastPlacement, margin: Vec2) -> Vec2 {
    match placement {
        ToastPlacement::TopLeft => egui::vec2(margin.x, margin.y),
        ToastPlacement::TopRight => egui::vec2(-margin.x, margin.y),
        ToastPlacement::BottomLeft => egui::vec2(margin.x, -margin.y),
        ToastPlacement::BottomRight => egui::vec2(-margin.x, -margin.y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toast_defaults_to_neutral_without_body() {
        let toast = Toast::new("Saved");

        assert_eq!(toast.intent, Intent::Neutral);
        assert!(toast.body.is_none());
        assert!(toast.width.is_none());
    }

    #[test]
    fn toast_width_has_floor() {
        assert_eq!(Toast::new("Saved").width(80.0).width, Some(180.0));
    }

    #[test]
    fn toast_stack_width_has_floor() {
        let toasts = [Toast::new("Saved")];
        let stack = ToastStack::new("toasts", &toasts).width(80.0);

        assert_eq!(stack.width, Some(240.0));
    }

    #[test]
    fn toast_stack_defaults_to_top_right() {
        let toasts = [Toast::new("Saved")];
        let stack = ToastStack::new("toasts", &toasts);

        assert_eq!(stack.placement, ToastPlacement::TopRight);
        assert_eq!(toast_anchor(stack.placement), Align2::RIGHT_TOP);
    }

    #[test]
    fn toast_anchor_offsets_follow_corner_direction() {
        let margin = egui::vec2(12.0, 8.0);

        assert_eq!(
            toast_anchor_offset(ToastPlacement::TopLeft, margin),
            egui::vec2(12.0, 8.0)
        );
        assert_eq!(
            toast_anchor_offset(ToastPlacement::BottomRight, margin),
            egui::vec2(-12.0, -8.0)
        );
    }
}
