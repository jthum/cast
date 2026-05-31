use egui::{
    Color32, Response, RichText, Sense, StrokeKind, Ui, Widget,
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

#[derive(Debug)]
pub struct Breadcrumb {
    items: Vec<String>,
    size: Size,
}

impl Breadcrumb {
    #[must_use]
    pub fn new<I, L>(items: I) -> Self
    where
        I: IntoIterator<Item = L>,
        L: Into<String>,
    {
        Self {
            items: items.into_iter().map(Into::into).collect(),
            size: Size::Medium,
        }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Widget for Breadcrumb {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let mut combined: Option<Response> = None;

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = theme.spacing.xs;
            for (index, item) in self.items.iter().enumerate() {
                let current = index + 1 == self.items.len();
                let response = breadcrumb_item(ui, &theme, item, self.size, current);
                combined = Some(match combined.take() {
                    Some(existing) => existing.union(response),
                    None => response,
                });
                if !current {
                    let separator = ui.label(
                        RichText::new("/")
                            .font(theme.typography.caption.clone())
                            .color(theme.colors.text_subtle),
                    );
                    combined = Some(match combined.take() {
                        Some(existing) => existing.union(separator),
                        None => separator,
                    });
                }
            }
        });

        combined.unwrap_or_else(|| ui.allocate_response(egui::Vec2::ZERO, Sense::hover()))
    }
}

#[derive(Debug)]
pub struct Pagination<'a> {
    page: &'a mut usize,
    page_count: usize,
    size: Size,
}

impl<'a> Pagination<'a> {
    #[must_use]
    pub fn new(page: &'a mut usize, page_count: usize) -> Self {
        Self {
            page,
            page_count: page_count.max(1),
            size: Size::Small,
        }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Widget for Pagination<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let page_count = self.page_count;
        *self.page = (*self.page).min(page_count - 1);
        let current = *self.page;
        let mut combined: Option<Response> = None;

        ui.horizontal_wrapped(|ui| {
            let previous = pagination_button(ui, "<", self.size, false, current == 0);
            if previous.clicked() && current > 0 {
                *self.page -= 1;
            }
            combined = Some(previous);

            for item in pagination_items(current, page_count) {
                match item {
                    PaginationItem::Page(index) => {
                        let mut response = pagination_button(
                            ui,
                            &(index + 1).to_string(),
                            self.size,
                            index == current,
                            false,
                        );
                        if response.clicked() && *self.page != index {
                            *self.page = index;
                            response.mark_changed();
                        }
                        combined = Some(match combined.take() {
                            Some(existing) => existing.union(response),
                            None => response,
                        });
                    }
                    PaginationItem::Ellipsis => {
                        let response = ui.label(
                            RichText::new("...")
                                .font(theme_for_ui(ui).typography.caption.clone())
                                .color(theme_for_ui(ui).colors.text_subtle),
                        );
                        combined = Some(match combined.take() {
                            Some(existing) => existing.union(response),
                            None => response,
                        });
                    }
                }
            }

            let next = pagination_button(ui, ">", self.size, false, current + 1 >= page_count);
            if next.clicked() && current + 1 < page_count {
                *self.page += 1;
            }
            combined = Some(match combined.take() {
                Some(existing) => existing.union(next),
                None => next,
            });
        });

        combined.unwrap_or_else(|| ui.allocate_response(egui::Vec2::ZERO, Sense::hover()))
    }
}

#[derive(Clone, Debug)]
enum PaginationItem {
    Page(usize),
    Ellipsis,
}

#[derive(Clone, Debug)]
pub struct SidebarItem {
    label: String,
    badge: Option<String>,
}

impl SidebarItem {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            badge: None,
        }
    }

    #[must_use]
    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }
}

#[derive(Debug)]
pub struct Sidebar<'a> {
    selected: &'a mut usize,
    items: Vec<SidebarItem>,
    title: Option<String>,
    subtitle: Option<String>,
    width: Option<f32>,
}

impl<'a> Sidebar<'a> {
    #[must_use]
    pub fn new<I>(selected: &'a mut usize, items: I) -> Self
    where
        I: IntoIterator<Item = SidebarItem>,
    {
        Self {
            selected,
            items: items.into_iter().collect(),
            title: None,
            subtitle: None,
            width: None,
        }
    }

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(180.0));
        self
    }
}

impl Widget for Sidebar<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().max(180.0));

        egui::Frame::new()
            .fill(theme.colors.surface)
            .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
            .corner_radius(egui::CornerRadius::same(theme.radius.lg as u8))
            .inner_margin(egui::Margin::same(theme.spacing.md as i8))
            .show(ui, |ui| {
                ui.set_width((width - theme.spacing.md * 2.0).max(80.0));
                if let Some(title) = self.title {
                    ui.label(
                        RichText::new(title)
                            .font(theme.typography.body_strong.clone())
                            .color(theme.colors.text),
                    );
                }
                if let Some(subtitle) = self.subtitle {
                    ui.label(
                        RichText::new(subtitle)
                            .font(theme.typography.caption.clone())
                            .color(theme.colors.text_muted),
                    );
                }
                if !self.items.is_empty() {
                    ui.add_space(theme.spacing.sm);
                }
                let mut combined: Option<Response> = None;
                for (index, item) in self.items.iter().enumerate() {
                    let mut response = sidebar_item_ui(ui, &theme, item, *self.selected == index);
                    if response.clicked() && *self.selected != index {
                        *self.selected = index;
                        response.mark_changed();
                    }
                    combined = Some(match combined {
                        Some(existing) => existing.union(response),
                        None => response,
                    });
                }
                combined.unwrap_or_else(|| ui.allocate_response(egui::Vec2::ZERO, Sense::hover()))
            })
            .inner
    }
}

fn breadcrumb_item(
    ui: &mut Ui,
    theme: &CastTheme,
    label: &str,
    size: Size,
    current: bool,
) -> Response {
    let font = match size {
        Size::Small => theme.typography.caption.clone(),
        Size::Medium => theme.typography.small.clone(),
        Size::Large => theme.typography.body.clone(),
    };
    let color = if current {
        theme.colors.text
    } else {
        theme.colors.text_muted
    };
    ui.label(
        RichText::new(label)
            .font(font)
            .color(color)
            .extra_letter_spacing(theme.typography.letter_spacing),
    )
}

fn pagination_button(
    ui: &mut Ui,
    label: &str,
    size: Size,
    selected: bool,
    disabled: bool,
) -> Response {
    let theme = theme_for_ui(ui);
    let metrics = resolve_control_metrics(&theme, size);
    let button_size = egui::vec2(
        metrics.min_height.max(28.0),
        (metrics.min_height - 4.0).max(24.0),
    );
    let sense = if disabled {
        Sense::hover()
    } else {
        Sense::click()
    };
    let (rect, response) = ui.allocate_exact_size(button_size, sense);

    if ui.is_rect_visible(rect) {
        let hovered = response.hovered() && !disabled;
        let fill = if selected {
            selected_fill(&theme, hovered, response.is_pointer_button_down_on())
        } else if hovered {
            theme.colors.surface_muted
        } else {
            Color32::TRANSPARENT
        };
        let border = if selected {
            selected_border(&theme, hovered, response.is_pointer_button_down_on())
        } else {
            theme.colors.border
        };
        ui.painter().rect(
            rect,
            egui::CornerRadius::same(theme.radius.md as u8),
            fill,
            egui::Stroke::new(theme.stroke.sm, border),
            StrokeKind::Outside,
        );
        let fg = if disabled {
            theme.colors.text_subtle
        } else if selected {
            theme.colors.primary_family.base
        } else {
            theme.colors.text
        };
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            theme.typography.small.clone(),
            fg,
        );
    }

    response
}

fn pagination_items(current: usize, page_count: usize) -> Vec<PaginationItem> {
    if page_count <= 7 {
        return (0..page_count).map(PaginationItem::Page).collect();
    }

    let mut items = Vec::new();
    let start = current.saturating_sub(1).max(1);
    let end = (current + 2).min(page_count - 1);

    items.push(PaginationItem::Page(0));
    if start > 1 {
        items.push(PaginationItem::Ellipsis);
    }
    for page in start..end {
        items.push(PaginationItem::Page(page));
    }
    if end < page_count - 1 {
        items.push(PaginationItem::Ellipsis);
    }
    items.push(PaginationItem::Page(page_count - 1));
    items
}

fn sidebar_item_ui(ui: &mut Ui, theme: &CastTheme, item: &SidebarItem, selected: bool) -> Response {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), theme.controls.min_height),
        Sense::click(),
    );
    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        let fill = if selected {
            selected_fill(theme, hovered, response.is_pointer_button_down_on())
        } else if hovered {
            theme.colors.surface_muted
        } else {
            Color32::TRANSPARENT
        };
        ui.painter()
            .rect_filled(rect, egui::CornerRadius::same(theme.radius.md as u8), fill);
        let text_color = if selected {
            theme.colors.primary_family.base
        } else if hovered {
            theme.colors.text
        } else {
            theme.colors.text_muted
        };
        ui.painter().text(
            rect.left_center() + egui::vec2(theme.spacing.sm, 0.0),
            egui::Align2::LEFT_CENTER,
            item.label.as_str(),
            theme.typography.button.clone(),
            text_color,
        );
        if let Some(badge) = &item.badge {
            ui.painter().text(
                rect.right_center() - egui::vec2(theme.spacing.sm, 0.0),
                egui::Align2::RIGHT_CENTER,
                badge.as_str(),
                theme.typography.caption.clone(),
                theme.colors.text_subtle,
            );
        }
    }
    response
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
    fn breadcrumb_collects_items() {
        let breadcrumb = Breadcrumb::new(["Workspace", "Reports"]);

        assert_eq!(breadcrumb.items, ["Workspace", "Reports"]);
        assert_eq!(breadcrumb.size, Size::Medium);
    }

    #[test]
    fn pagination_items_keep_edges_for_long_ranges() {
        let items = pagination_items(5, 12);

        assert!(matches!(items.first(), Some(PaginationItem::Page(0))));
        assert!(matches!(items.last(), Some(PaginationItem::Page(11))));
        assert!(
            items
                .iter()
                .any(|item| matches!(item, PaginationItem::Ellipsis))
        );
    }

    #[test]
    fn sidebar_items_can_carry_badges() {
        let item = SidebarItem::new("Reports").badge("12");
        let mut selected = 0;
        let sidebar = Sidebar::new(&mut selected, [item.clone()])
            .title("Project")
            .subtitle("Workspace");

        assert_eq!(item.badge.as_deref(), Some("12"));
        assert_eq!(sidebar.items.len(), 1);
        assert_eq!(sidebar.title.as_deref(), Some("Project"));
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
