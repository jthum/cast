use egui::{InnerResponse, RichText, Ui};

use crate::{
    components::Button,
    foundation::{Intent, Size, Variant},
    theme::theme_for_ui,
};

#[derive(Debug)]
pub struct Carousel<'a> {
    index: &'a mut usize,
    slide_count: usize,
    width: Option<f32>,
    height: Option<f32>,
    label: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CarouselResponse {
    pub index: usize,
    pub previous_clicked: bool,
    pub next_clicked: bool,
}

impl<'a> Carousel<'a> {
    #[must_use]
    pub fn new(index: &'a mut usize, slide_count: usize) -> Self {
        Self {
            index,
            slide_count: slide_count.max(1),
            width: None,
            height: None,
            label: None,
        }
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(220.0));
        self
    }

    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height.max(96.0));
        self
    }

    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn show(
        self,
        ui: &mut Ui,
        add_slide: impl FnOnce(&mut Ui, usize),
    ) -> InnerResponse<CarouselResponse> {
        let theme = theme_for_ui(ui);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(260.0));
        let height = self.height.unwrap_or(148.0);
        *self.index = (*self.index).min(self.slide_count - 1);
        let mut previous_clicked = false;
        let mut next_clicked = false;

        egui::Frame::new()
            .fill(theme.colors.surface)
            .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
            .corner_radius(egui::CornerRadius::same(theme.radius.lg.round() as u8))
            .inner_margin(egui::Margin::same(theme.spacing.sm as i8))
            .show(ui, |ui| {
                ui.set_width(width);
                ui.set_max_width(width);

                ui.horizontal(|ui| {
                    if let Some(label) = &self.label {
                        ui.label(
                            RichText::new(label)
                                .font(theme.typography.body_strong.clone())
                                .color(theme.colors.text),
                        );
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add(
                                Button::new(">")
                                    .intent(Intent::Neutral)
                                    .variant(Variant::Outline)
                                    .size(Size::Small),
                            )
                            .clicked()
                            && *self.index + 1 < self.slide_count
                        {
                            *self.index += 1;
                            next_clicked = true;
                        }
                        if ui
                            .add(
                                Button::new("<")
                                    .intent(Intent::Neutral)
                                    .variant(Variant::Outline)
                                    .size(Size::Small),
                            )
                            .clicked()
                            && *self.index > 0
                        {
                            *self.index -= 1;
                            previous_clicked = true;
                        }
                    });
                });
                ui.add_space(theme.spacing.xs);

                egui::Frame::new()
                    .fill(theme.colors.surface_muted)
                    .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
                    .corner_radius(egui::CornerRadius::same(theme.radius.md.round() as u8))
                    .inner_margin(egui::Margin::same(theme.spacing.md as i8))
                    .show(ui, |ui| {
                        ui.set_width(width - theme.spacing.sm * 2.0);
                        ui.set_min_height(height);
                        add_slide(ui, *self.index);
                    });

                ui.add_space(theme.spacing.xs);
                ui.horizontal_centered(|ui| {
                    for slide in 0..self.slide_count {
                        paint_carousel_dot(ui, slide == *self.index);
                    }
                });

                CarouselResponse {
                    index: *self.index,
                    previous_clicked,
                    next_clicked,
                }
            })
    }
}

fn paint_carousel_dot(ui: &mut Ui, selected: bool) {
    let theme = theme_for_ui(ui);
    let size = if selected { 8.0 } else { 6.0 };
    let (rect, _) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
    ui.painter().circle_filled(
        rect.center(),
        size / 2.0,
        if selected {
            theme.colors.primary_family.base
        } else {
            theme.colors.border_strong
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carousel_has_at_least_one_slide() {
        let mut index = 8;
        let carousel = Carousel::new(&mut index, 0).width(10.0).height(20.0);

        assert_eq!(carousel.slide_count, 1);
        assert_eq!(carousel.width, Some(220.0));
        assert_eq!(carousel.height, Some(96.0));
    }

    #[test]
    fn carousel_response_records_navigation_flags() {
        let response = CarouselResponse {
            index: 2,
            previous_clicked: true,
            next_clicked: false,
        };

        assert_eq!(response.index, 2);
        assert!(response.previous_clicked);
        assert!(!response.next_clicked);
    }
}
