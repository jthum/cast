use egui::{
    Color32, Response, Sense, StrokeKind, Ui, Widget,
    text::{LayoutJob, TextFormat},
};

use crate::{
    color::with_alpha,
    foundation::Size,
    style::resolve_control_metrics,
    theme::{CastTheme, ThemeMode, theme_for_ui},
};

#[derive(Debug)]
pub struct Tabs<'a> {
    selected: &'a mut usize,
    labels: Vec<String>,
    size: Size,
}

impl<'a> Tabs<'a> {
    #[must_use]
    pub fn new<I, L>(selected: &'a mut usize, labels: I) -> Self
    where
        I: IntoIterator<Item = L>,
        L: Into<String>,
    {
        Self {
            selected,
            labels: labels.into_iter().map(Into::into).collect(),
            size: Size::Medium,
        }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Widget for Tabs<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut combined = ui.allocate_response(egui::Vec2::ZERO, Sense::hover());

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = theme_for_ui(ui).spacing.xs;
            for (index, label) in self.labels.iter().enumerate() {
                let selected = *self.selected == index;
                let mut response = nav_item(ui, label, self.size, selected, NavStyle::Tab);
                if response.clicked() && *self.selected != index {
                    *self.selected = index;
                    response.mark_changed();
                }
                combined = combined.union(response);
            }
        });

        combined
    }
}

#[derive(Debug)]
pub struct SegmentedControl<'a> {
    selected: &'a mut usize,
    labels: Vec<String>,
    size: Size,
}

impl<'a> SegmentedControl<'a> {
    #[must_use]
    pub fn new<I, L>(selected: &'a mut usize, labels: I) -> Self
    where
        I: IntoIterator<Item = L>,
        L: Into<String>,
    {
        Self {
            selected,
            labels: labels.into_iter().map(Into::into).collect(),
            size: Size::Medium,
        }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Widget for SegmentedControl<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let metrics = resolve_control_metrics(&theme, self.size);
        let height = metrics
            .min_height
            .max(theme.components.button.min_height - 4.0);
        let start = ui.cursor().min;
        let mut combined = ui.allocate_response(egui::Vec2::ZERO, Sense::hover());

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            for (index, label) in self.labels.iter().enumerate() {
                let selected = *self.selected == index;
                let mut response = nav_item(ui, label, self.size, selected, NavStyle::Segmented);
                if response.clicked() && *self.selected != index {
                    *self.selected = index;
                    response.mark_changed();
                }
                combined = combined.union(response);
            }
        });

        if ui.is_rect_visible(combined.rect) && combined.rect.is_positive() {
            let frame_rect =
                egui::Rect::from_min_max(start, combined.rect.max).expand2(egui::vec2(
                    theme.spacing.xs,
                    ((height - combined.rect.height()) / 2.0).max(0.0),
                ));
            ui.painter().rect_stroke(
                frame_rect,
                egui::CornerRadius::same(theme.radius.md as u8),
                egui::Stroke::new(theme.stroke.sm, theme.colors.border),
                StrokeKind::Outside,
            );
        }

        combined
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NavStyle {
    Tab,
    Segmented,
}

fn nav_item(ui: &mut Ui, label: &str, size: Size, selected: bool, style: NavStyle) -> Response {
    let theme = theme_for_ui(ui);
    let metrics = resolve_control_metrics(&theme, size);
    let mut font_id = theme.typography.button.clone();
    font_id.size = match size {
        Size::Small => theme.typography.small.size,
        Size::Medium => theme.typography.body.size,
        Size::Large => theme.typography.body.size + 1.0,
    };
    let galley = ui.painter().layout_job(nav_layout_job(
        label.to_owned(),
        font_id,
        theme.typography.letter_spacing,
    ));
    let desired_size = egui::vec2(
        galley.size().x + metrics.padding.x * 1.7,
        (galley.size().y + metrics.padding.y * 1.7).max(metrics.min_height - 4.0),
    );
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        let pressed = response.is_pointer_button_down_on();
        paint_nav_item(ui, &theme, rect, selected, hovered, pressed, style);
        let fg = nav_fg(&theme, selected, hovered);
        ui.painter()
            .galley(rect.center() - galley.size() / 2.0, galley, fg);
    }

    response
}

fn paint_nav_item(
    ui: &Ui,
    theme: &CastTheme,
    rect: egui::Rect,
    selected: bool,
    hovered: bool,
    pressed: bool,
    style: NavStyle,
) {
    let radius = egui::CornerRadius::same(theme.radius.md as u8);
    let fill = nav_fill(theme, selected, hovered, pressed, style);
    let stroke = match style {
        NavStyle::Tab => egui::Stroke::NONE,
        NavStyle::Segmented if selected => {
            egui::Stroke::new(theme.stroke.sm, theme.colors.primary_family.border)
        }
        NavStyle::Segmented => egui::Stroke::new(theme.stroke.sm, Color32::TRANSPARENT),
    };

    ui.painter()
        .rect(rect, radius, fill, stroke, StrokeKind::Outside);

    if style == NavStyle::Tab && selected {
        let line = egui::Rect::from_min_max(
            egui::pos2(rect.min.x + theme.spacing.xs, rect.max.y - 2.0),
            egui::pos2(rect.max.x - theme.spacing.xs, rect.max.y),
        );
        ui.painter().rect_filled(
            line,
            egui::CornerRadius::same(1),
            theme.colors.primary_family.base,
        );
    }
}

fn nav_fill(
    theme: &CastTheme,
    selected: bool,
    hovered: bool,
    pressed: bool,
    style: NavStyle,
) -> Color32 {
    let base = if selected {
        match style {
            NavStyle::Tab => Color32::TRANSPARENT,
            NavStyle::Segmented => theme.colors.primary_family.subtle,
        }
    } else if hovered {
        theme.colors.surface_muted
    } else {
        Color32::TRANSPARENT
    };

    if pressed {
        let anchor = match theme.mode {
            ThemeMode::Light => Color32::BLACK,
            ThemeMode::Dark => Color32::WHITE,
        };
        base.lerp_to_gamma(anchor, 0.10)
    } else {
        base
    }
}

fn nav_fg(theme: &CastTheme, selected: bool, hovered: bool) -> Color32 {
    if selected {
        theme.colors.primary_family.base
    } else if hovered {
        theme.colors.text
    } else {
        with_alpha(theme.colors.text, 225)
    }
}

fn nav_layout_job(text: String, font_id: egui::FontId, letter_spacing: f32) -> LayoutJob {
    LayoutJob::single_section(
        text,
        TextFormat {
            font_id,
            extra_letter_spacing: letter_spacing,
            color: Color32::PLACEHOLDER,
            ..Default::default()
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tabs_store_labels_and_default_size() {
        let mut selected = 0;
        let tabs = Tabs::new(&mut selected, ["One", "Two"]);

        assert_eq!(tabs.labels, ["One", "Two"]);
        assert_eq!(tabs.size, Size::Medium);
    }

    #[test]
    fn segmented_control_can_be_sized() {
        let mut selected = 0;
        let segmented = SegmentedControl::new(&mut selected, ["Light", "Dark"]).size(Size::Small);

        assert_eq!(segmented.size, Size::Small);
    }
}
