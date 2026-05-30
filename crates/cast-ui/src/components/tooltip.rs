use egui::{InnerResponse, Response, RichText, Ui};

use crate::{
    foundation::Placement,
    style::tooltip_frame,
    theme::{CastTheme, theme_for_ui},
};

#[derive(Clone, Debug)]
pub struct Tooltip {
    body: String,
    title: Option<String>,
    placement: Placement,
    width: Option<f32>,
    at_pointer: bool,
}

impl Tooltip {
    #[must_use]
    pub fn new(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            title: None,
            placement: Placement::Top,
            width: None,
            at_pointer: false,
        }
    }

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(80.0));
        self
    }

    #[must_use]
    pub fn at_pointer(mut self) -> Self {
        self.at_pointer = true;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let inner = ui.scope(add_contents);
        let response = self.show_on(ui, inner.response);

        InnerResponse {
            inner: inner.inner,
            response,
        }
    }

    pub fn show_on(self, ui: &Ui, response: Response) -> Response {
        let theme = theme_for_ui(ui);
        let width = self.width.unwrap_or_else(|| tooltip_width(&theme));
        let mut tooltip = egui::Tooltip::for_enabled(&response)
            .gap(theme.spacing.xs)
            .width(width);
        tooltip.popup = tooltip
            .popup
            .frame(tooltip_frame(&theme))
            .align(tooltip_align(self.placement));
        if self.at_pointer {
            tooltip = tooltip.at_pointer();
        }

        tooltip.show(|ui| {
            ui.set_max_width(width);
            paint_tooltip_content(ui, &theme, self.title.as_deref(), &self.body);
        });

        response
    }
}

fn paint_tooltip_content(ui: &mut Ui, theme: &CastTheme, title: Option<&str>, body: &str) {
    if let Some(title) = title {
        ui.label(
            RichText::new(title)
                .family(theme.typography.strong.family.clone())
                .size(theme.typography.small.size)
                .color(theme.colors.text)
                .extra_letter_spacing(theme.typography.letter_spacing),
        );
        ui.add_space(theme.spacing.xs * 0.5);
    }

    ui.label(
        RichText::new(body)
            .family(theme.typography.caption.family.clone())
            .size(theme.typography.caption.size)
            .color(theme.colors.text_muted)
            .extra_letter_spacing(theme.typography.letter_spacing),
    );
}

fn tooltip_width(_theme: &CastTheme) -> f32 {
    240.0
}

fn tooltip_align(placement: Placement) -> egui::RectAlign {
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
    fn tooltip_defaults_to_top_placement() {
        let tooltip = Tooltip::new("Explain this");

        assert_eq!(tooltip.placement, Placement::Top);
        assert_eq!(tooltip.body, "Explain this");
        assert!(tooltip.title.is_none());
    }

    #[test]
    fn tooltip_width_has_reasonable_floor() {
        let theme = CastTheme::light();

        assert!(tooltip_width(&theme) >= 240.0);
    }

    #[test]
    fn tooltip_alignment_follows_requested_placement() {
        assert_eq!(tooltip_align(Placement::Top), egui::RectAlign::TOP);
        assert_eq!(tooltip_align(Placement::Right), egui::RectAlign::RIGHT);
        assert_eq!(tooltip_align(Placement::Bottom), egui::RectAlign::BOTTOM);
        assert_eq!(tooltip_align(Placement::Left), egui::RectAlign::LEFT);
    }
}
