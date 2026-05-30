use egui::{InnerResponse, Response, RichText, Ui};

use crate::{
    foundation::Placement,
    style::popover_frame,
    theme::{CastTheme, theme_for_ui},
};

#[derive(Clone, Debug)]
pub struct Popover {
    title: Option<String>,
    body: Option<String>,
    placement: Placement,
    width: Option<f32>,
}

impl Popover {
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: None,
            body: None,
            placement: Placement::Bottom,
            width: None,
        }
    }

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    #[must_use]
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(140.0));
        self
    }

    pub fn show<T, C>(
        self,
        ui: &mut Ui,
        add_trigger: impl FnOnce(&mut Ui) -> T,
        add_contents: impl FnOnce(&mut Ui) -> C,
    ) -> InnerResponse<T> {
        let trigger = ui.scope(add_trigger);
        let response = self.show_on(ui, &trigger.response, add_contents);

        InnerResponse {
            inner: trigger.inner,
            response,
        }
    }

    pub fn show_on<C>(
        self,
        ui: &Ui,
        response: &Response,
        add_contents: impl FnOnce(&mut Ui) -> C,
    ) -> Response {
        let theme = theme_for_ui(ui);
        let width = self.width.unwrap_or(280.0);

        egui::Popup::from_toggle_button_response(response)
            .frame(popover_frame(&theme))
            .width(width)
            .gap(theme.spacing.xs)
            .align(popover_align(self.placement))
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                ui.set_min_width(width);
                ui.set_max_width(width);
                paint_popover_header(ui, &theme, self.title.as_deref(), self.body.as_deref());
                add_contents(ui);
            });

        response.clone()
    }
}

impl Default for Popover {
    fn default() -> Self {
        Self::new()
    }
}

fn paint_popover_header(ui: &mut Ui, theme: &CastTheme, title: Option<&str>, body: Option<&str>) {
    if let Some(title) = title {
        ui.label(
            RichText::new(title)
                .family(theme.typography.strong.family.clone())
                .size(theme.typography.body.size)
                .color(theme.colors.text)
                .extra_letter_spacing(theme.typography.letter_spacing),
        );
    }

    if let Some(body) = body {
        if title.is_some() {
            ui.add_space(theme.spacing.xs);
        }
        ui.label(
            RichText::new(body)
                .family(theme.typography.small.family.clone())
                .size(theme.typography.small.size)
                .color(theme.colors.text_muted)
                .extra_letter_spacing(theme.typography.letter_spacing),
        );
    }

    if title.is_some() || body.is_some() {
        ui.add_space(theme.spacing.md);
    }
}

fn popover_align(placement: Placement) -> egui::RectAlign {
    match placement {
        Placement::Top => egui::RectAlign::TOP,
        Placement::Right => egui::RectAlign::RIGHT,
        Placement::Bottom => egui::RectAlign::BOTTOM,
        Placement::Left => egui::RectAlign::LEFT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popover_defaults_to_bottom_placement() {
        let popover = Popover::new();

        assert_eq!(popover.placement, Placement::Bottom);
        assert!(popover.title.is_none());
        assert!(popover.body.is_none());
    }

    #[test]
    fn popover_width_has_floor() {
        assert_eq!(Popover::new().width(80.0).width, Some(140.0));
    }

    #[test]
    fn popover_alignment_follows_requested_placement() {
        assert_eq!(popover_align(Placement::Top), egui::RectAlign::TOP);
        assert_eq!(popover_align(Placement::Right), egui::RectAlign::RIGHT);
        assert_eq!(popover_align(Placement::Bottom), egui::RectAlign::BOTTOM);
        assert_eq!(popover_align(Placement::Left), egui::RectAlign::LEFT);
    }
}
