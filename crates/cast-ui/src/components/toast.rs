use egui::{
    Align2, Color32, Context, Id, InnerResponse, Order, Response, RichText, Sense, Stroke,
    StrokeKind, Ui, Vec2, Widget,
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
    dismissible: bool,
}

impl Toast {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: None,
            intent: Intent::Neutral,
            width: None,
            dismissible: true,
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

    #[must_use]
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    pub fn show(self, ui: &mut Ui) -> ToastResponse {
        self.show_inner(ui, None::<fn(&mut Ui)>)
    }

    pub fn show_with(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) -> ToastResponse {
        self.show_inner(ui, Some(add_contents))
    }

    fn show_inner(self, ui: &mut Ui, add_contents: Option<impl FnOnce(&mut Ui)>) -> ToastResponse {
        let theme = theme_for_ui(ui);
        let colors = alert_intent_colors(&theme, self.intent);
        let mut dismissed = false;

        let response = toast_frame(&theme, colors.border)
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

                        if let Some(add_contents) = add_contents {
                            ui.add_space(theme.spacing.xs);
                            add_contents(ui);
                        }
                    });

                    if self.dismissible {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            dismissed = toast_close_button(ui, &theme).clicked();
                        });
                    }
                });
            })
            .response;

        ToastResponse {
            response,
            dismissed,
        }
    }
}

impl Widget for Toast {
    fn ui(self, ui: &mut Ui) -> Response {
        self.dismissible(false).show(ui).response
    }
}

#[derive(Debug)]
pub struct ToastResponse {
    pub response: Response,
    pub dismissed: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToastPlacement {
    TopLeft,
    #[default]
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToastStackMode {
    Expanded,
    #[default]
    Compact,
}

#[derive(Clone, Debug)]
pub struct ToastStack<'a> {
    id: Id,
    toasts: &'a [Toast],
    placement: ToastPlacement,
    mode: ToastStackMode,
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
            mode: ToastStackMode::Compact,
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
    pub fn mode(mut self, mode: ToastStackMode) -> Self {
        self.mode = mode;
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

    pub fn show(self, ctx: &Context) -> Option<InnerResponse<ToastStackResponse>> {
        if self.toasts.is_empty() {
            return None;
        }

        let hover_id = self.id.with("hovered");
        let was_hovered = ctx.data(|data| data.get_temp::<bool>(hover_id).unwrap_or(false));
        let collapsed =
            self.mode == ToastStackMode::Compact && !was_hovered && self.toasts.len() > 1;

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
                    let mut stack_response = ToastStackResponse {
                        dismissed_indices: Vec::new(),
                        expanded: !collapsed,
                    };

                    if collapsed {
                        paint_collapsed_stack_backing(ui, &theme, width, self.toasts.len());
                        let response = self.toasts[0].clone().width(width).show(ui);
                        if response.dismissed {
                            stack_response.dismissed_indices.push(0);
                        }
                    } else {
                        for (index, toast) in self.toasts.iter().cloned().enumerate() {
                            let response = toast.width(width).show(ui);
                            if response.dismissed {
                                stack_response.dismissed_indices.push(index);
                            }
                        }
                    }

                    stack_response
                }),
        )
        .inspect(|inner| {
            ctx.data_mut(|data| data.insert_temp(hover_id, inner.response.hovered()));
        })
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ToastStackResponse {
    pub dismissed_indices: Vec<usize>,
    pub expanded: bool,
}

fn paint_toast_marker(ui: &mut Ui, theme: &CastTheme, color: Color32) {
    let rect_size = egui::vec2(10.0, theme.typography.body.size);
    let (rect, _) = ui.allocate_exact_size(rect_size, Sense::hover());
    let center = egui::pos2(rect.center().x, rect.center().y);

    ui.painter().circle_filled(center, 4.0, color);
}

fn toast_close_button(ui: &mut Ui, theme: &CastTheme) -> Response {
    let side = 24.0;
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(side), Sense::click());

    if ui.is_rect_visible(rect) {
        let fill = if response.is_pointer_button_down_on() {
            egui::Color32::from_black_alpha(24)
        } else if response.hovered() {
            egui::Color32::from_black_alpha(12)
        } else {
            Color32::TRANSPARENT
        };

        ui.painter().rect(
            rect,
            egui::CornerRadius::same(theme.radius.sm as u8),
            fill,
            Stroke::NONE,
            StrokeKind::Outside,
        );

        let center = rect.center();
        let offset = 4.0;
        let stroke = Stroke::new(theme.stroke.md, theme.colors.text_muted);
        ui.painter().line_segment(
            [
                center + egui::vec2(-offset, -offset),
                center + egui::vec2(offset, offset),
            ],
            stroke,
        );
        ui.painter().line_segment(
            [
                center + egui::vec2(-offset, offset),
                center + egui::vec2(offset, -offset),
            ],
            stroke,
        );
    }

    response
}

fn paint_collapsed_stack_backing(ui: &mut Ui, theme: &CastTheme, width: f32, count: usize) {
    let visible_backing = count.saturating_sub(1).min(2);
    if visible_backing == 0 {
        return;
    }

    let top_left = ui.cursor().min;
    for depth in (1..=visible_backing).rev() {
        let inset = depth as f32 * 8.0;
        let y_offset = depth as f32 * 7.0;
        let rect = egui::Rect::from_min_size(
            egui::pos2(top_left.x + inset, top_left.y + y_offset),
            egui::vec2((width - inset * 2.0).max(180.0), 52.0),
        );
        ui.painter().rect(
            rect,
            egui::CornerRadius::same(theme.radius.lg as u8),
            theme.colors.surface_overlay,
            Stroke::new(theme.stroke.sm, theme.colors.border),
            StrokeKind::Outside,
        );
    }
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
        assert!(toast.dismissible);
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
        assert_eq!(stack.mode, ToastStackMode::Compact);
        assert_eq!(toast_anchor(stack.placement), Align2::RIGHT_TOP);
    }

    #[test]
    fn toast_stack_mode_can_be_expanded() {
        let toasts = [Toast::new("Saved")];
        let stack = ToastStack::new("toasts", &toasts).mode(ToastStackMode::Expanded);

        assert_eq!(stack.mode, ToastStackMode::Expanded);
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
