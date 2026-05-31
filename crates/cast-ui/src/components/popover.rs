use egui::{InnerResponse, Response, RichText, Ui};

use crate::{
    components::card::{
        SurfaceSectionStyle, show_surface_sections_inside_with_radius,
        show_surface_sections_optional_with_radius,
    },
    foundation::Placement,
    style::{popover_frame, popover_shell_frame},
    theme::{CastTheme, theme_for_ui},
};

#[derive(Clone, Debug)]
pub struct Popover {
    title: Option<String>,
    body: Option<String>,
    placement: Placement,
    width: Option<f32>,
    sections: SurfaceSectionStyle,
}

impl Popover {
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: None,
            body: None,
            placement: Placement::Bottom,
            width: None,
            sections: SurfaceSectionStyle::flat(),
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

    #[must_use]
    pub fn section_style(mut self, sections: SurfaceSectionStyle) -> Self {
        self.sections = sections;
        self
    }

    #[must_use]
    pub fn muted_sections(mut self) -> Self {
        self.sections = SurfaceSectionStyle::muted();
        self
    }

    pub fn show<C>(
        self,
        ui: &mut Ui,
        add_trigger: impl FnOnce(&mut Ui) -> Response,
        add_contents: impl FnOnce(&mut Ui) -> C,
    ) -> InnerResponse<Response> {
        let trigger = ui.scope(add_trigger);
        let trigger_response = trigger.inner;
        let response = self.show_on(ui, &trigger_response, add_contents);

        InnerResponse {
            inner: trigger_response,
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

    pub fn show_with_footer<C>(
        self,
        ui: &mut Ui,
        add_trigger: impl FnOnce(&mut Ui) -> Response,
        add_contents: impl FnOnce(&mut Ui) -> C,
        add_footer: impl FnOnce(&mut Ui),
    ) -> InnerResponse<Response> {
        let trigger = ui.scope(add_trigger);
        let trigger_response = trigger.inner;
        let response = self.show_on_with_footer(ui, &trigger_response, add_contents, add_footer);

        InnerResponse {
            inner: trigger_response,
            response,
        }
    }

    pub fn show_on_with_footer<C>(
        self,
        ui: &Ui,
        response: &Response,
        add_contents: impl FnOnce(&mut Ui) -> C,
        add_footer: impl FnOnce(&mut Ui),
    ) -> Response {
        let title = self.title.clone();
        let body = self.body.clone();

        self.show_on_sections(
            ui,
            response,
            move |ui| {
                paint_popover_header(ui, &theme_for_ui(ui), title.as_deref(), body.as_deref());
            },
            add_contents,
            add_footer,
        )
    }

    pub fn show_on_sections<C>(
        self,
        ui: &Ui,
        response: &Response,
        add_header: impl FnOnce(&mut Ui),
        add_contents: impl FnOnce(&mut Ui) -> C,
        add_footer: impl FnOnce(&mut Ui),
    ) -> Response {
        let theme = theme_for_ui(ui);
        let width = self.width.unwrap_or(280.0);

        egui::Popup::from_toggle_button_response(response)
            .frame(popover_shell_frame(&theme))
            .width(width)
            .gap(theme.spacing.xs)
            .align(popover_align(self.placement))
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                ui.set_min_width(width);
                ui.set_max_width(width);
                show_surface_sections_inside_with_radius(
                    ui,
                    &theme,
                    self.sections,
                    theme.components.section.compact_padding,
                    theme.radius.lg,
                    add_header,
                    add_contents,
                    add_footer,
                );
            });

        response.clone()
    }
}

impl Default for Popover {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct HoverCard {
    title: Option<String>,
    body: Option<String>,
    placement: Placement,
    width: Option<f32>,
    sections: SurfaceSectionStyle,
}

impl HoverCard {
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: None,
            body: None,
            placement: Placement::Right,
            width: None,
            sections: SurfaceSectionStyle::flat(),
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
        self.width = Some(width.max(160.0));
        self
    }

    #[must_use]
    pub fn section_style(mut self, sections: SurfaceSectionStyle) -> Self {
        self.sections = sections;
        self
    }

    #[must_use]
    pub fn muted_sections(mut self) -> Self {
        self.sections = SurfaceSectionStyle::muted();
        self
    }

    pub fn show<C>(
        self,
        ui: &mut Ui,
        add_trigger: impl FnOnce(&mut Ui) -> Response,
        add_contents: impl FnOnce(&mut Ui) -> C,
    ) -> InnerResponse<Response> {
        let trigger = ui.scope(add_trigger);
        let trigger_response = trigger.inner;
        let response = self.show_on(ui, &trigger_response, add_contents);

        InnerResponse {
            inner: trigger_response,
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
        let width = self.width.unwrap_or(320.0);
        let mut tooltip = egui::Tooltip::for_enabled(response)
            .gap(theme.spacing.xs)
            .width(width);
        tooltip.popup = tooltip
            .popup
            .frame(popover_shell_frame(&theme))
            .align(popover_align(self.placement));

        tooltip.show(|ui| {
            ui.set_min_width(width);
            ui.set_max_width(width);
            show_surface_sections_optional_with_radius(
                ui,
                &theme,
                self.sections,
                theme.components.section.compact_padding,
                theme.radius.lg,
                Some(|ui: &mut Ui| {
                    paint_popover_header(ui, &theme, self.title.as_deref(), self.body.as_deref());
                }),
                add_contents,
                None::<fn(&mut Ui)>,
            );
        });

        response.clone()
    }
}

impl Default for HoverCard {
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
        assert_eq!(popover.sections, SurfaceSectionStyle::flat());
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

    #[test]
    fn popover_can_use_muted_sections() {
        let popover = Popover::new().muted_sections();

        assert_eq!(popover.sections, SurfaceSectionStyle::muted());
    }

    #[test]
    fn hover_card_defaults_to_right_placement() {
        let card = HoverCard::new();

        assert_eq!(card.placement, Placement::Right);
        assert_eq!(card.sections, SurfaceSectionStyle::flat());
        assert!(card.title.is_none());
    }

    #[test]
    fn hover_card_width_has_floor() {
        assert_eq!(HoverCard::new().width(80.0).width, Some(160.0));
    }
}
