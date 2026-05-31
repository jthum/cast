use egui::{
    Color32, Id, InnerResponse, Pos2, Rect, Response, RichText, Sense, StrokeKind, Ui, Vec2,
    epaint::Stroke,
};

use crate::{
    color::mix_with_transparent,
    components::{
        Button,
        card::{SurfaceChrome, SurfaceSectionStyle, paint_section_divider, show_surface_section},
    },
    foundation::{Intent, Placement, Size, Variant},
    style::{dialog_backdrop, dialog_frame, dialog_shell_frame},
    theme::{CastTheme, current_theme},
};

#[derive(Debug)]
pub struct Dialog<'a> {
    open: &'a mut bool,
    id: Id,
    title: Option<String>,
    description: Option<String>,
    width: Option<f32>,
    closable: bool,
    sections: SurfaceSectionStyle,
}

impl<'a> Dialog<'a> {
    #[must_use]
    pub fn new(open: &'a mut bool, id_source: impl std::hash::Hash) -> Self {
        Self {
            open,
            id: Id::new(id_source),
            title: None,
            description: None,
            width: None,
            closable: true,
            sections: SurfaceSectionStyle::flat(),
        }
    }

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(260.0));
        self
    }

    #[must_use]
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
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

    pub fn show<R>(
        self,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut Ui, &mut DialogController) -> R,
    ) -> Option<egui::ModalResponse<R>> {
        if !*self.open {
            return None;
        }

        let theme = current_theme(ctx).unwrap_or_else(CastTheme::light);
        let width = self.width.unwrap_or(420.0);
        let mut controller = DialogController::default();
        let response = egui::Modal::new(self.id)
            .frame(dialog_frame(&theme))
            .backdrop_color(dialog_backdrop(&theme))
            .show(ctx, |ui| {
                ui.set_min_width(width);
                ui.set_max_width(width);

                paint_dialog_header(
                    ui,
                    &theme,
                    self.title.as_deref(),
                    self.description.as_deref(),
                    self.closable,
                    &mut controller,
                );

                add_contents(ui, &mut controller)
            });

        if response.should_close() || controller.close_requested {
            *self.open = false;
        }

        Some(response)
    }

    pub fn show_with_footer<R>(
        self,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut Ui, &mut DialogController) -> R,
        add_footer: impl FnOnce(&mut Ui, &mut DialogController),
    ) -> Option<egui::ModalResponse<R>> {
        let title = self.title.clone();
        let description = self.description.clone();
        let closable = self.closable;

        self.show_sections(
            ctx,
            move |ui, dialog| {
                paint_dialog_header(
                    ui,
                    &current_theme(ui.ctx()).unwrap_or_else(CastTheme::light),
                    title.as_deref(),
                    description.as_deref(),
                    closable,
                    dialog,
                );
            },
            add_contents,
            add_footer,
        )
    }

    pub fn show_sections<R>(
        self,
        ctx: &egui::Context,
        add_header: impl FnOnce(&mut Ui, &mut DialogController),
        add_contents: impl FnOnce(&mut Ui, &mut DialogController) -> R,
        add_footer: impl FnOnce(&mut Ui, &mut DialogController),
    ) -> Option<egui::ModalResponse<R>> {
        if !*self.open {
            return None;
        }

        let theme = current_theme(ctx).unwrap_or_else(CastTheme::light);
        let width = self.width.unwrap_or(420.0);
        let mut controller = DialogController::default();
        let response = egui::Modal::new(self.id)
            .frame(dialog_shell_frame(&theme))
            .backdrop_color(dialog_backdrop(&theme))
            .show(ctx, |ui| {
                ui.set_min_width(width);
                ui.set_max_width(width);
                show_dialog_sections(
                    ui,
                    &theme,
                    self.sections,
                    &mut controller,
                    add_header,
                    add_contents,
                    add_footer,
                )
            });

        if response.should_close() || controller.close_requested {
            *self.open = false;
        }

        Some(response)
    }
}

#[derive(Default, Debug)]
pub struct DialogController {
    close_requested: bool,
}

impl DialogController {
    pub fn close(&mut self) {
        self.close_requested = true;
    }

    #[must_use]
    pub fn close_requested(&self) -> bool {
        self.close_requested
    }
}

#[derive(Debug)]
pub struct ConfirmDialog<'a> {
    open: &'a mut bool,
    id: Id,
    title: String,
    description: String,
    confirm_label: String,
    cancel_label: String,
    intent: Intent,
    width: Option<f32>,
}

impl<'a> ConfirmDialog<'a> {
    #[must_use]
    pub fn new(open: &'a mut bool, id_source: impl std::hash::Hash) -> Self {
        Self {
            open,
            id: Id::new(id_source),
            title: "Confirm action".to_owned(),
            description: "This action needs confirmation before continuing.".to_owned(),
            confirm_label: "Confirm".to_owned(),
            cancel_label: "Cancel".to_owned(),
            intent: Intent::Danger,
            width: None,
        }
    }

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    #[must_use]
    pub fn confirm_label(mut self, confirm_label: impl Into<String>) -> Self {
        self.confirm_label = confirm_label.into();
        self
    }

    #[must_use]
    pub fn cancel_label(mut self, cancel_label: impl Into<String>) -> Self {
        self.cancel_label = cancel_label.into();
        self
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(280.0));
        self
    }

    pub fn show(self, ctx: &egui::Context) -> Option<ConfirmDialogResponse> {
        let mut result = None;
        let open = self.open;
        let width = self.width;
        let title = self.title;
        let description = self.description;
        let confirm_label = self.confirm_label;
        let cancel_label = self.cancel_label;
        let intent = self.intent;

        Dialog {
            open,
            id: self.id,
            title: Some(title),
            description: Some(description),
            width,
            closable: true,
            sections: SurfaceSectionStyle::flat(),
        }
        .show(ctx, |ui, dialog| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add(Button::new(confirm_label).intent(intent).size(Size::Small))
                    .clicked()
                {
                    result = Some(ConfirmDialogResponse::Confirmed);
                    dialog.close();
                }

                if ui
                    .add(
                        Button::new(cancel_label)
                            .intent(Intent::Neutral)
                            .variant(Variant::Outline)
                            .size(Size::Small),
                    )
                    .clicked()
                {
                    result = Some(ConfirmDialogResponse::Cancelled);
                    dialog.close();
                }
            });
        });

        result
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfirmDialogResponse {
    Confirmed,
    Cancelled,
}

#[derive(Debug)]
pub struct Sheet<'a> {
    open: &'a mut bool,
    id: Id,
    title: Option<String>,
    description: Option<String>,
    placement: Placement,
    extent: Option<f32>,
    closable: bool,
    sections: SurfaceSectionStyle,
}

impl<'a> Sheet<'a> {
    #[must_use]
    pub fn new(open: &'a mut bool, id_source: impl std::hash::Hash) -> Self {
        Self {
            open,
            id: Id::new(id_source),
            title: None,
            description: None,
            placement: Placement::Right,
            extent: None,
            closable: true,
            sections: SurfaceSectionStyle::flat(),
        }
    }

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.extent = Some(width.max(sheet_extent_floor()));
        self
    }

    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.extent = Some(height.max(sheet_extent_floor()));
        self
    }

    #[must_use]
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
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

    pub fn show<R>(
        self,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut Ui, &mut SheetController) -> R,
    ) -> Option<InnerResponse<R>> {
        let theme = current_theme(ctx).unwrap_or_else(CastTheme::light);

        if !*self.open {
            ctx.animate_bool_with_time_and_easing(
                self.id.with("slide"),
                false,
                theme.animation.normal_seconds(),
                egui::emath::easing::cubic_out,
            );
            return None;
        }

        let screen_rect = ctx.content_rect();
        let extent = self.extent.unwrap_or(420.0).max(sheet_extent_floor());
        let (pos, size) = sheet_geometry(screen_rect, self.placement, extent);
        let slide_progress = ctx.animate_bool_with_time_and_easing(
            self.id.with("slide"),
            true,
            theme.animation.normal_seconds(),
            egui::emath::easing::cubic_out,
        );
        let pos = sheet_slide_position(pos, size, self.placement, slide_progress);
        let backdrop_id = self.id.with("backdrop");
        let backdrop_response = egui::Area::new(backdrop_id)
            .order(egui::Order::Middle)
            .fixed_pos(screen_rect.min)
            .show(ctx, |ui| {
                ui.painter()
                    .rect_filled(screen_rect, 0.0, dialog_backdrop(&theme));
                let (_, response) = ui.allocate_exact_size(screen_rect.size(), Sense::click());
                response
            });

        let mut controller = SheetController::default();
        let area_response = egui::Area::new(self.id)
            .order(egui::Order::Foreground)
            .fixed_pos(pos)
            .show(ctx, |ui| {
                sheet_frame(&theme, self.placement).show(ui, |ui| {
                    ui.set_min_size(size);
                    ui.set_max_size(size);
                    let mut header_controller = DialogController::default();

                    paint_dialog_header(
                        ui,
                        &theme,
                        self.title.as_deref(),
                        self.description.as_deref(),
                        self.closable,
                        &mut header_controller,
                    );

                    if header_controller.close_requested() {
                        controller.close();
                    }

                    add_contents(ui, &mut controller)
                })
            });

        if (self.closable && backdrop_response.inner.clicked()) || controller.close_requested {
            *self.open = false;
        }

        Some(area_response.inner)
    }

    pub fn show_sections<R>(
        self,
        ctx: &egui::Context,
        add_header: impl FnOnce(&mut Ui, &mut SheetController),
        add_contents: impl FnOnce(&mut Ui, &mut SheetController) -> R,
        add_footer: impl FnOnce(&mut Ui, &mut SheetController),
    ) -> Option<InnerResponse<R>> {
        let theme = current_theme(ctx).unwrap_or_else(CastTheme::light);

        if !*self.open {
            ctx.animate_bool_with_time_and_easing(
                self.id.with("slide"),
                false,
                theme.animation.normal_seconds(),
                egui::emath::easing::cubic_out,
            );
            return None;
        }

        let screen_rect = ctx.content_rect();
        let extent = self.extent.unwrap_or(420.0).max(sheet_extent_floor());
        let (pos, size) = sheet_geometry(screen_rect, self.placement, extent);
        let slide_progress = ctx.animate_bool_with_time_and_easing(
            self.id.with("slide"),
            true,
            theme.animation.normal_seconds(),
            egui::emath::easing::cubic_out,
        );
        let pos = sheet_slide_position(pos, size, self.placement, slide_progress);
        let backdrop_response = egui::Area::new(self.id.with("backdrop"))
            .order(egui::Order::Middle)
            .fixed_pos(screen_rect.min)
            .show(ctx, |ui| {
                ui.painter()
                    .rect_filled(screen_rect, 0.0, dialog_backdrop(&theme));
                let (_, response) = ui.allocate_exact_size(screen_rect.size(), Sense::click());
                response
            });

        let mut controller = SheetController::default();
        let area_response = egui::Area::new(self.id)
            .order(egui::Order::Foreground)
            .fixed_pos(pos)
            .show(ctx, |ui| {
                sheet_shell_frame(&theme, self.placement).show(ui, |ui| {
                    let content_size = sheet_content_size(size, &theme);
                    ui.set_min_size(content_size);
                    ui.set_max_size(content_size);
                    show_sheet_sections(
                        ui,
                        &theme,
                        self.sections,
                        &mut controller,
                        add_header,
                        add_contents,
                        add_footer,
                    )
                })
            });

        if (self.closable && backdrop_response.inner.clicked()) || controller.close_requested {
            *self.open = false;
        }

        Some(area_response.inner)
    }

    pub fn show_with_footer<R>(
        self,
        ctx: &egui::Context,
        add_contents: impl FnOnce(&mut Ui, &mut SheetController) -> R,
        add_footer: impl FnOnce(&mut Ui, &mut SheetController),
    ) -> Option<InnerResponse<R>> {
        let title = self.title.clone();
        let description = self.description.clone();
        let closable = self.closable;

        self.show_sections(
            ctx,
            move |ui, sheet| {
                let mut dialog = DialogController::default();
                paint_dialog_header(
                    ui,
                    &current_theme(ui.ctx()).unwrap_or_else(CastTheme::light),
                    title.as_deref(),
                    description.as_deref(),
                    closable,
                    &mut dialog,
                );
                if dialog.close_requested() {
                    sheet.close();
                }
            },
            add_contents,
            add_footer,
        )
    }
}

#[derive(Default, Debug)]
pub struct SheetController {
    close_requested: bool,
}

impl SheetController {
    pub fn close(&mut self) {
        self.close_requested = true;
    }

    #[must_use]
    pub fn close_requested(&self) -> bool {
        self.close_requested
    }
}

fn show_dialog_sections<R>(
    ui: &mut Ui,
    theme: &CastTheme,
    sections: SurfaceSectionStyle,
    controller: &mut DialogController,
    add_header: impl FnOnce(&mut Ui, &mut DialogController),
    add_contents: impl FnOnce(&mut Ui, &mut DialogController) -> R,
    add_footer: impl FnOnce(&mut Ui, &mut DialogController),
) -> R {
    let previous_spacing = ui.spacing().item_spacing;
    ui.spacing_mut().item_spacing.y = 0.0;

    let header = show_surface_section(ui, theme, sections.header, theme.spacing.lg, |ui| {
        add_header(ui, controller);
    });
    if sections.dividers {
        paint_section_divider(ui, theme, header.response.rect, header.response.rect.max.y);
    }

    let body = show_surface_section(ui, theme, SurfaceChrome::Flat, theme.spacing.lg, |ui| {
        add_contents(ui, controller)
    })
    .inner;

    let footer = show_surface_section(ui, theme, sections.footer, theme.spacing.lg, |ui| {
        add_footer(ui, controller);
    });
    if sections.dividers {
        paint_section_divider(ui, theme, footer.response.rect, footer.response.rect.min.y);
    }

    ui.spacing_mut().item_spacing = previous_spacing;
    body
}

fn show_sheet_sections<R>(
    ui: &mut Ui,
    theme: &CastTheme,
    sections: SurfaceSectionStyle,
    controller: &mut SheetController,
    add_header: impl FnOnce(&mut Ui, &mut SheetController),
    add_contents: impl FnOnce(&mut Ui, &mut SheetController) -> R,
    add_footer: impl FnOnce(&mut Ui, &mut SheetController),
) -> R {
    let previous_spacing = ui.spacing().item_spacing;
    ui.spacing_mut().item_spacing.y = 0.0;

    let header = show_surface_section(ui, theme, sections.header, theme.spacing.lg, |ui| {
        add_header(ui, controller);
    });
    if sections.dividers {
        paint_section_divider(ui, theme, header.response.rect, header.response.rect.max.y);
    }

    let body = show_surface_section(ui, theme, SurfaceChrome::Flat, theme.spacing.lg, |ui| {
        add_contents(ui, controller)
    })
    .inner;

    let footer = show_surface_section(ui, theme, sections.footer, theme.spacing.lg, |ui| {
        add_footer(ui, controller);
    });
    if sections.dividers {
        paint_section_divider(ui, theme, footer.response.rect, footer.response.rect.min.y);
    }

    ui.spacing_mut().item_spacing = previous_spacing;
    body
}

fn sheet_frame(theme: &CastTheme, placement: Placement) -> egui::Frame {
    sheet_shell_frame(theme, placement).inner_margin(egui::Margin::same(theme.spacing.lg as i8))
}

fn sheet_shell_frame(theme: &CastTheme, placement: Placement) -> egui::Frame {
    egui::Frame::new()
        .fill(theme.colors.surface_overlay)
        .stroke(Stroke::new(theme.stroke.sm.max(1.0), theme.colors.border))
        .corner_radius(sheet_corner_radius(theme, placement))
        .shadow(egui::epaint::Shadow {
            offset: [0, 10],
            blur: 28,
            spread: 0,
            color: mix_with_transparent(Color32::BLACK, 0.24),
        })
        .inner_margin(egui::Margin::same(0))
}

fn sheet_corner_radius(theme: &CastTheme, placement: Placement) -> egui::CornerRadius {
    let radius = theme.radius.lg as u8;

    match placement {
        Placement::Left => egui::CornerRadius {
            nw: 0,
            ne: radius,
            sw: 0,
            se: radius,
        },
        Placement::Right => egui::CornerRadius {
            nw: radius,
            ne: 0,
            sw: radius,
            se: 0,
        },
        Placement::Top => egui::CornerRadius {
            nw: 0,
            ne: 0,
            sw: radius,
            se: radius,
        },
        Placement::Bottom => egui::CornerRadius {
            nw: radius,
            ne: radius,
            sw: 0,
            se: 0,
        },
    }
}

fn sheet_geometry(screen: Rect, placement: Placement, extent: f32) -> (Pos2, Vec2) {
    let extent = extent.max(sheet_extent_floor());

    match placement {
        Placement::Left => (
            screen.min,
            egui::vec2(extent.min(screen.width()), screen.height()),
        ),
        Placement::Right => (
            egui::pos2(screen.max.x - extent.min(screen.width()), screen.min.y),
            egui::vec2(extent.min(screen.width()), screen.height()),
        ),
        Placement::Top => (
            screen.min,
            egui::vec2(screen.width(), extent.min(screen.height())),
        ),
        Placement::Bottom => (
            egui::pos2(screen.min.x, screen.max.y - extent.min(screen.height())),
            egui::vec2(screen.width(), extent.min(screen.height())),
        ),
    }
}

fn sheet_slide_position(pos: Pos2, size: Vec2, placement: Placement, progress: f32) -> Pos2 {
    let offset = match placement {
        Placement::Left => egui::vec2(-size.x, 0.0),
        Placement::Right => egui::vec2(size.x, 0.0),
        Placement::Top => egui::vec2(0.0, -size.y),
        Placement::Bottom => egui::vec2(0.0, size.y),
    };

    pos + offset * (1.0 - progress.clamp(0.0, 1.0))
}

fn sheet_content_size(frame_size: Vec2, theme: &CastTheme) -> Vec2 {
    let margin = theme.spacing.lg * 2.0;
    egui::vec2(
        (frame_size.x - margin).max(sheet_extent_floor() - margin),
        (frame_size.y - margin).max(0.0),
    )
}

fn sheet_extent_floor() -> f32 {
    260.0
}

fn paint_dialog_header(
    ui: &mut Ui,
    theme: &CastTheme,
    title: Option<&str>,
    description: Option<&str>,
    closable: bool,
    controller: &mut DialogController,
) {
    let has_header = title.is_some() || description.is_some() || closable;
    if !has_header {
        return;
    }

    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            if let Some(title) = title {
                ui.label(
                    RichText::new(title)
                        .family(theme.typography.heading_sm.family.clone())
                        .size(theme.typography.heading_sm.size)
                        .color(theme.colors.text)
                        .extra_letter_spacing(theme.typography.letter_spacing),
                );
            }
            if let Some(description) = description {
                if title.is_some() {
                    ui.add_space(theme.spacing.xs);
                }
                ui.label(
                    RichText::new(description)
                        .family(theme.typography.small.family.clone())
                        .size(theme.typography.small.size)
                        .color(theme.colors.text_muted)
                        .extra_letter_spacing(theme.typography.letter_spacing),
                );
            }
        });

        if closable {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                if close_icon_button(ui, theme).clicked() {
                    controller.close();
                }
            });
        }
    });

    ui.add_space(theme.spacing.lg);
}

fn close_icon_button(ui: &mut Ui, theme: &CastTheme) -> Response {
    let side = 28.0;
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(side), Sense::click());

    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        let pressed = response.is_pointer_button_down_on();
        let accent = theme.colors.text_muted;
        let fill = if pressed {
            mix_with_transparent(accent, 0.16)
        } else if hovered {
            mix_with_transparent(accent, 0.08)
        } else {
            Color32::TRANSPARENT
        };

        ui.painter().rect(
            rect,
            egui::CornerRadius::same(theme.radius.md as u8),
            fill,
            Stroke::NONE,
            StrokeKind::Outside,
        );

        let center = rect.center();
        let offset = 4.5;
        let stroke = Stroke::new(theme.stroke.md, accent);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialog_defaults_to_closable_with_no_header_text() {
        let mut open = true;
        let dialog = Dialog::new(&mut open, "dialog");

        assert!(dialog.closable);
        assert!(dialog.title.is_none());
        assert!(dialog.description.is_none());
        assert_eq!(dialog.sections, SurfaceSectionStyle::flat());
    }

    #[test]
    fn dialog_width_has_floor() {
        let mut open = true;
        let dialog = Dialog::new(&mut open, "dialog").width(120.0);

        assert_eq!(dialog.width, Some(260.0));
    }

    #[test]
    fn dialog_can_opt_into_muted_sections() {
        let mut open = true;
        let dialog = Dialog::new(&mut open, "dialog").muted_sections();

        assert_eq!(dialog.sections, SurfaceSectionStyle::muted());
    }

    #[test]
    fn dialog_controller_tracks_close_request() {
        let mut controller = DialogController::default();

        assert!(!controller.close_requested());
        controller.close();
        assert!(controller.close_requested());
    }

    #[test]
    fn confirm_dialog_defaults_to_danger_confirmation() {
        let mut open = true;
        let dialog = ConfirmDialog::new(&mut open, "confirm");

        assert_eq!(dialog.intent, Intent::Danger);
        assert_eq!(dialog.confirm_label, "Confirm");
        assert_eq!(dialog.cancel_label, "Cancel");
    }

    #[test]
    fn confirm_dialog_width_has_floor() {
        let mut open = true;
        let dialog = ConfirmDialog::new(&mut open, "confirm").width(120.0);

        assert_eq!(dialog.width, Some(280.0));
    }

    #[test]
    fn sheet_defaults_to_right_side_and_closable() {
        let mut open = true;
        let sheet = Sheet::new(&mut open, "sheet");

        assert_eq!(sheet.placement, Placement::Right);
        assert!(sheet.closable);
        assert!(sheet.title.is_none());
        assert_eq!(sheet.sections, SurfaceSectionStyle::flat());
    }

    #[test]
    fn sheet_extent_has_floor() {
        let mut open = true;
        let sheet = Sheet::new(&mut open, "sheet").width(120.0);

        assert_eq!(sheet.extent, Some(sheet_extent_floor()));
    }

    #[test]
    fn sheet_can_opt_into_muted_sections() {
        let mut open = true;
        let sheet = Sheet::new(&mut open, "sheet").muted_sections();

        assert_eq!(sheet.sections, SurfaceSectionStyle::muted());
    }

    #[test]
    fn sheet_geometry_places_right_sheet_at_screen_edge() {
        let screen = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 700.0));
        let (pos, size) = sheet_geometry(screen, Placement::Right, 320.0);

        assert_eq!(pos, Pos2::new(680.0, 0.0));
        assert_eq!(size, Vec2::new(320.0, 700.0));
    }

    #[test]
    fn sheet_slide_position_offsets_from_attached_edge() {
        let pos = Pos2::new(680.0, 0.0);
        let size = Vec2::new(320.0, 700.0);

        assert_eq!(
            sheet_slide_position(pos, size, Placement::Right, 0.0),
            Pos2::new(1000.0, 0.0)
        );
        assert_eq!(sheet_slide_position(pos, size, Placement::Right, 1.0), pos);
    }

    #[test]
    fn sheet_content_size_accounts_for_frame_margin() {
        let theme = CastTheme::light();
        let size = sheet_content_size(Vec2::new(420.0, 700.0), &theme);

        assert!(size.x < 420.0);
        assert!(size.y < 700.0);
    }

    #[test]
    fn dark_sheet_uses_neutral_border_token() {
        let theme = CastTheme::dark();
        let frame = sheet_frame(&theme, Placement::Right);

        assert_eq!(frame.stroke.color, theme.colors.border);
        assert!(frame.stroke.width >= 1.0);
    }

    #[test]
    fn sheet_corner_radius_only_rounds_exposed_edge() {
        let theme = CastTheme::light();
        let radius = theme.radius.lg as u8;
        let right = sheet_corner_radius(&theme, Placement::Right);
        let bottom = sheet_corner_radius(&theme, Placement::Bottom);

        assert_eq!(right.nw, radius);
        assert_eq!(right.sw, radius);
        assert_eq!(right.ne, 0);
        assert_eq!(right.se, 0);
        assert_eq!(bottom.nw, radius);
        assert_eq!(bottom.ne, radius);
        assert_eq!(bottom.sw, 0);
        assert_eq!(bottom.se, 0);
    }
}
