use egui::{
    Color32, Response, Sense, StrokeKind, Ui, Widget,
    text::{LayoutJob, TextFormat},
};

use crate::{
    color::{mix_with_transparent, with_alpha},
    foundation::Size,
    style::IntentColors,
    theme::{CastTheme, theme_for_ui},
};

#[derive(Clone, Debug)]
pub struct ListRow {
    title: String,
    subtitle: Option<String>,
    leading: Option<String>,
    trailing: Option<String>,
    selected: bool,
    enabled: bool,
    size: Size,
}

impl ListRow {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            leading: None,
            trailing: None,
            selected: false,
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
    pub fn leading(mut self, leading: impl Into<String>) -> Self {
        self.leading = Some(leading.into());
        self
    }

    #[must_use]
    pub fn trailing(mut self, trailing: impl Into<String>) -> Self {
        self.trailing = Some(trailing.into());
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

impl Widget for ListRow {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let height = list_row_height(self.size, self.subtitle.is_some());
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
            let colors = selectable_row_colors(&theme, self.selected, hovered, pressed);
            paint_selectable_row_background(ui, &theme, rect, colors, self.selected);
            paint_list_row_content(ui, &theme, rect, self, colors.fg);
        }

        response
    }
}

#[derive(Debug)]
pub struct DataTable<'a> {
    selected: &'a mut usize,
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
    size: Size,
}

impl<'a> DataTable<'a> {
    #[must_use]
    pub fn new<IC, C, IR, R, Cell>(selected: &'a mut usize, columns: IC, rows: IR) -> Self
    where
        IC: IntoIterator<Item = C>,
        C: Into<String>,
        IR: IntoIterator<Item = R>,
        R: IntoIterator<Item = Cell>,
        Cell: Into<String>,
    {
        Self {
            selected,
            columns: columns.into_iter().map(Into::into).collect(),
            rows: rows
                .into_iter()
                .map(|row| row.into_iter().map(Into::into).collect())
                .collect(),
            size: Size::Medium,
        }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Widget for DataTable<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let width = ui.available_width().max(240.0);
        let columns = self.columns.len().max(1);
        let column_width = width / columns as f32;

        let header_response = paint_table_header(ui, &theme, width, column_width, &self.columns);
        let mut combined = Some(header_response);

        for (index, row) in self.rows.iter().enumerate() {
            let selected = *self.selected == index;
            let mut response =
                paint_table_row(ui, &theme, width, column_width, row, selected, self.size);
            if response.clicked() && *self.selected != index {
                *self.selected = index;
                response.mark_changed();
            }
            combined = Some(match combined.take() {
                Some(existing) => existing.union(response),
                None => response,
            });
        }

        combined.unwrap_or_else(|| ui.allocate_response(egui::Vec2::ZERO, Sense::hover()))
    }
}

fn paint_list_row_content(ui: &Ui, theme: &CastTheme, rect: egui::Rect, row: ListRow, fg: Color32) {
    let padding_x = theme.spacing.sm;
    let mut text_x = rect.min.x + padding_x;

    if let Some(leading) = row.leading {
        let galley = ui.painter().layout_job(row_layout_job(
            leading,
            theme.typography.button.clone(),
            theme.typography.letter_spacing,
        ));
        ui.painter().galley(
            egui::pos2(text_x, rect.center().y - galley.size().y / 2.0),
            galley.clone(),
            theme.colors.text_muted,
        );
        text_x += galley.size().x + theme.spacing.sm;
    }

    let title_font = match row.size {
        Size::Small => theme.typography.small.clone(),
        Size::Medium => theme.typography.body.clone(),
        Size::Large => theme.typography.body_strong.clone(),
    };
    let title = ui.painter().layout_job(row_layout_job(
        row.title,
        title_font,
        theme.typography.letter_spacing,
    ));

    if let Some(subtitle) = row.subtitle {
        let subtitle = ui.painter().layout_job(row_layout_job(
            subtitle,
            theme.typography.caption.clone(),
            theme.typography.letter_spacing,
        ));
        let block_height = title.size().y + 2.0 + subtitle.size().y;
        let y = rect.center().y - block_height / 2.0;
        ui.painter().galley(egui::pos2(text_x, y), title, fg);
        ui.painter().galley(
            egui::pos2(text_x, y + theme.typography.body.size + 1.0),
            subtitle,
            theme.colors.text_muted,
        );
    } else {
        ui.painter().galley(
            egui::pos2(text_x, rect.center().y - title.size().y / 2.0),
            title,
            fg,
        );
    }

    if let Some(trailing) = row.trailing {
        let galley = ui.painter().layout_job(row_layout_job(
            trailing,
            theme.typography.caption.clone(),
            theme.typography.letter_spacing,
        ));
        ui.painter().galley(
            egui::pos2(
                rect.max.x - padding_x - galley.size().x,
                rect.center().y - galley.size().y / 2.0,
            ),
            galley,
            theme.colors.text_muted,
        );
    }
}

fn paint_table_header(
    ui: &mut Ui,
    theme: &CastTheme,
    width: f32,
    column_width: f32,
    columns: &[String],
) -> Response {
    let height = 30.0;
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), Sense::hover());
    let radius = egui::CornerRadius {
        nw: theme.radius.md.round() as u8,
        ne: theme.radius.md.round() as u8,
        sw: 0,
        se: 0,
    };

    ui.painter().rect(
        rect,
        radius,
        theme.colors.surface_muted,
        egui::Stroke::new(theme.stroke.sm, theme.colors.border),
        StrokeKind::Outside,
    );
    for (index, column) in columns.iter().enumerate() {
        let galley = ui.painter().layout_job(row_layout_job(
            column.clone(),
            theme.typography.caption.clone(),
            theme.typography.letter_spacing,
        ));
        ui.painter().galley(
            egui::pos2(
                rect.min.x + index as f32 * column_width + theme.spacing.sm,
                rect.center().y - galley.size().y / 2.0,
            ),
            galley,
            theme.colors.text_muted,
        );
    }

    response
}

fn paint_table_row(
    ui: &mut Ui,
    theme: &CastTheme,
    width: f32,
    column_width: f32,
    cells: &[String],
    selected: bool,
    size: Size,
) -> Response {
    let height = table_row_height(size);
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), Sense::click());
    let hovered = response.hovered();
    let pressed = response.is_pointer_button_down_on();
    let colors = selectable_row_colors(theme, selected, hovered, pressed);

    paint_selectable_row_background(ui, theme, rect, colors, selected);
    for (index, cell) in cells.iter().enumerate() {
        let galley = ui.painter().layout_job(row_layout_job(
            cell.clone(),
            theme.typography.small.clone(),
            theme.typography.letter_spacing,
        ));
        ui.painter().galley(
            egui::pos2(
                rect.min.x + index as f32 * column_width + theme.spacing.sm,
                rect.center().y - galley.size().y / 2.0,
            ),
            galley,
            if selected {
                colors.fg
            } else {
                with_alpha(theme.colors.text, 230)
            },
        );
    }

    response
}

fn paint_selectable_row_background(
    ui: &Ui,
    theme: &CastTheme,
    rect: egui::Rect,
    colors: IntentColors,
    selected: bool,
) {
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(theme.radius.md.round() as u8),
        colors.fill,
        egui::Stroke::new(theme.stroke.sm, colors.border),
        StrokeKind::Outside,
    );

    if selected {
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
}

fn selectable_row_colors(
    theme: &CastTheme,
    selected: bool,
    hovered: bool,
    pressed: bool,
) -> IntentColors {
    if selected {
        let alpha = if pressed {
            0.12
        } else if hovered {
            0.09
        } else {
            0.05
        };
        IntentColors {
            fill: mix_with_transparent(theme.colors.primary_family.base, alpha),
            fg: theme.colors.primary_family.base,
            border: mix_with_transparent(theme.colors.primary_family.base, 0.30),
        }
    } else if pressed {
        IntentColors {
            fill: theme.colors.surface_raised,
            fg: theme.colors.text,
            border: Color32::TRANSPARENT,
        }
    } else if hovered {
        IntentColors {
            fill: theme.colors.surface_muted,
            fg: theme.colors.text,
            border: Color32::TRANSPARENT,
        }
    } else {
        IntentColors {
            fill: Color32::TRANSPARENT,
            fg: theme.colors.text,
            border: Color32::TRANSPARENT,
        }
    }
}

fn list_row_height(size: Size, has_subtitle: bool) -> f32 {
    let base = match size {
        Size::Small => 32.0,
        Size::Medium => 38.0,
        Size::Large => 44.0,
    };
    if has_subtitle { base + 14.0 } else { base }
}

fn table_row_height(size: Size) -> f32 {
    match size {
        Size::Small => 30.0,
        Size::Medium => 36.0,
        Size::Large => 42.0,
    }
}

fn row_layout_job(text: String, font_id: egui::FontId, letter_spacing: f32) -> LayoutJob {
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
    fn list_row_height_accounts_for_subtitle() {
        assert!(list_row_height(Size::Medium, true) > list_row_height(Size::Medium, false));
    }

    #[test]
    fn table_row_heights_scale_by_size() {
        assert!(table_row_height(Size::Small) < table_row_height(Size::Medium));
        assert!(table_row_height(Size::Medium) < table_row_height(Size::Large));
    }

    #[test]
    fn selected_row_uses_transparent_primary_tints() {
        let theme = CastTheme::light();
        let colors = selectable_row_colors(&theme, true, false, false);
        let [_, _, _, fill_alpha] = colors.fill.to_srgba_unmultiplied();
        let [_, _, _, border_alpha] = colors.border.to_srgba_unmultiplied();

        assert_eq!(colors.fg, theme.colors.primary_family.base);
        assert_eq!(fill_alpha, 13);
        assert_eq!(border_alpha, 77);
    }

    #[test]
    fn data_table_collects_columns_and_rows() {
        let mut selected = 0;
        let table = DataTable::new(
            &mut selected,
            ["Name", "State"],
            [["Build", "Done"], ["Review", "Pending"]],
        );

        assert_eq!(table.columns, ["Name", "State"]);
        assert_eq!(table.rows.len(), 2);
    }
}
