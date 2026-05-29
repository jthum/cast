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
            paint_selectable_row_background(ui, &theme, rect, colors);
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
    column_weights: Option<Vec<f32>>,
    right_aligned_columns: Vec<usize>,
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
            column_weights: None,
            right_aligned_columns: Vec::new(),
            size: Size::Medium,
        }
    }

    #[must_use]
    pub fn column_weights<I>(mut self, weights: I) -> Self
    where
        I: IntoIterator<Item = f32>,
    {
        self.column_weights = Some(weights.into_iter().collect());
        self
    }

    #[must_use]
    pub fn right_aligned_columns<I>(mut self, columns: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        self.right_aligned_columns = columns.into_iter().collect();
        self
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
        let column_widths = table_column_widths(width, columns, self.column_weights.as_deref());
        let header_height = table_header_height(self.size);
        let row_height = table_row_height(self.size);
        let table_height = header_height + row_height * self.rows.len() as f32;
        let (rect, table_response) =
            ui.allocate_exact_size(egui::vec2(width, table_height), Sense::hover());
        let mut combined = table_response;

        if ui.is_rect_visible(rect) {
            paint_table_frame(ui, &theme, rect, header_height);
            paint_table_header(
                ui,
                &theme,
                rect,
                header_height,
                &column_widths,
                &self.columns,
            );
        }

        for (index, row) in self.rows.iter().enumerate() {
            let row_rect = egui::Rect::from_min_size(
                egui::pos2(
                    rect.min.x,
                    rect.min.y + header_height + row_height * index as f32,
                ),
                egui::vec2(width, row_height),
            );
            let selected = *self.selected == index;
            let mut response = ui.interact(
                row_rect,
                ui.make_persistent_id(("cast_data_table_row", index)),
                Sense::click(),
            );

            if ui.is_rect_visible(row_rect) {
                paint_table_row(
                    ui,
                    &theme,
                    row_rect,
                    &column_widths,
                    row,
                    selected,
                    self.size,
                    &self.right_aligned_columns,
                    response.hovered(),
                    response.is_pointer_button_down_on(),
                );
            }

            if response.clicked() && *self.selected != index {
                *self.selected = index;
                response.mark_changed();
            }
            combined = combined.union(response);
        }

        combined
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

fn paint_table_frame(ui: &Ui, theme: &CastTheme, rect: egui::Rect, header_height: f32) {
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(theme.radius.lg.round() as u8),
        theme.colors.surface,
        egui::Stroke::new(theme.stroke.sm, theme.colors.border),
        StrokeKind::Outside,
    );

    let header_rect = egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), header_height));
    ui.painter().rect_filled(
        header_rect,
        egui::CornerRadius {
            nw: theme.radius.lg.round() as u8,
            ne: theme.radius.lg.round() as u8,
            sw: 0,
            se: 0,
        },
        theme.colors.surface_muted,
    );
}

fn paint_table_header(
    ui: &mut Ui,
    theme: &CastTheme,
    rect: egui::Rect,
    height: f32,
    column_widths: &[f32],
    columns: &[String],
) {
    let header_rect = egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), height));
    let mut x = header_rect.min.x;

    for (index, column) in columns.iter().enumerate() {
        let column_width = column_widths.get(index).copied().unwrap_or(0.0);
        if index > 0 {
            paint_table_vertical_rule(ui, theme, x, rect);
        }

        let galley = ui.painter().layout_job(row_layout_job(
            column.clone(),
            theme.typography.small.clone(),
            theme.typography.letter_spacing,
        ));
        ui.painter().galley(
            egui::pos2(
                x + table_cell_padding(theme),
                header_rect.center().y - galley.size().y / 2.0,
            ),
            galley,
            with_alpha(theme.colors.text, 190),
        );

        x += column_width;
    }

    ui.painter().line_segment(
        [
            egui::pos2(header_rect.min.x, header_rect.max.y),
            egui::pos2(header_rect.max.x, header_rect.max.y),
        ],
        egui::Stroke::new(theme.stroke.sm, theme.colors.border),
    );
}

#[allow(clippy::too_many_arguments)]
fn paint_table_row(
    ui: &mut Ui,
    theme: &CastTheme,
    rect: egui::Rect,
    column_widths: &[f32],
    cells: &[String],
    selected: bool,
    size: Size,
    right_aligned_columns: &[usize],
    hovered: bool,
    pressed: bool,
) {
    let colors = selectable_row_colors(theme, selected, hovered, pressed);

    paint_table_row_background(ui, rect, colors);
    let mut x = rect.min.x;
    for (index, cell) in cells.iter().enumerate() {
        let column_width = column_widths.get(index).copied().unwrap_or(0.0);
        if index > 0 {
            paint_table_vertical_rule(ui, theme, x, rect);
        }

        let galley = ui.painter().layout_job(row_layout_job(
            cell.clone(),
            table_cell_font(theme, size),
            theme.typography.letter_spacing,
        ));
        let text_x = if right_aligned_columns.contains(&index) {
            x + column_width - table_cell_padding(theme) - galley.size().x
        } else {
            x + table_cell_padding(theme)
        };
        ui.painter().galley(
            egui::pos2(text_x, rect.center().y - galley.size().y / 2.0),
            galley,
            with_alpha(theme.colors.text, 230),
        );
        x += column_width;
    }

    ui.painter().line_segment(
        [
            egui::pos2(rect.min.x, rect.max.y),
            egui::pos2(rect.max.x, rect.max.y),
        ],
        egui::Stroke::new(theme.stroke.sm, theme.colors.border),
    );
}

fn paint_selectable_row_background(
    ui: &Ui,
    theme: &CastTheme,
    rect: egui::Rect,
    colors: IntentColors,
) {
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(theme.radius.md.round() as u8),
        colors.fill,
        egui::Stroke::new(theme.stroke.sm, colors.border),
        StrokeKind::Outside,
    );
}

fn paint_table_row_background(ui: &Ui, rect: egui::Rect, colors: IntentColors) {
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::ZERO, colors.fill);
}

fn paint_table_vertical_rule(ui: &Ui, theme: &CastTheme, x: f32, rect: egui::Rect) {
    ui.painter().line_segment(
        [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
        egui::Stroke::new(theme.stroke.sm, theme.colors.border),
    );
}

fn selectable_row_colors(
    theme: &CastTheme,
    selected: bool,
    hovered: bool,
    pressed: bool,
) -> IntentColors {
    if selected {
        IntentColors {
            fill: if pressed {
                mix_with_transparent(theme.colors.primary_family.base, 0.08)
            } else if hovered {
                mix_with_transparent(theme.colors.primary_family.base, 0.07)
            } else {
                mix_with_transparent(theme.colors.primary_family.base, 0.05)
            },
            fg: theme.colors.text,
            border: Color32::TRANSPARENT,
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

fn table_column_widths(width: f32, columns: usize, weights: Option<&[f32]>) -> Vec<f32> {
    let weights = weights
        .filter(|weights| weights.len() == columns && weights.iter().all(|weight| *weight > 0.0));
    match weights {
        Some(weights) => {
            let total = weights.iter().sum::<f32>();
            weights
                .iter()
                .map(|weight| width * (*weight / total))
                .collect()
        }
        None => vec![width / columns as f32; columns],
    }
}

fn table_cell_padding(theme: &CastTheme) -> f32 {
    theme.spacing.sm
}

fn table_header_height(size: Size) -> f32 {
    match size {
        Size::Small => 32.0,
        Size::Medium => 36.0,
        Size::Large => 40.0,
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
        Size::Small => 32.0,
        Size::Medium => 38.0,
        Size::Large => 44.0,
    }
}

fn table_cell_font(theme: &CastTheme, size: Size) -> egui::FontId {
    match size {
        Size::Small => theme.typography.small.clone(),
        Size::Medium => theme.typography.body.clone(),
        Size::Large => theme.typography.body.clone(),
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
    fn table_header_heights_scale_by_size() {
        assert!(table_header_height(Size::Small) < table_header_height(Size::Medium));
        assert!(table_header_height(Size::Medium) < table_header_height(Size::Large));
    }

    #[test]
    fn table_column_widths_follow_relative_weights() {
        let widths = table_column_widths(600.0, 3, Some(&[2.0, 1.0, 3.0]));

        assert_eq!(widths, vec![200.0, 100.0, 300.0]);
    }

    #[test]
    fn table_column_widths_fall_back_when_weights_do_not_match() {
        let widths = table_column_widths(600.0, 3, Some(&[2.0, 1.0]));

        assert_eq!(widths, vec![200.0, 200.0, 200.0]);
    }

    #[test]
    fn selected_row_uses_muted_background_without_text_override() {
        let theme = CastTheme::light();
        let colors = selectable_row_colors(&theme, true, false, false);

        assert_eq!(
            colors.fill,
            mix_with_transparent(theme.colors.primary_family.base, 0.05)
        );
        assert_eq!(colors.fg, theme.colors.text);
        assert_eq!(colors.border, Color32::TRANSPARENT);
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

    #[test]
    fn data_table_stores_layout_options() {
        let mut selected = 0;
        let table = DataTable::new(&mut selected, ["Name", "Value"], [["Cast", "42"]])
            .column_weights([2.0, 1.0])
            .right_aligned_columns([1])
            .size(Size::Small);

        assert_eq!(table.column_weights, Some(vec![2.0, 1.0]));
        assert_eq!(table.right_aligned_columns, vec![1]);
        assert_eq!(table.size, Size::Small);
    }
}
