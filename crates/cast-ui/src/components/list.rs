use egui::{
    Color32, Response, RichText, Sense, StrokeKind, Ui, UiBuilder, Widget,
    text::{LayoutJob, TextFormat},
};

use crate::{
    color::{mix_oklch, mix_with_transparent, with_alpha},
    foundation::Size,
    style::IntentColors,
    theme::{CastTheme, ThemeMode, theme_for_ui},
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

#[derive(Clone, Debug)]
pub struct Table {
    columns: Vec<String>,
    column_weights: Option<Vec<f32>>,
    right_aligned_columns: Vec<usize>,
    selected_rows: Vec<usize>,
    expanded_rows: Vec<usize>,
    expanded_row_height: f32,
    size: Size,
    sticky_body_height: Option<f32>,
    min_column_width: f32,
}

impl Table {
    #[must_use]
    pub fn new<IC, C>(columns: IC) -> Self
    where
        IC: IntoIterator<Item = C>,
        C: Into<String>,
    {
        Self {
            columns: columns.into_iter().map(Into::into).collect(),
            column_weights: None,
            right_aligned_columns: Vec::new(),
            selected_rows: Vec::new(),
            expanded_rows: Vec::new(),
            expanded_row_height: 72.0,
            size: Size::Medium,
            sticky_body_height: None,
            min_column_width: 96.0,
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
    pub fn selected_rows<I>(mut self, rows: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        self.selected_rows = rows.into_iter().collect();
        self
    }

    #[must_use]
    pub fn expanded_rows<I>(mut self, rows: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        self.expanded_rows = rows.into_iter().collect();
        self
    }

    #[must_use]
    pub fn expanded_row_height(mut self, height: f32) -> Self {
        self.expanded_row_height = height.max(32.0);
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn sticky_header(mut self, body_height: f32) -> Self {
        self.sticky_body_height = Some(body_height.max(0.0));
        self
    }

    #[must_use]
    pub fn min_column_width(mut self, width: f32) -> Self {
        self.min_column_width = width.max(24.0);
        self
    }

    pub fn show<F>(self, ui: &mut Ui, row_count: usize, mut add_row: F) -> Response
    where
        F: for<'row> FnMut(&mut TableRow<'row>, usize),
    {
        self.show_with_optional_details(
            ui,
            row_count,
            &mut add_row,
            None::<&mut fn(&mut TableDetailRow<'_>, usize)>,
        )
    }

    pub fn show_with_details<F, D>(
        self,
        ui: &mut Ui,
        row_count: usize,
        mut add_row: F,
        mut add_detail: D,
    ) -> Response
    where
        F: for<'row> FnMut(&mut TableRow<'row>, usize),
        D: for<'detail> FnMut(&mut TableDetailRow<'detail>, usize),
    {
        self.show_with_optional_details(ui, row_count, &mut add_row, Some(&mut add_detail))
    }

    fn show_with_optional_details<F, D>(
        self,
        ui: &mut Ui,
        row_count: usize,
        add_row: &mut F,
        add_detail: Option<&mut D>,
    ) -> Response
    where
        F: for<'row> FnMut(&mut TableRow<'row>, usize),
        D: for<'detail> FnMut(&mut TableDetailRow<'detail>, usize),
    {
        let theme = theme_for_ui(ui);
        let viewport_width = ui.available_width().max(240.0);
        let columns = self.columns.len().max(1);
        let table_width = table_content_width(viewport_width, columns, self.min_column_width);
        let column_widths =
            table_column_widths(table_width, columns, self.column_weights.as_deref());
        let header_height = table_header_height(self.size);
        let row_height = table_row_height(self.size);
        let rows_height = expanded_table_rows_height(
            row_count,
            row_height,
            self.expanded_row_height,
            &self.expanded_rows,
        );
        let body_height = table_body_height(rows_height, self.sticky_body_height);
        let table_height = header_height + body_height;
        let table_id = ui.next_auto_id();

        let output = egui::ScrollArea::horizontal()
            .id_salt(table_id.with("horizontal"))
            .max_width(viewport_width)
            .auto_shrink([false, false])
            .show_viewport(ui, |ui, _viewport| {
                paint_widget_table_surface(
                    ui,
                    &theme,
                    table_id,
                    table_width,
                    table_height,
                    header_height,
                    body_height,
                    rows_height,
                    row_height,
                    self.sticky_body_height,
                    &column_widths,
                    &self.columns,
                    row_count,
                    self.size,
                    &self.right_aligned_columns,
                    &self.selected_rows,
                    &self.expanded_rows,
                    self.expanded_row_height,
                    add_row,
                    add_detail,
                )
            });

        output.inner
    }
}

pub struct TableRow<'a> {
    ui: &'a mut Ui,
    theme: &'a CastTheme,
    rect: egui::Rect,
    column_widths: &'a [f32],
    right_aligned_columns: &'a [usize],
    size: Size,
    row_index: usize,
    column_index: usize,
}

pub struct TableDetailRow<'a> {
    ui: &'a mut Ui,
    theme: &'a CastTheme,
    rect: egui::Rect,
    row_index: usize,
}

impl TableDetailRow<'_> {
    pub fn show(&mut self, add_contents: impl FnOnce(&mut Ui)) {
        let content_rect = table_detail_content_rect(self.theme, self.rect);
        let mut detail_ui = self.ui.new_child(
            UiBuilder::new()
                .max_rect(content_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center))
                .id_salt(("cast_table_detail", self.row_index)),
        );
        detail_ui.set_clip_rect(self.rect.intersect(self.ui.clip_rect()));
        detail_ui.spacing_mut().item_spacing =
            egui::vec2(self.theme.spacing.sm, self.theme.spacing.xs);
        add_contents(&mut detail_ui);
    }
}

impl TableRow<'_> {
    pub fn cell(&mut self, add_contents: impl FnOnce(&mut Ui)) {
        self.cell_with_layout(None, add_contents);
    }

    pub fn centered_cell(&mut self, add_contents: impl FnOnce(&mut Ui)) {
        self.cell_with_layout(
            Some(egui::Layout::centered_and_justified(
                egui::Direction::LeftToRight,
            )),
            add_contents,
        );
    }

    fn cell_with_layout(
        &mut self,
        explicit_layout: Option<egui::Layout>,
        add_contents: impl FnOnce(&mut Ui),
    ) {
        let Some(cell_rect) = self.next_cell_rect() else {
            return;
        };
        let column_index = self.column_index - 1;
        let content_rect = table_cell_content_rect(self.theme, cell_rect);
        let layout = explicit_layout.unwrap_or_else(|| {
            if self.right_aligned_columns.contains(&column_index) {
                egui::Layout::right_to_left(egui::Align::Center)
            } else {
                egui::Layout::left_to_right(egui::Align::Center)
            }
        });
        let mut cell_ui = self.ui.new_child(
            UiBuilder::new()
                .max_rect(content_rect)
                .layout(layout)
                .id_salt(("cast_table_cell", self.row_index, column_index)),
        );
        cell_ui.set_clip_rect(cell_rect.intersect(self.ui.clip_rect()));
        cell_ui.spacing_mut().item_spacing =
            egui::vec2(self.theme.spacing.xs, self.theme.spacing.xs);
        add_contents(&mut cell_ui);
    }

    pub fn text(&mut self, text: impl Into<String>) {
        let theme = self.theme;
        let font = table_cell_font(theme, self.size);
        self.cell(|ui| {
            ui.label(
                RichText::new(text.into())
                    .family(font.family.clone())
                    .size(font.size)
                    .color(with_alpha(theme.colors.text, 230))
                    .extra_letter_spacing(theme.typography.letter_spacing),
            );
        });
    }

    fn next_cell_rect(&mut self) -> Option<egui::Rect> {
        let column_index = self.column_index;
        let column_width = self.column_widths.get(column_index).copied()?;
        let x = self.rect.min.x
            + self
                .column_widths
                .iter()
                .take(column_index)
                .copied()
                .sum::<f32>();
        self.column_index += 1;

        Some(egui::Rect::from_min_size(
            egui::pos2(x, self.rect.min.y),
            egui::vec2(column_width, self.rect.height()),
        ))
    }
}

#[derive(Debug)]
pub struct TextTable<'a> {
    selected: &'a mut usize,
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
    column_weights: Option<Vec<f32>>,
    right_aligned_columns: Vec<usize>,
    size: Size,
    sticky_body_height: Option<f32>,
    min_column_width: f32,
}

impl<'a> TextTable<'a> {
    pub const NO_SELECTION: usize = usize::MAX;

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
            sticky_body_height: None,
            min_column_width: 96.0,
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

    #[must_use]
    pub fn sticky_header(mut self, body_height: f32) -> Self {
        self.sticky_body_height = Some(body_height.max(0.0));
        self
    }

    #[must_use]
    pub fn min_column_width(mut self, width: f32) -> Self {
        self.min_column_width = width.max(24.0);
        self
    }
}

impl Widget for TextTable<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let viewport_width = ui.available_width().max(240.0);
        let columns = self.columns.len().max(1);
        let table_width = table_content_width(viewport_width, columns, self.min_column_width);
        let column_widths =
            table_column_widths(table_width, columns, self.column_weights.as_deref());
        let header_height = table_header_height(self.size);
        let row_height = table_row_height(self.size);
        let rows_height = row_height * self.rows.len() as f32;
        let body_height = table_body_height(rows_height, self.sticky_body_height);
        let table_height = header_height + body_height;
        let table_id = ui.next_auto_id();

        let output = egui::ScrollArea::horizontal()
            .id_salt(table_id.with("horizontal"))
            .max_width(viewport_width)
            .auto_shrink([false, false])
            .show_viewport(ui, |ui, _viewport| {
                paint_text_table_surface(
                    ui,
                    &theme,
                    table_id,
                    table_width,
                    table_height,
                    header_height,
                    body_height,
                    rows_height,
                    row_height,
                    self.sticky_body_height,
                    self.selected,
                    &column_widths,
                    &self.columns,
                    &self.rows,
                    self.size,
                    &self.right_aligned_columns,
                )
            });

        output.inner
    }
}

#[allow(clippy::too_many_arguments)]
fn paint_text_table_surface(
    ui: &mut Ui,
    theme: &CastTheme,
    table_id: egui::Id,
    table_width: f32,
    table_height: f32,
    header_height: f32,
    body_height: f32,
    rows_height: f32,
    row_height: f32,
    sticky_body_height: Option<f32>,
    selected: &mut usize,
    column_widths: &[f32],
    columns: &[String],
    rows: &[Vec<String>],
    size: Size,
    right_aligned_columns: &[usize],
) -> Response {
    let (rect, table_response) =
        ui.allocate_exact_size(egui::vec2(table_width, table_height), Sense::hover());
    let mut combined = table_response;

    if ui.is_rect_visible(rect) {
        paint_table_frame(ui, theme, rect, header_height);
        paint_table_header(ui, theme, rect, header_height, column_widths, columns);
    }

    let body_rect =
        egui::Rect::from_min_max(egui::pos2(rect.min.x, rect.min.y + header_height), rect.max);
    let mut body_ui = ui.new_child(
        UiBuilder::new()
            .max_rect(body_rect)
            .layout(*ui.layout())
            .id_salt(table_id.with("body")),
    );
    body_ui.shrink_clip_rect(body_rect);

    if sticky_body_height.is_some() && rows_height > body_height {
        let scroll_response = egui::ScrollArea::vertical()
            .id_salt(table_id.with("scroll"))
            .max_height(body_height)
            .auto_shrink([false, false])
            .show_viewport(&mut body_ui, |ui, viewport| {
                paint_table_rows_viewport(
                    ui,
                    theme,
                    table_width,
                    rows_height,
                    row_height,
                    viewport,
                    selected,
                    column_widths,
                    rows,
                    size,
                    right_aligned_columns,
                )
            });
        combined = combined.union(scroll_response.inner);
    } else {
        combined = combined.union(paint_table_rows_viewport(
            &mut body_ui,
            theme,
            table_width,
            rows_height,
            row_height,
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(table_width, rows_height)),
            selected,
            column_widths,
            rows,
            size,
            right_aligned_columns,
        ));
    }

    if ui.is_rect_visible(rect) {
        paint_table_outline(ui, theme, rect);
    }

    combined
}

#[allow(clippy::too_many_arguments)]
fn paint_widget_table_surface<F, D>(
    ui: &mut Ui,
    theme: &CastTheme,
    table_id: egui::Id,
    table_width: f32,
    table_height: f32,
    header_height: f32,
    body_height: f32,
    rows_height: f32,
    row_height: f32,
    sticky_body_height: Option<f32>,
    column_widths: &[f32],
    columns: &[String],
    row_count: usize,
    size: Size,
    right_aligned_columns: &[usize],
    selected_rows: &[usize],
    expanded_rows: &[usize],
    expanded_row_height: f32,
    add_row: &mut F,
    add_detail: Option<&mut D>,
) -> Response
where
    F: for<'row> FnMut(&mut TableRow<'row>, usize),
    D: for<'detail> FnMut(&mut TableDetailRow<'detail>, usize),
{
    let (rect, table_response) =
        ui.allocate_exact_size(egui::vec2(table_width, table_height), Sense::hover());
    let mut combined = table_response;

    if ui.is_rect_visible(rect) {
        paint_table_frame(ui, theme, rect, header_height);
        paint_table_header(ui, theme, rect, header_height, column_widths, columns);
    }

    let body_rect =
        egui::Rect::from_min_max(egui::pos2(rect.min.x, rect.min.y + header_height), rect.max);
    let mut body_ui = ui.new_child(
        UiBuilder::new()
            .max_rect(body_rect)
            .layout(*ui.layout())
            .id_salt(table_id.with("body")),
    );
    body_ui.shrink_clip_rect(body_rect);

    if sticky_body_height.is_some() && rows_height > body_height {
        let scroll_response = egui::ScrollArea::vertical()
            .id_salt(table_id.with("scroll"))
            .max_height(body_height)
            .auto_shrink([false, false])
            .show_viewport(&mut body_ui, |ui, viewport| {
                paint_widget_table_rows_viewport(
                    ui,
                    theme,
                    table_width,
                    rows_height,
                    row_height,
                    viewport,
                    column_widths,
                    row_count,
                    size,
                    right_aligned_columns,
                    selected_rows,
                    expanded_rows,
                    expanded_row_height,
                    add_row,
                    add_detail,
                )
            });
        combined = combined.union(scroll_response.inner);
    } else {
        combined = combined.union(paint_widget_table_rows_viewport(
            &mut body_ui,
            theme,
            table_width,
            rows_height,
            row_height,
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(table_width, rows_height)),
            column_widths,
            row_count,
            size,
            right_aligned_columns,
            selected_rows,
            expanded_rows,
            expanded_row_height,
            add_row,
            add_detail,
        ));
    }

    if ui.is_rect_visible(rect) {
        paint_table_outline(ui, theme, rect);
    }

    combined
}

#[allow(clippy::too_many_arguments)]
fn paint_table_rows_viewport(
    ui: &mut Ui,
    theme: &CastTheme,
    width: f32,
    rows_height: f32,
    row_height: f32,
    viewport: egui::Rect,
    selected_index: &mut usize,
    column_widths: &[f32],
    rows: &[Vec<String>],
    size: Size,
    right_aligned_columns: &[usize],
) -> Response {
    let (_, content_rect) = ui.allocate_space(egui::vec2(width, rows_height));
    let content_response = ui.interact(
        content_rect,
        ui.make_persistent_id("cast_text_table_rows"),
        Sense::hover(),
    );
    let mut combined = content_response;
    let visible_start = (viewport.min.y / row_height).floor().max(0.0) as usize;
    let visible_end = ((viewport.max.y / row_height).ceil() as usize + 1).min(rows.len());

    for (index, row) in rows
        .iter()
        .enumerate()
        .take(visible_end)
        .skip(visible_start)
    {
        let row_rect = egui::Rect::from_min_size(
            egui::pos2(
                content_rect.min.x,
                content_rect.min.y + row_height * index as f32,
            ),
            egui::vec2(width, row_height),
        );
        let selected = *selected_index == index;
        let last_row = index + 1 == rows.len();
        let response = ui.interact(
            row_rect,
            ui.make_persistent_id(("cast_text_table_row", index)),
            Sense::hover(),
        );

        if ui.is_rect_visible(row_rect) {
            paint_table_row(
                ui,
                theme,
                row_rect,
                column_widths,
                row,
                selected,
                last_row,
                size,
                right_aligned_columns,
                response.hovered(),
                response.is_pointer_button_down_on(),
            );
        }

        combined = combined.union(response);
    }

    combined
}

#[allow(clippy::too_many_arguments)]
fn paint_widget_table_rows_viewport<F, D>(
    ui: &mut Ui,
    theme: &CastTheme,
    width: f32,
    rows_height: f32,
    row_height: f32,
    _viewport: egui::Rect,
    column_widths: &[f32],
    row_count: usize,
    size: Size,
    right_aligned_columns: &[usize],
    selected_rows: &[usize],
    expanded_rows: &[usize],
    expanded_row_height: f32,
    add_row: &mut F,
    mut add_detail: Option<&mut D>,
) -> Response
where
    F: for<'row> FnMut(&mut TableRow<'row>, usize),
    D: for<'detail> FnMut(&mut TableDetailRow<'detail>, usize),
{
    let (_, content_rect) = ui.allocate_space(egui::vec2(width, rows_height));
    let content_response = ui.interact(
        content_rect,
        ui.make_persistent_id("cast_table_rows"),
        Sense::hover(),
    );
    let mut combined = content_response;
    let mut y = content_rect.min.y;

    for index in 0..row_count {
        let expanded = expanded_rows.contains(&index);
        let last_row = index + 1 == row_count && !expanded;
        let row_rect = egui::Rect::from_min_size(
            egui::pos2(content_rect.min.x, y),
            egui::vec2(width, row_height),
        );
        let selected = selected_rows.contains(&index);
        let response = ui.interact(
            row_rect,
            ui.make_persistent_id(("cast_table_row", index)),
            Sense::hover(),
        );

        if ui.is_rect_visible(row_rect) {
            let colors = table_row_colors(
                theme,
                selected,
                !selected && response.hovered(),
                response.is_pointer_button_down_on(),
            );
            paint_table_row_background(ui, theme, row_rect, colors, last_row);
            paint_table_row_rules(ui, theme, row_rect, column_widths, last_row);

            let mut row = TableRow {
                ui,
                theme,
                rect: row_rect,
                column_widths,
                right_aligned_columns,
                size,
                row_index: index,
                column_index: 0,
            };
            add_row(&mut row, index);
        }

        combined = combined.union(response);

        y += row_height;

        if expanded {
            let detail_rect = egui::Rect::from_min_size(
                egui::pos2(content_rect.min.x, y),
                egui::vec2(width, expanded_row_height),
            );
            let last_detail = index + 1 == row_count;
            let detail_response = ui.interact(
                detail_rect,
                ui.make_persistent_id(("cast_table_detail_row", index)),
                Sense::hover(),
            );

            if ui.is_rect_visible(detail_rect) {
                paint_table_detail_row(ui, theme, detail_rect, last_detail);
                if let Some(add_detail) = add_detail.as_deref_mut() {
                    let mut detail = TableDetailRow {
                        ui,
                        theme,
                        rect: detail_rect,
                        row_index: index,
                    };
                    add_detail(&mut detail, index);
                }
            }

            combined = combined.union(detail_response);
            y += expanded_row_height;
        }
    }

    combined
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
        egui::Stroke::NONE,
        StrokeKind::Outside,
    );

    let header_rect = egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), header_height));
    let header_fill_rect = header_rect.expand2(egui::vec2(0.5, 0.0));
    ui.painter().rect_filled(
        header_fill_rect,
        egui::CornerRadius {
            nw: theme.radius.lg.round() as u8,
            ne: theme.radius.lg.round() as u8,
            sw: 0,
            se: 0,
        },
        table_header_fill(theme),
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
            paint_table_rule(ui, x, header_rect, table_internal_rule_color(theme));
        }

        let galley = ui.painter().layout_job(row_layout_job(
            column.clone(),
            table_header_font(theme),
            theme.typography.letter_spacing,
        ));
        ui.painter().galley(
            egui::pos2(
                x + table_cell_padding(theme),
                header_rect.center().y - galley.size().y / 2.0,
            ),
            galley,
            table_header_text_color(theme),
        );

        x += column_width;
    }

    ui.painter().line_segment(
        [
            egui::pos2(header_rect.min.x, header_rect.max.y),
            egui::pos2(header_rect.max.x, header_rect.max.y),
        ],
        egui::Stroke::new(theme.stroke.sm, table_internal_rule_color(theme)),
    );
}

fn paint_table_outline(ui: &Ui, theme: &CastTheme, rect: egui::Rect) {
    ui.painter().rect_stroke(
        rect,
        egui::CornerRadius::same(theme.radius.lg.round() as u8),
        egui::Stroke::new(table_outline_width(theme), table_outline_color(theme)),
        StrokeKind::Outside,
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
    last_row: bool,
    size: Size,
    right_aligned_columns: &[usize],
    hovered: bool,
    pressed: bool,
) {
    let colors = table_row_colors(theme, selected, hovered, pressed);

    paint_table_row_background(ui, theme, rect, colors, last_row);
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

    if !last_row {
        paint_table_horizontal_rule(ui, theme, rect.max.y, rect);
    }
}

fn paint_table_row_rules(
    ui: &Ui,
    theme: &CastTheme,
    rect: egui::Rect,
    column_widths: &[f32],
    last_row: bool,
) {
    let mut x = rect.min.x;
    for (index, column_width) in column_widths.iter().copied().enumerate() {
        if index > 0 {
            paint_table_vertical_rule(ui, theme, x, rect);
        }
        x += column_width;
    }

    if !last_row {
        paint_table_horizontal_rule(ui, theme, rect.max.y, rect);
    }
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

fn paint_table_row_background(
    ui: &Ui,
    theme: &CastTheme,
    rect: egui::Rect,
    colors: IntentColors,
    last_row: bool,
) {
    let radius = if last_row {
        egui::CornerRadius {
            nw: 0,
            ne: 0,
            sw: theme.radius.lg.round() as u8,
            se: theme.radius.lg.round() as u8,
        }
    } else {
        egui::CornerRadius::ZERO
    };
    ui.painter().rect_filled(rect, radius, colors.fill);
}

fn paint_table_detail_row(ui: &Ui, theme: &CastTheme, rect: egui::Rect, last_row: bool) {
    let radius = if last_row {
        egui::CornerRadius {
            nw: 0,
            ne: 0,
            sw: theme.radius.lg.round() as u8,
            se: theme.radius.lg.round() as u8,
        }
    } else {
        egui::CornerRadius::ZERO
    };

    ui.painter().rect_filled(
        rect,
        radius,
        mix_with_transparent(theme.colors.primary_family.base, 0.018),
    );
    if !last_row {
        paint_table_horizontal_rule(ui, theme, rect.max.y, rect);
    }
}

fn paint_table_vertical_rule(ui: &Ui, theme: &CastTheme, x: f32, rect: egui::Rect) {
    paint_table_rule(ui, x, rect, table_internal_rule_color(theme));
}

fn paint_table_horizontal_rule(ui: &Ui, theme: &CastTheme, y: f32, rect: egui::Rect) {
    ui.painter().line_segment(
        [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
        egui::Stroke::new(theme.stroke.sm, table_internal_rule_color(theme)),
    );
}

fn paint_table_rule(ui: &Ui, x: f32, rect: egui::Rect, color: Color32) {
    ui.painter().line_segment(
        [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
        egui::Stroke::new(1.0, color),
    );
}

fn table_header_fill(theme: &CastTheme) -> Color32 {
    mix_with_transparent(theme.colors.primary_family.base, 0.05)
}

fn table_header_text_color(theme: &CastTheme) -> Color32 {
    match theme.mode {
        ThemeMode::Light => theme.colors.text,
        ThemeMode::Dark => mix_oklch(theme.colors.text, theme.colors.surface, 0.10),
    }
}

fn table_header_font(theme: &CastTheme) -> egui::FontId {
    let mut font = theme.typography.strong.clone();
    font.size = theme.typography.small.size;
    font
}

fn table_outline_width(theme: &CastTheme) -> f32 {
    theme.stroke.md.max(theme.stroke.sm)
}

fn table_outline_color(theme: &CastTheme) -> Color32 {
    match theme.mode {
        ThemeMode::Light => mix_with_transparent(theme.colors.primary_family.base, 0.16),
        ThemeMode::Dark => mix_with_transparent(theme.colors.text_muted, 0.34),
    }
}

fn table_internal_rule_color(theme: &CastTheme) -> Color32 {
    match theme.mode {
        ThemeMode::Light => mix_with_transparent(theme.colors.primary_family.base, 0.10),
        ThemeMode::Dark => mix_with_transparent(theme.colors.text_muted, 0.24),
    }
}

fn table_hover_fill(theme: &CastTheme, pressed: bool) -> Color32 {
    mix_with_transparent(
        theme.colors.primary_family.base,
        if pressed { 0.035 } else { 0.025 },
    )
}

fn table_row_colors(
    theme: &CastTheme,
    selected: bool,
    hovered: bool,
    pressed: bool,
) -> IntentColors {
    if selected {
        IntentColors {
            fill: table_hover_fill(theme, false),
            fg: theme.colors.text,
            border: Color32::TRANSPARENT,
        }
    } else if pressed || hovered {
        IntentColors {
            fill: table_hover_fill(theme, pressed),
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
            fill: mix_with_transparent(theme.colors.primary_family.base, 0.05),
            fg: theme.colors.text,
            border: Color32::TRANSPARENT,
        }
    } else if hovered {
        IntentColors {
            fill: mix_with_transparent(theme.colors.primary_family.base, 0.025),
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

fn table_content_width(viewport_width: f32, columns: usize, min_column_width: f32) -> f32 {
    viewport_width.max(columns as f32 * min_column_width)
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

fn table_cell_content_rect(theme: &CastTheme, cell_rect: egui::Rect) -> egui::Rect {
    let horizontal_padding =
        table_cell_padding(theme).min(((cell_rect.width() - 24.0) / 2.0).max(theme.spacing.xs));
    cell_rect.shrink2(egui::vec2(horizontal_padding, 0.0))
}

fn table_detail_content_rect(theme: &CastTheme, detail_rect: egui::Rect) -> egui::Rect {
    detail_rect.shrink2(egui::vec2(
        table_cell_padding(theme) + theme.spacing.xs,
        theme.spacing.md,
    ))
}

fn table_body_height(rows_height: f32, sticky_body_height: Option<f32>) -> f32 {
    sticky_body_height
        .map(|height| height.min(rows_height))
        .unwrap_or(rows_height)
}

fn expanded_table_rows_height(
    row_count: usize,
    row_height: f32,
    expanded_row_height: f32,
    expanded_rows: &[usize],
) -> f32 {
    let expanded_count = expanded_rows
        .iter()
        .copied()
        .filter(|index| *index < row_count)
        .count();

    row_height * row_count as f32 + expanded_row_height * expanded_count as f32
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
    fn table_content_width_expands_for_many_columns() {
        assert_eq!(table_content_width(600.0, 3, 96.0), 600.0);
        assert_eq!(table_content_width(600.0, 8, 96.0), 768.0);
    }

    #[test]
    fn table_chrome_uses_local_primary_subtle_treatment() {
        let theme = CastTheme::light();

        assert_eq!(
            table_header_fill(&theme),
            mix_with_transparent(theme.colors.primary_family.base, 0.05)
        );
        assert_eq!(table_header_text_color(&theme), theme.colors.text);
        assert_eq!(
            table_outline_color(&theme),
            mix_with_transparent(theme.colors.primary_family.base, 0.16)
        );
        assert_eq!(
            table_internal_rule_color(&theme),
            mix_with_transparent(theme.colors.primary_family.base, 0.10)
        );
        assert_eq!(
            table_header_font(&theme).family,
            theme.typography.strong.family
        );
        assert_eq!(table_header_font(&theme).size, theme.typography.small.size);
        assert_ne!(table_internal_rule_color(&theme), theme.colors.border);
        assert_ne!(
            table_outline_color(&theme),
            table_internal_rule_color(&theme)
        );
    }

    #[test]
    fn dark_table_rules_are_more_visible() {
        let theme = CastTheme::dark();

        assert_eq!(
            table_outline_color(&theme),
            mix_with_transparent(theme.colors.text_muted, 0.34)
        );
        assert_eq!(
            table_internal_rule_color(&theme),
            mix_with_transparent(theme.colors.text_muted, 0.24)
        );
    }

    #[test]
    fn table_hover_uses_lighter_primary_subtle_fill() {
        let theme = CastTheme::light();
        let colors = table_row_colors(&theme, false, true, false);

        assert_eq!(colors.fill, table_hover_fill(&theme, false));
        assert_eq!(
            colors.fill,
            mix_with_transparent(theme.colors.primary_family.base, 0.025)
        );
        assert_ne!(colors.fill, table_header_fill(&theme));
        assert_eq!(colors.fg, theme.colors.text);
    }

    #[test]
    fn selected_table_hover_keeps_selected_fill() {
        let theme = CastTheme::light();
        let colors = table_row_colors(&theme, true, true, false);

        assert_eq!(colors.fill, table_hover_fill(&theme, false));
        assert_eq!(colors.fg, theme.colors.text);
    }

    #[test]
    fn sticky_table_body_height_caps_rows() {
        assert_eq!(table_body_height(640.0, Some(320.0)), 320.0);
        assert_eq!(table_body_height(160.0, Some(320.0)), 160.0);
        assert_eq!(table_body_height(640.0, None), 640.0);
    }

    #[test]
    fn expanded_table_rows_add_detail_height() {
        assert_eq!(expanded_table_rows_height(4, 32.0, 64.0, &[1, 3]), 256.0);
        assert_eq!(expanded_table_rows_height(4, 32.0, 64.0, &[4, 8]), 128.0);
    }

    #[test]
    fn table_stores_expanded_row_options() {
        let table = Table::new(["Name"])
            .expanded_rows([0, 2])
            .expanded_row_height(88.0);

        assert_eq!(table.expanded_rows, [0, 2]);
        assert_eq!(table.expanded_row_height, 88.0);
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
    fn hovered_rows_use_light_primary_tint() {
        let theme = CastTheme::light();
        let colors = selectable_row_colors(&theme, false, true, false);

        assert_eq!(
            colors.fill,
            mix_with_transparent(theme.colors.primary_family.base, 0.025)
        );
        assert_eq!(colors.fg, theme.colors.text);
    }

    #[test]
    fn text_table_collects_columns_and_rows() {
        let mut selected = 0;
        let table = TextTable::new(
            &mut selected,
            ["Name", "State"],
            [["Build", "Done"], ["Review", "Pending"]],
        );

        assert_eq!(table.columns, ["Name", "State"]);
        assert_eq!(table.rows.len(), 2);
    }

    #[test]
    fn text_table_stores_layout_options() {
        let mut selected = 0;
        let table = TextTable::new(&mut selected, ["Name", "Value"], [["Cast", "42"]])
            .column_weights([2.0, 1.0])
            .right_aligned_columns([1])
            .size(Size::Small)
            .sticky_header(320.0)
            .min_column_width(128.0);

        assert_eq!(table.column_weights, Some(vec![2.0, 1.0]));
        assert_eq!(table.right_aligned_columns, vec![1]);
        assert_eq!(table.size, Size::Small);
        assert_eq!(table.sticky_body_height, Some(320.0));
        assert_eq!(table.min_column_width, 128.0);
    }

    #[test]
    fn text_table_can_represent_no_external_selection() {
        let mut selected = TextTable::NO_SELECTION;
        let table = TextTable::new(&mut selected, ["Name"], [["Cast"]]);

        assert_eq!(*table.selected, TextTable::NO_SELECTION);
    }

    #[test]
    fn table_collects_columns_and_layout_options() {
        let table = Table::new(["Name", "Status", "Value"])
            .column_weights([2.0, 1.0, 1.0])
            .right_aligned_columns([2])
            .selected_rows([1, 3])
            .size(Size::Small)
            .sticky_header(280.0)
            .min_column_width(120.0);

        assert_eq!(table.columns, ["Name", "Status", "Value"]);
        assert_eq!(table.column_weights, Some(vec![2.0, 1.0, 1.0]));
        assert_eq!(table.right_aligned_columns, vec![2]);
        assert_eq!(table.selected_rows, vec![1, 3]);
        assert_eq!(table.size, Size::Small);
        assert_eq!(table.sticky_body_height, Some(280.0));
        assert_eq!(table.min_column_width, 120.0);
    }

    #[test]
    fn table_cell_padding_adapts_for_narrow_widget_columns() {
        let theme = CastTheme::light();
        let narrow = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(32.0, 32.0));
        let wide = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(120.0, 32.0));

        assert_eq!(
            table_cell_content_rect(&theme, narrow).min.x,
            theme.spacing.xs
        );
        assert_eq!(
            table_cell_content_rect(&theme, wide).min.x,
            table_cell_padding(&theme)
        );
    }
}
