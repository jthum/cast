use egui::{
    Color32, InnerResponse, Response, Sense, StrokeKind, Ui,
    text::{LayoutJob, TextFormat},
};

use crate::{
    color::{contrast_ratio, mix_oklch, mix_with_transparent},
    foundation::{Intent, Size, Variant},
    style::resolve_intent_colors,
    theme::{CastTheme, theme_for_ui},
};

#[derive(Debug)]
pub struct Disclosure<'a> {
    open: &'a mut bool,
    title: String,
    subtitle: Option<String>,
    trailing: Option<TrailingContent>,
    enabled: bool,
    size: Size,
}

impl<'a> Disclosure<'a> {
    #[must_use]
    pub fn new(open: &'a mut bool, title: impl Into<String>) -> Self {
        Self {
            open,
            title: title.into(),
            subtitle: None,
            trailing: None,
            enabled: true,
            size: Size::Medium,
        }
    }

    #[must_use]
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    #[must_use]
    pub fn trailing(mut self, trailing: impl Into<String>) -> Self {
        self.trailing = Some(TrailingContent::Text(trailing.into()));
        self
    }

    #[must_use]
    pub fn trailing_badge(mut self, label: impl Into<String>, intent: Intent) -> Self {
        self.trailing = Some(TrailingContent::Badge {
            label: label.into(),
            intent,
        });
        self
    }

    #[must_use]
    pub fn trailing_status_dot(mut self, label: impl Into<String>, intent: Intent) -> Self {
        self.trailing = Some(TrailingContent::StatusDot {
            label: label.into(),
            intent,
        });
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

    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> DisclosureResponse<R> {
        let theme = theme_for_ui(ui);
        let header = DisclosureHeader {
            title: self.title,
            subtitle: self.subtitle,
            trailing: self.trailing,
            open: *self.open,
            enabled: self.enabled,
            size: self.size,
            chrome: HeaderChrome::Filled,
        }
        .show(ui, &theme);

        if self.enabled && header.clicked() {
            *self.open = !*self.open;
        }

        let body = if *self.open {
            ui.add_space(-ui.spacing().item_spacing.y);
            let body = show_disclosure_body(ui, &theme, add_contents);
            paint_disclosure_divider(ui, &theme, header.rect.max.y);
            Some(body)
        } else {
            None
        };

        DisclosureResponse {
            header_response: header,
            body,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AccordionItem {
    title: String,
    subtitle: Option<String>,
    trailing: Option<TrailingContent>,
    enabled: bool,
}

impl AccordionItem {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            trailing: None,
            enabled: true,
        }
    }

    #[must_use]
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    #[must_use]
    pub fn trailing(mut self, trailing: impl Into<String>) -> Self {
        self.trailing = Some(TrailingContent::Text(trailing.into()));
        self
    }

    #[must_use]
    pub fn trailing_badge(mut self, label: impl Into<String>, intent: Intent) -> Self {
        self.trailing = Some(TrailingContent::Badge {
            label: label.into(),
            intent,
        });
        self
    }

    #[must_use]
    pub fn trailing_status_dot(mut self, label: impl Into<String>, intent: Intent) -> Self {
        self.trailing = Some(TrailingContent::StatusDot {
            label: label.into(),
            intent,
        });
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

impl From<&str> for AccordionItem {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for AccordionItem {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Debug)]
pub struct Accordion<'a> {
    open: &'a mut Option<usize>,
    items: Vec<AccordionItem>,
    size: Size,
}

impl<'a> Accordion<'a> {
    #[must_use]
    pub fn new<I, Item>(open: &'a mut Option<usize>, items: I) -> Self
    where
        I: IntoIterator<Item = Item>,
        Item: Into<AccordionItem>,
    {
        Self {
            open,
            items: items.into_iter().map(Into::into).collect(),
            size: Size::Medium,
        }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        mut add_contents: impl FnMut(&mut Ui, usize) -> R,
    ) -> Response {
        let theme = theme_for_ui(ui);
        let mut combined: Option<Response> = None;
        let item_count = self.items.len();

        for (index, item) in self.items.into_iter().enumerate() {
            if index > 0 {
                ui.add_space(-ui.spacing().item_spacing.y);
            }

            let open = *self.open == Some(index);
            let header = DisclosureHeader {
                title: item.title,
                subtitle: item.subtitle,
                trailing: item.trailing,
                open,
                enabled: item.enabled,
                size: self.size,
                chrome: HeaderChrome::Plain,
            }
            .show(ui, &theme);
            let header_bottom = header.rect.max.y;

            if item.enabled && header.clicked() {
                *self.open = next_accordion_open(*self.open, index);
            }

            combined = Some(match combined {
                Some(response) => response.union(header),
                None => header,
            });

            if *self.open == Some(index) {
                ui.add_space(-ui.spacing().item_spacing.y);
                let body = show_disclosure_body(ui, &theme, |ui| add_contents(ui, index));
                if index + 1 < item_count {
                    paint_disclosure_divider(ui, &theme, body.response.rect.max.y);
                }
                if let Some(response) = combined.take() {
                    combined = Some(response.union(body.response));
                }
            } else if index + 1 < item_count {
                paint_disclosure_divider(ui, &theme, header_bottom);
            }
        }

        combined.unwrap_or_else(|| ui.allocate_response(egui::Vec2::ZERO, Sense::hover()))
    }
}

pub struct DisclosureResponse<R> {
    pub header_response: Response,
    pub body: Option<InnerResponse<R>>,
}

impl<R> DisclosureResponse<R> {
    #[must_use]
    pub fn response(&self) -> &Response {
        &self.header_response
    }
}

#[derive(Debug)]
struct DisclosureHeader {
    title: String,
    subtitle: Option<String>,
    trailing: Option<TrailingContent>,
    open: bool,
    enabled: bool,
    size: Size,
    chrome: HeaderChrome,
}

#[derive(Clone, Debug)]
enum TrailingContent {
    Text(String),
    Badge { label: String, intent: Intent },
    StatusDot { label: String, intent: Intent },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HeaderChrome {
    Filled,
    Plain,
}

impl DisclosureHeader {
    fn show(self, ui: &mut Ui, theme: &CastTheme) -> Response {
        let height = disclosure_header_height(self.size, self.subtitle.is_some())
            + disclosure_header_extra_padding(self.chrome);
        let width = ui.available_width().max(160.0);
        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), sense);

        if ui.is_rect_visible(rect) {
            let hovered = self.enabled && response.hovered();
            let pressed = self.enabled && response.is_pointer_button_down_on();
            paint_disclosure_header(ui, theme, rect, self, hovered, pressed);
        }

        response
    }
}

fn show_disclosure_body<R>(
    ui: &mut Ui,
    theme: &CastTheme,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    egui::Frame::new()
        .corner_radius(egui::CornerRadius::same(theme.radius.md.round() as u8))
        .inner_margin(egui::Margin::same(0))
        .show(ui, add_contents)
}

fn paint_disclosure_header(
    ui: &Ui,
    theme: &CastTheme,
    rect: egui::Rect,
    header: DisclosureHeader,
    hovered: bool,
    pressed: bool,
) {
    let fill = disclosure_header_fill(theme, header.chrome, header.open, hovered, pressed);
    let radius = disclosure_header_radius(theme, header.chrome, header.open);
    ui.painter().rect(
        rect,
        radius,
        fill,
        egui::Stroke::new(theme.stroke.sm, Color32::TRANSPARENT),
        StrokeKind::Outside,
    );

    let icon_x = rect.min.x + theme.spacing.sm + 6.0;
    let text_right = rect.max.x - theme.spacing.sm;
    let title_color = if header.enabled {
        theme.colors.text
    } else {
        theme.colors.text_subtle
    };

    let title_font = disclosure_title_font(theme, header.size, header.chrome, header.open);
    let title = ui.painter().layout_job(disclosure_layout_job(
        header.title,
        title_font,
        theme.typography.letter_spacing,
    ));

    let subtitle = header.subtitle.map(|subtitle| {
        ui.painter().layout_job(disclosure_layout_job(
            subtitle,
            theme.typography.caption.clone(),
            theme.typography.letter_spacing,
        ))
    });
    let trailing = header
        .trailing
        .map(|content| layout_trailing_content(ui, theme, content));
    let trailing_width = trailing.as_ref().map_or(0.0, TrailingLayout::width);
    let title_subtitle_gap = theme.spacing.xs;
    let block_height = if let Some(subtitle) = &subtitle {
        title.size().y + title_subtitle_gap + subtitle.size().y
    } else {
        title.size().y
    };
    let title_y = if subtitle.is_some() {
        rect.center().y - block_height / 2.0
    } else {
        rect.center().y - title.size().y / 2.0
    };
    let title_height = title.size().y;
    let icon_y = disclosure_icon_y(rect, title_y, title.size().y, subtitle.is_some());
    let icon_rect =
        egui::Rect::from_center_size(egui::pos2(icon_x, icon_y), egui::vec2(14.0, 14.0));
    paint_disclosure_chevron(
        ui,
        icon_rect,
        header.open,
        if header.enabled {
            theme.colors.text_muted
        } else {
            theme.colors.text_subtle
        },
    );

    let text_x = icon_rect.max.x + theme.spacing.sm;

    ui.painter()
        .galley(egui::pos2(text_x, title_y), title, title_color);

    if let Some(subtitle) = subtitle {
        ui.painter().galley(
            egui::pos2(text_x, title_y + title_height + title_subtitle_gap),
            subtitle,
            theme.colors.text_muted,
        );
    }

    if let Some(trailing) = trailing {
        paint_trailing_content(
            ui,
            theme,
            egui::pos2(text_right - trailing_width, rect.center().y),
            trailing,
        );
    }
}

fn paint_disclosure_divider(ui: &Ui, theme: &CastTheme, y: f32) {
    let rect = ui.max_rect();
    ui.painter().line_segment(
        [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
        egui::Stroke::new(theme.stroke.sm, disclosure_divider_color(theme)),
    );
}

enum TrailingLayout {
    Text(std::sync::Arc<egui::Galley>),
    Badge {
        label: std::sync::Arc<egui::Galley>,
        intent: Intent,
        size: egui::Vec2,
    },
    StatusDot {
        label: std::sync::Arc<egui::Galley>,
        intent: Intent,
        size: egui::Vec2,
    },
}

impl TrailingLayout {
    fn width(&self) -> f32 {
        match self {
            Self::Text(galley) => galley.size().x,
            Self::Badge { size, .. } => size.x,
            Self::StatusDot { size, .. } => size.x,
        }
    }
}

fn layout_trailing_content(ui: &Ui, theme: &CastTheme, content: TrailingContent) -> TrailingLayout {
    match content {
        TrailingContent::Text(text) => {
            TrailingLayout::Text(ui.painter().layout_job(disclosure_layout_job(
                text,
                theme.typography.caption.clone(),
                theme.typography.letter_spacing,
            )))
        }
        TrailingContent::Badge { label, intent } => {
            let galley = ui.painter().layout_job(disclosure_layout_job(
                label,
                theme.typography.caption.clone(),
                theme.typography.letter_spacing,
            ));
            let padding = egui::vec2(theme.spacing.sm, theme.spacing.xs * 0.5);
            let size = egui::vec2(
                galley.size().x + padding.x * 2.0,
                galley.size().y + padding.y * 2.0,
            );

            TrailingLayout::Badge {
                label: galley,
                intent,
                size,
            }
        }
        TrailingContent::StatusDot { label, intent } => {
            let galley = ui.painter().layout_job(disclosure_layout_job(
                label,
                theme.typography.caption.clone(),
                theme.typography.letter_spacing,
            ));
            let padding = egui::vec2(theme.spacing.sm, theme.spacing.xs * 0.5);
            let dot_size = 7.0;
            let dot_gap = theme.spacing.xs + 1.0;
            let size = egui::vec2(
                galley.size().x + padding.x * 2.0 + dot_size + dot_gap,
                galley.size().y + padding.y * 2.0,
            );

            TrailingLayout::StatusDot {
                label: galley,
                intent,
                size,
            }
        }
    }
}

fn paint_trailing_content(
    ui: &Ui,
    theme: &CastTheme,
    center_left: egui::Pos2,
    trailing: TrailingLayout,
) {
    match trailing {
        TrailingLayout::Text(galley) => {
            ui.painter().galley(
                egui::pos2(center_left.x, center_left.y - galley.size().y / 2.0),
                galley,
                theme.colors.text_muted,
            );
        }
        TrailingLayout::Badge {
            label,
            intent,
            size,
        } => {
            let colors = resolve_intent_colors(theme, intent, Variant::Solid);
            let rect = egui::Rect::from_min_size(
                egui::pos2(center_left.x, center_left.y - size.y / 2.0),
                size,
            );
            ui.painter().rect(
                rect,
                egui::CornerRadius::same(theme.components.badge.radius as u8),
                colors.fill,
                egui::Stroke::NONE,
                StrokeKind::Outside,
            );
            ui.painter().galley(
                egui::pos2(
                    rect.center().x - label.size().x / 2.0,
                    rect.center().y - label.size().y / 2.0,
                ),
                label,
                readable_badge_text(colors.fill),
            );
        }
        TrailingLayout::StatusDot {
            label,
            intent,
            size,
        } => {
            let rect = egui::Rect::from_min_size(
                egui::pos2(center_left.x, center_left.y - size.y / 2.0),
                size,
            );
            let border = match theme.mode {
                crate::theme::ThemeMode::Light => theme.colors.border,
                crate::theme::ThemeMode::Dark => {
                    mix_with_transparent(theme.colors.text_muted, 0.28)
                }
            };
            ui.painter().rect(
                rect,
                egui::CornerRadius::same(theme.components.badge.radius as u8),
                Color32::TRANSPARENT,
                egui::Stroke::new(theme.components.badge.border_width, border),
                StrokeKind::Outside,
            );

            let dot_size = 7.0;
            let dot_gap = theme.spacing.xs + 1.0;
            let x = rect.min.x + theme.spacing.sm;
            ui.painter().circle_filled(
                egui::pos2(x + dot_size / 2.0, rect.center().y),
                dot_size / 2.0,
                trailing_dot_color(theme, intent),
            );
            ui.painter().galley(
                egui::pos2(
                    x + dot_size + dot_gap,
                    rect.center().y - label.size().y / 2.0,
                ),
                label,
                theme.colors.text,
            );
        }
    }
}

fn paint_disclosure_chevron(ui: &Ui, rect: egui::Rect, open: bool, color: Color32) {
    let stroke = egui::Stroke::new(1.6, color);
    let center = rect.center();
    let half = 3.5;

    let points = if open {
        [
            egui::pos2(center.x - half, center.y - half / 2.0),
            egui::pos2(center.x, center.y + half / 2.0),
            egui::pos2(center.x + half, center.y - half / 2.0),
        ]
    } else {
        [
            egui::pos2(center.x - half / 2.0, center.y - half),
            egui::pos2(center.x + half / 2.0, center.y),
            egui::pos2(center.x - half / 2.0, center.y + half),
        ]
    };

    ui.painter().line_segment([points[0], points[1]], stroke);
    ui.painter().line_segment([points[1], points[2]], stroke);
}

fn disclosure_header_fill(
    theme: &CastTheme,
    chrome: HeaderChrome,
    _open: bool,
    hovered: bool,
    pressed: bool,
) -> Color32 {
    if pressed {
        mix_oklch(theme.colors.surface, theme.colors.surface_muted, 0.54)
    } else if hovered {
        mix_oklch(theme.colors.surface, theme.colors.surface_muted, 0.38)
    } else if chrome == HeaderChrome::Filled {
        theme.colors.surface_muted
    } else {
        Color32::TRANSPARENT
    }
}

fn disclosure_header_radius(
    theme: &CastTheme,
    chrome: HeaderChrome,
    open: bool,
) -> egui::CornerRadius {
    if chrome == HeaderChrome::Plain {
        return egui::CornerRadius::same(0);
    }

    let radius = theme.radius.md.round() as u8;
    if chrome == HeaderChrome::Filled && open {
        egui::CornerRadius {
            nw: radius,
            ne: radius,
            sw: 0,
            se: 0,
        }
    } else {
        egui::CornerRadius::same(radius)
    }
}

fn disclosure_divider_color(theme: &CastTheme) -> Color32 {
    match theme.mode {
        crate::theme::ThemeMode::Light => theme.colors.border,
        crate::theme::ThemeMode::Dark => mix_with_transparent(theme.colors.text_muted, 0.30),
    }
}

fn readable_badge_text(fill: Color32) -> Color32 {
    if contrast_ratio(fill, Color32::WHITE) >= contrast_ratio(fill, Color32::BLACK) {
        Color32::WHITE
    } else {
        Color32::BLACK
    }
}

fn trailing_dot_color(theme: &CastTheme, intent: Intent) -> Color32 {
    match intent {
        Intent::Neutral => theme.colors.text_muted,
        Intent::Primary => theme.colors.primary_family.base,
        Intent::Secondary => theme.colors.secondary_family.base,
        Intent::Success => theme.colors.success_family.base,
        Intent::Warning => theme.colors.warning_family.base,
        Intent::Danger => theme.colors.danger_family.base,
        Intent::Info => theme.colors.info_family.base,
    }
}

fn disclosure_header_height(size: Size, has_subtitle: bool) -> f32 {
    let base = match size {
        Size::Small => 32.0,
        Size::Medium => 38.0,
        Size::Large => 44.0,
    };

    if has_subtitle { base + 14.0 } else { base }
}

fn disclosure_header_extra_padding(chrome: HeaderChrome) -> f32 {
    match chrome {
        HeaderChrome::Filled => 0.0,
        HeaderChrome::Plain => 4.0,
    }
}

fn disclosure_title_font(
    theme: &CastTheme,
    size: Size,
    chrome: HeaderChrome,
    open: bool,
) -> egui::FontId {
    let mut font = match size {
        Size::Small => theme.typography.small.clone(),
        Size::Medium => theme.typography.body.clone(),
        Size::Large => theme.typography.body.clone(),
    };

    if chrome == HeaderChrome::Plain && open {
        font.family = theme.typography.body_strong.family.clone();
    } else if !matches!(size, Size::Small) {
        font.family = theme.typography.body_strong.family.clone();
    }

    font
}

fn disclosure_icon_y(rect: egui::Rect, title_y: f32, title_height: f32, has_subtitle: bool) -> f32 {
    if has_subtitle {
        title_y + title_height / 2.0
    } else {
        rect.center().y
    }
}

fn next_accordion_open(current: Option<usize>, clicked: usize) -> Option<usize> {
    if current == Some(clicked) {
        None
    } else {
        Some(clicked)
    }
}

fn disclosure_layout_job(text: String, font_id: egui::FontId, letter_spacing: f32) -> LayoutJob {
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
    fn disclosure_header_height_accounts_for_subtitle() {
        assert!(
            disclosure_header_height(Size::Medium, true)
                > disclosure_header_height(Size::Medium, false)
        );
        assert!(
            disclosure_header_height(Size::Small, false)
                < disclosure_header_height(Size::Large, false)
        );
    }

    #[test]
    fn plain_headers_get_extra_internal_padding() {
        assert_eq!(disclosure_header_extra_padding(HeaderChrome::Filled), 0.0);
        assert!(disclosure_header_extra_padding(HeaderChrome::Plain) > 0.0);
    }

    #[test]
    fn open_plain_headers_use_stronger_title_family() {
        let theme = CastTheme::light();

        assert_eq!(
            disclosure_title_font(&theme, Size::Small, HeaderChrome::Plain, true).family,
            theme.typography.body_strong.family
        );
        assert_eq!(
            disclosure_title_font(&theme, Size::Small, HeaderChrome::Plain, false).family,
            theme.typography.small.family
        );
    }

    #[test]
    fn disclosure_icon_aligns_with_title_when_subtitled() {
        let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(240.0, 52.0));

        assert_eq!(disclosure_icon_y(rect, 12.0, 14.0, true), 19.0);
        assert_eq!(disclosure_icon_y(rect, 12.0, 14.0, false), rect.center().y);
    }

    #[test]
    fn accordion_item_can_be_built_from_string() {
        let item = AccordionItem::from("Activity");

        assert_eq!(item.title, "Activity");
        assert!(item.subtitle.is_none());
        assert!(item.enabled);
    }

    #[test]
    fn accordion_open_state_toggles_clicked_item() {
        assert_eq!(next_accordion_open(None, 1), Some(1));
        assert_eq!(next_accordion_open(Some(0), 1), Some(1));
        assert_eq!(next_accordion_open(Some(1), 1), None);
    }

    #[test]
    fn main_disclosure_headers_use_table_header_style_fill() {
        let theme = CastTheme::light();

        assert_eq!(
            disclosure_header_fill(&theme, HeaderChrome::Filled, true, false, false),
            theme.colors.surface_muted
        );
        assert_eq!(
            disclosure_header_fill(&theme, HeaderChrome::Filled, false, false, false),
            theme.colors.surface_muted
        );
    }

    #[test]
    fn open_filled_headers_square_bottom_corners() {
        let theme = CastTheme::light();
        let radius = theme.radius.md.round() as u8;

        assert_eq!(
            disclosure_header_radius(&theme, HeaderChrome::Filled, true),
            egui::CornerRadius {
                nw: radius,
                ne: radius,
                sw: 0,
                se: 0,
            }
        );
        assert_eq!(
            disclosure_header_radius(&theme, HeaderChrome::Filled, false),
            egui::CornerRadius::same(radius)
        );
    }

    #[test]
    fn plain_accordion_headers_are_square_rows() {
        let theme = CastTheme::light();

        assert_eq!(
            disclosure_header_radius(&theme, HeaderChrome::Plain, true),
            egui::CornerRadius::same(0)
        );
    }

    #[test]
    fn disclosure_dividers_follow_table_rule_tint() {
        let theme = CastTheme::light();

        assert_eq!(disclosure_divider_color(&theme), theme.colors.border);
    }

    #[test]
    fn accordion_headers_are_plain_when_idle() {
        let theme = CastTheme::light();

        assert_eq!(
            disclosure_header_fill(&theme, HeaderChrome::Plain, true, false, false),
            Color32::TRANSPARENT
        );
    }

    #[test]
    fn solid_trailing_badges_choose_readable_text() {
        assert_eq!(readable_badge_text(Color32::BLACK), Color32::WHITE);
        assert_eq!(readable_badge_text(Color32::WHITE), Color32::BLACK);
    }
}
