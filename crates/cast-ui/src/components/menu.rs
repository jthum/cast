use egui::{
    Color32, Response, Sense, StrokeKind, Ui, Widget,
    text::{LayoutJob, TextFormat},
};

use crate::{
    color::{mix_with_transparent, with_alpha},
    foundation::{Intent, Size},
    style::{IntentColors, menu_frame, resolve_control_metrics},
    theme::{CastTheme, theme_for_ui},
};

#[derive(Debug)]
pub struct Dropdown<'a> {
    selected: &'a mut usize,
    labels: Vec<String>,
    placeholder: String,
    width: Option<f32>,
    size: Size,
    enabled: bool,
}

impl<'a> Dropdown<'a> {
    #[must_use]
    pub fn new<I, L>(selected: &'a mut usize, labels: I) -> Self
    where
        I: IntoIterator<Item = L>,
        L: Into<String>,
    {
        Self {
            selected,
            labels: labels.into_iter().map(Into::into).collect(),
            placeholder: "Select".to_owned(),
            width: None,
            size: Size::Medium,
            enabled: true,
        }
    }

    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

impl Widget for Dropdown<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let label = self
            .labels
            .get(*self.selected)
            .map(String::as_str)
            .unwrap_or(&self.placeholder);
        let width = self.width.unwrap_or(180.0);
        let mut response = dropdown_trigger(ui, label, width, self.size, self.enabled);
        let mut changed = false;

        if self.enabled {
            egui::Popup::menu(&response)
                .frame(menu_frame(&theme))
                .width(width.max(response.rect.width()))
                .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
                .show(|ui| {
                    ui.set_min_width(width.max(response.rect.width()) - theme.spacing.sm);
                    ui.spacing_mut().item_spacing.y = theme.spacing.xs / 2.0;

                    for (index, label) in self.labels.iter().enumerate() {
                        let item_response =
                            ui.add(MenuItem::new(label).selected(*self.selected == index));
                        if item_response.clicked() && *self.selected != index {
                            *self.selected = index;
                            changed = true;
                        }
                    }
                });
        }

        if changed {
            response.mark_changed();
        }

        response
    }
}

fn dropdown_trigger(ui: &mut Ui, label: &str, width: f32, size: Size, enabled: bool) -> Response {
    let theme = theme_for_ui(ui);
    let metrics = resolve_control_metrics(&theme, size);
    let mut font_id = theme.typography.button.clone();
    font_id.size = match size {
        Size::Small => theme.typography.small.size,
        Size::Medium => theme.typography.body.size,
        Size::Large => theme.typography.body.size + 1.0,
    };
    let galley = ui.painter().layout_job(menu_layout_job(
        label.to_owned(),
        font_id,
        theme.typography.letter_spacing,
    ));
    let desired_size = egui::vec2(
        width.max(galley.size().x + metrics.padding.x * 2.0 + dropdown_icon_space(&theme)),
        metrics.min_height,
    );
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(desired_size, sense);

    if ui.is_rect_visible(rect) {
        let hovered = enabled && response.hovered();
        let pressed = enabled && response.is_pointer_button_down_on();
        let fill = if pressed {
            theme.colors.surface_raised
        } else if hovered {
            theme.colors.surface_muted
        } else {
            Color32::TRANSPARENT
        };
        let fg = if enabled {
            theme.colors.text
        } else {
            theme.colors.text_subtle
        };
        let border = if enabled {
            mix_with_transparent(theme.colors.text, 0.30)
        } else {
            theme.colors.border
        };
        let radius = egui::CornerRadius::same(theme.components.button.radius.round() as u8);

        ui.painter().rect(
            rect,
            radius,
            fill,
            egui::Stroke::new(theme.components.button.border_width, border),
            StrokeKind::Outside,
        );
        ui.painter().galley(
            egui::pos2(
                rect.min.x + metrics.padding.x,
                rect.center().y - galley.size().y / 2.0,
            ),
            galley,
            fg,
        );
        paint_dropdown_chevron(ui, &theme, rect, fg);
    }

    response
}

fn paint_dropdown_chevron(ui: &Ui, theme: &CastTheme, rect: egui::Rect, color: Color32) {
    let points = dropdown_chevron_points(rect, theme);
    let stroke = egui::Stroke::new(theme.stroke.md.max(1.5), color);

    ui.painter().line_segment([points[0], points[1]], stroke);
    ui.painter().line_segment([points[1], points[2]], stroke);
}

fn dropdown_chevron_points(rect: egui::Rect, theme: &CastTheme) -> [egui::Pos2; 3] {
    let center = egui::pos2(rect.max.x - theme.spacing.md, rect.center().y);
    let size = 4.0;

    [
        egui::pos2(center.x - size, center.y - size * 0.45),
        egui::pos2(center.x, center.y + size * 0.55),
        egui::pos2(center.x + size, center.y - size * 0.45),
    ]
}

fn dropdown_icon_space(theme: &CastTheme) -> f32 {
    theme.spacing.lg + theme.spacing.sm
}

#[derive(Clone, Debug)]
pub struct MenuItem {
    label: String,
    intent: Intent,
    selected: bool,
    enabled: bool,
    size: Size,
}

impl MenuItem {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            intent: Intent::Neutral,
            selected: false,
            enabled: true,
            size: Size::Medium,
        }
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    #[must_use]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Widget for MenuItem {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let metrics = resolve_control_metrics(&theme, self.size);
        let mut font_id = theme.typography.button.clone();
        font_id.size = match self.size {
            Size::Small => theme.typography.small.size,
            Size::Medium => theme.typography.body.size,
            Size::Large => theme.typography.body.size + 1.0,
        };
        let galley = ui.painter().layout_job(menu_layout_job(
            self.label,
            font_id,
            theme.typography.letter_spacing,
        ));
        let desired_size = egui::vec2(
            ui.available_width()
                .max(galley.size().x + metrics.padding.x * 2.0),
            (galley.size().y + metrics.padding.y * 1.55).max(metrics.min_height - 6.0),
        );
        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, response) = ui.allocate_exact_size(desired_size, sense);

        if ui.is_rect_visible(rect) {
            let hovered = self.enabled && response.hovered();
            let pressed = self.enabled && response.is_pointer_button_down_on();
            let colors = menu_item_colors(&theme, self.intent, self.selected, hovered, pressed);
            let radius = egui::CornerRadius::same(theme.radius.md.round() as u8);

            ui.painter().rect(
                rect,
                radius,
                colors.fill,
                egui::Stroke::new(theme.stroke.sm, colors.border),
                StrokeKind::Outside,
            );

            if self.selected {
                let accent = egui::Rect::from_min_max(
                    egui::pos2(rect.min.x, rect.min.y + theme.spacing.xs),
                    egui::pos2(rect.min.x + 2.0, rect.max.y - theme.spacing.xs),
                );
                ui.painter().rect_filled(
                    accent,
                    egui::CornerRadius::same(1),
                    theme.colors.primary_family.base,
                );
            }

            let fg = if self.enabled {
                colors.fg
            } else {
                theme.colors.text_subtle
            };
            let text_pos = egui::pos2(
                rect.min.x + metrics.padding.x,
                rect.center().y - galley.size().y / 2.0,
            );
            ui.painter().galley(text_pos, galley, fg);
        }

        response
    }
}

fn menu_item_colors(
    theme: &CastTheme,
    intent: Intent,
    selected: bool,
    hovered: bool,
    pressed: bool,
) -> IntentColors {
    let accent = menu_item_accent(theme, intent);
    let selected_alpha = if pressed {
        0.12
    } else if hovered {
        0.09
    } else {
        0.05
    };

    if selected {
        IntentColors {
            fill: mix_with_transparent(accent, selected_alpha),
            fg: accent,
            border: mix_with_transparent(accent, 0.30),
        }
    } else if pressed {
        IntentColors {
            fill: theme.colors.surface_raised,
            fg: menu_item_fg(theme, intent),
            border: Color32::TRANSPARENT,
        }
    } else if hovered {
        IntentColors {
            fill: theme.colors.surface_muted,
            fg: menu_item_fg(theme, intent),
            border: Color32::TRANSPARENT,
        }
    } else {
        IntentColors {
            fill: Color32::TRANSPARENT,
            fg: menu_item_fg(theme, intent),
            border: Color32::TRANSPARENT,
        }
    }
}

fn menu_item_accent(theme: &CastTheme, intent: Intent) -> Color32 {
    match intent {
        Intent::Neutral | Intent::Primary => theme.colors.primary_family.base,
        Intent::Secondary => theme.colors.secondary_family.base,
        Intent::Success => theme.colors.success_family.base,
        Intent::Warning => theme.colors.warning_family.base,
        Intent::Danger => theme.colors.danger_family.base,
        Intent::Info => theme.colors.info_family.base,
    }
}

fn menu_item_fg(theme: &CastTheme, intent: Intent) -> Color32 {
    match intent {
        Intent::Neutral => with_alpha(theme.colors.text, 225),
        _ => menu_item_accent(theme, intent),
    }
}

fn menu_layout_job(text: String, font_id: egui::FontId, letter_spacing: f32) -> LayoutJob {
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
    fn dropdown_stores_labels_and_default_placeholder() {
        let mut selected = 0;
        let dropdown = Dropdown::new(&mut selected, ["One", "Two"]);

        assert_eq!(dropdown.labels, ["One", "Two"]);
        assert_eq!(dropdown.placeholder, "Select");
    }

    #[test]
    fn selected_menu_item_uses_transparent_accent_tints() {
        let theme = CastTheme::light();
        let colors = menu_item_colors(&theme, Intent::Primary, true, false, false);
        let [_, _, _, fill_alpha] = colors.fill.to_srgba_unmultiplied();
        let [_, _, _, border_alpha] = colors.border.to_srgba_unmultiplied();

        assert_eq!(colors.fg, theme.colors.primary_family.base);
        assert_eq!(fill_alpha, 13);
        assert_eq!(border_alpha, 77);
    }

    #[test]
    fn danger_menu_item_uses_danger_foreground() {
        let theme = CastTheme::light();
        let colors = menu_item_colors(&theme, Intent::Danger, false, false, false);

        assert_eq!(colors.fg, theme.colors.danger_family.base);
    }

    #[test]
    fn dropdown_chevron_points_form_downward_caret() {
        let theme = CastTheme::light();
        let rect = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(180.0, 36.0));
        let points = dropdown_chevron_points(rect, &theme);

        assert!(points[1].y > points[0].y);
        assert!(points[1].y > points[2].y);
        assert_eq!(points[0].y, points[2].y);
    }

    #[test]
    fn dropdown_icon_space_uses_theme_spacing() {
        let theme = CastTheme::light();

        assert_eq!(
            dropdown_icon_space(&theme),
            theme.spacing.lg + theme.spacing.sm
        );
    }
}
