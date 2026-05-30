use egui::{Color32, Id, Response, RichText, Sense, StrokeKind, Ui, Vec2, epaint::Stroke};

use crate::{
    color::mix_with_transparent,
    style::{dialog_backdrop, dialog_frame},
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
    }

    #[test]
    fn dialog_width_has_floor() {
        let mut open = true;
        let dialog = Dialog::new(&mut open, "dialog").width(120.0);

        assert_eq!(dialog.width, Some(260.0));
    }

    #[test]
    fn dialog_controller_tracks_close_request() {
        let mut controller = DialogController::default();

        assert!(!controller.close_requested());
        controller.close();
        assert!(controller.close_requested());
    }
}
