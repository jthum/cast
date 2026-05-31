use egui::{
    Color32, Response, Sense, StrokeKind, Ui, Widget,
    text::{LayoutJob, TextFormat},
};

use crate::{
    color::{contrast_ratio, mix_oklch, mix_with_transparent, with_alpha},
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
        let theme = theme_for_ui(ui);
        let frame_padding = 3.0;
        let frame = egui::Frame::new()
            .fill(theme.colors.surface_muted)
            .corner_radius(tab_frame_radius(&theme, frame_padding))
            .inner_margin(egui::Margin::symmetric(
                frame_padding as i8,
                frame_padding as i8,
            ))
            .show(ui, |ui| {
                let mut combined: Option<Response> = None;

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = theme.spacing.xs / 2.0;
                    for (index, label) in self.labels.iter().enumerate() {
                        let selected = *self.selected == index;
                        let mut response = nav_item(ui, label, self.size, selected, NavStyle::Tab);
                        if response.clicked() && *self.selected != index {
                            *self.selected = index;
                            response.mark_changed();
                        }
                        combined = Some(match combined.take() {
                            Some(existing) => existing.union(response),
                            None => response,
                        });
                    }
                });

                combined.unwrap_or_else(|| ui.allocate_response(egui::Vec2::ZERO, Sense::hover()))
            });

        frame.response.union(frame.inner)
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
        let frame_padding = 3.0;
        let item_gap = segmented_item_gap(&theme);
        let mut combined: Option<Response> = None;

        let frame = egui::Frame::new()
            .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
            .corner_radius(segmented_frame_radius(&theme, frame_padding))
            .inner_margin(egui::Margin::symmetric(
                frame_padding as i8,
                frame_padding as i8,
            ))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = item_gap;
                    for (index, label) in self.labels.iter().enumerate() {
                        let selected = *self.selected == index;
                        let mut response =
                            nav_item(ui, label, self.size, selected, NavStyle::Segmented);
                        if response.clicked() && *self.selected != index {
                            *self.selected = index;
                            response.mark_changed();
                        }
                        combined = Some(match combined.take() {
                            Some(existing) => existing.union(response),
                            None => response,
                        });
                    }
                });

                combined.unwrap_or_else(|| ui.allocate_response(egui::Vec2::ZERO, Sense::hover()))
            });

        frame.response.union(frame.inner)
    }
}

#[derive(Debug)]
pub struct NavList<'a> {
    selected: &'a mut usize,
    labels: Vec<String>,
    size: Size,
}

impl<'a> NavList<'a> {
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

impl Widget for NavList<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut combined: Option<Response> = None;

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = theme_for_ui(ui).spacing.xs;
            for (index, label) in self.labels.iter().enumerate() {
                let selected = *self.selected == index;
                let mut response = nav_list_item(ui, label, self.size, selected);
                if response.clicked() && *self.selected != index {
                    *self.selected = index;
                    response.mark_changed();
                }
                combined = Some(match combined.take() {
                    Some(existing) => existing.union(response),
                    None => response,
                });
            }
        });

        combined.unwrap_or_else(|| ui.allocate_response(egui::Vec2::ZERO, Sense::hover()))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NavStyle {
    Tab,
    Segmented,
    List,
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
    let horizontal_padding = match style {
        NavStyle::Tab => metrics.padding.x * 1.15,
        NavStyle::Segmented | NavStyle::List => metrics.padding.x * 1.25,
    };
    let vertical_padding = match style {
        NavStyle::Tab => metrics.padding.y * 1.15,
        NavStyle::Segmented | NavStyle::List => metrics.padding.y * 1.1,
    };
    let desired_size = egui::vec2(
        galley.size().x + horizontal_padding * 2.0,
        (galley.size().y + vertical_padding * 2.0).max(metrics.min_height - 4.0),
    );
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        let pressed = response.is_pointer_button_down_on();
        paint_nav_item(ui, &theme, rect, selected, hovered, pressed, style);
        let fg = nav_fg(&theme, selected, hovered, style);
        ui.painter()
            .galley(rect.center() - galley.size() / 2.0, galley, fg);
    }

    response
}

fn nav_list_item(ui: &mut Ui, label: &str, size: Size, selected: bool) -> Response {
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
        ui.available_width()
            .max(galley.size().x + metrics.padding.x * 2.0),
        (galley.size().y + metrics.padding.y * 1.5).max(metrics.min_height - 4.0),
    );
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        let pressed = response.is_pointer_button_down_on();
        let fill = nav_fill(&theme, selected, hovered, pressed, NavStyle::List);
        let border = if selected {
            selected_border(&theme, hovered, pressed)
        } else {
            Color32::TRANSPARENT
        };
        ui.painter().rect(
            rect,
            egui::CornerRadius::same(theme.radius.md as u8),
            fill,
            egui::Stroke::new(theme.stroke.sm, border),
            StrokeKind::Outside,
        );

        let text_pos = egui::pos2(
            rect.min.x + metrics.padding.x,
            rect.center().y - galley.size().y / 2.0,
        );
        ui.painter().galley(
            text_pos,
            galley,
            nav_fg(&theme, selected, hovered, NavStyle::List),
        );
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
    let radius = match style {
        NavStyle::Tab => tab_item_radius(theme),
        NavStyle::Segmented | NavStyle::List => segmented_item_radius(theme),
    };
    let fill = nav_fill(theme, selected, hovered, pressed, style);
    let stroke = match style {
        NavStyle::Tab => egui::Stroke::NONE,
        NavStyle::Segmented if selected => {
            egui::Stroke::new(theme.stroke.sm, selected_border(theme, hovered, pressed))
        }
        NavStyle::Segmented | NavStyle::List => {
            egui::Stroke::new(theme.stroke.sm, Color32::TRANSPARENT)
        }
    };

    ui.painter()
        .rect(rect, radius, fill, stroke, StrokeKind::Outside);
}

fn tab_frame_radius(theme: &CastTheme, frame_padding: f32) -> egui::CornerRadius {
    egui::CornerRadius::same((tab_item_radius_px(theme) + frame_padding).round() as u8)
}

fn tab_item_radius(theme: &CastTheme) -> egui::CornerRadius {
    egui::CornerRadius::same(tab_item_radius_px(theme).round() as u8)
}

fn tab_item_radius_px(theme: &CastTheme) -> f32 {
    theme.radius.md * 2.0
}

fn segmented_frame_radius(theme: &CastTheme, frame_padding: f32) -> egui::CornerRadius {
    egui::CornerRadius::same((theme.radius.md + frame_padding).round() as u8)
}

fn segmented_item_radius(theme: &CastTheme) -> egui::CornerRadius {
    egui::CornerRadius::same(theme.radius.md.round() as u8)
}

fn segmented_item_gap(theme: &CastTheme) -> f32 {
    (theme.stroke.sm * 2.0).max(2.0)
}

fn nav_fill(
    theme: &CastTheme,
    selected: bool,
    hovered: bool,
    pressed: bool,
    style: NavStyle,
) -> Color32 {
    if selected {
        match style {
            NavStyle::Tab => theme.colors.surface,
            NavStyle::Segmented if theme.mode == ThemeMode::Dark => theme.colors.text,
            NavStyle::Segmented | NavStyle::List => selected_fill(theme, hovered, pressed),
        }
    } else if hovered && style == NavStyle::Tab {
        theme.colors.surface_raised
    } else if hovered {
        theme.colors.surface_muted
    } else {
        Color32::TRANSPARENT
    }
}

fn selected_fill(theme: &CastTheme, hovered: bool, pressed: bool) -> Color32 {
    let alpha = if pressed {
        theme.tone.subtle_active_fill_alpha
    } else if hovered {
        theme.tone.subtle_hover_fill_alpha
    } else {
        theme.tone.subtle_fill_alpha
    };
    mix_with_transparent(theme.colors.primary_family.base, alpha)
}

fn selected_border(theme: &CastTheme, hovered: bool, pressed: bool) -> Color32 {
    let alpha = if pressed {
        theme.tone.subtle_active_border_alpha
    } else if hovered {
        theme.tone.subtle_hover_border_alpha
    } else {
        theme.tone.subtle_border_alpha
    };
    mix_with_transparent(theme.colors.primary_family.base, alpha)
}

fn nav_fg(theme: &CastTheme, selected: bool, hovered: bool, style: NavStyle) -> Color32 {
    if selected {
        match style {
            NavStyle::Tab => theme.colors.text,
            NavStyle::Segmented if theme.mode == ThemeMode::Dark => {
                dark_segmented_selected_fg(theme)
            }
            NavStyle::Segmented | NavStyle::List => theme.colors.primary_family.base,
        }
    } else if style == NavStyle::Tab && hovered {
        theme.colors.text
    } else if style == NavStyle::Tab {
        theme.colors.text_muted
    } else if hovered {
        theme.colors.text
    } else {
        with_alpha(theme.colors.text, 225)
    }
}

fn dark_segmented_selected_fg(theme: &CastTheme) -> Color32 {
    let base = theme.colors.primary_family.base;
    if contrast_ratio(theme.colors.text, base) >= 4.5 {
        base
    } else {
        mix_oklch(base, Color32::BLACK, 0.36)
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

    #[test]
    fn tab_radius_follows_theme_radius() {
        let mut sharp = CastTheme::light();
        let mut soft = CastTheme::light();
        sharp.radius.md = 2.0;
        soft.radius.md = 10.0;

        assert!(tab_item_radius(&soft).nw > tab_item_radius(&sharp).nw);
        assert!(tab_frame_radius(&soft, 3.0).nw > tab_frame_radius(&sharp, 3.0).nw);
    }

    #[test]
    fn segmented_frame_radius_accounts_for_inner_padding() {
        let theme = CastTheme::light();
        let frame_radius = segmented_frame_radius(&theme, 3.0);
        let item_radius = segmented_item_radius(&theme);

        assert_eq!(frame_radius.nw, item_radius.nw + 3);
    }

    #[test]
    fn segmented_item_gap_separates_neighboring_interaction_fills() {
        let theme = CastTheme::light();

        assert!(segmented_item_gap(&theme) >= theme.stroke.sm * 2.0);
    }

    #[test]
    fn nav_list_stores_labels() {
        let mut selected = 0;
        let nav = NavList::new(&mut selected, ["Workbench", "Components"]);

        assert_eq!(nav.labels, ["Workbench", "Components"]);
    }

    #[test]
    fn selected_navigation_colors_use_transparent_primary_tints() {
        let theme = CastTheme::light();
        let fill = selected_fill(&theme, false, false);
        let border = selected_border(&theme, false, false);
        let [fill_r, _, _, fill_a] = fill.to_srgba_unmultiplied();
        let [border_r, _, _, border_a] = border.to_srgba_unmultiplied();

        assert!((i16::from(fill_r) - i16::from(theme.colors.primary_family.base.r())).abs() <= 3);
        assert_eq!(fill_a, 13);
        assert!((i16::from(border_r) - i16::from(theme.colors.primary_family.base.r())).abs() <= 3);
        assert_eq!(border_a, 77);
    }

    #[test]
    fn dark_segmented_selection_uses_light_pill_with_primary_text() {
        let theme = CastTheme::dark();
        let fill = nav_fill(&theme, true, false, false, NavStyle::Segmented);
        let fg = nav_fg(&theme, true, false, NavStyle::Segmented);

        assert_eq!(fill, theme.colors.text);
        assert!(contrast_ratio(fill, fg) >= 4.5);
    }

    #[test]
    fn dark_nav_list_keeps_subtle_selected_tint() {
        let theme = CastTheme::dark();
        let fill = nav_fill(&theme, true, false, false, NavStyle::List);
        let [_, _, _, alpha] = fill.to_srgba_unmultiplied();

        assert_ne!(fill, theme.colors.text);
        assert_eq!(alpha, 13);
    }

    #[test]
    fn selected_tabs_use_surface_pill_colors() {
        let theme = CastTheme::light();

        assert_eq!(
            nav_fill(&theme, true, false, false, NavStyle::Tab),
            theme.colors.surface
        );
        assert_eq!(
            nav_fg(&theme, true, false, NavStyle::Tab),
            theme.colors.text
        );
        assert_eq!(
            nav_fg(&theme, false, false, NavStyle::Tab),
            theme.colors.text_muted
        );
        assert_eq!(
            nav_fg(&theme, true, false, NavStyle::Segmented),
            theme.colors.primary_family.base
        );
    }
}
