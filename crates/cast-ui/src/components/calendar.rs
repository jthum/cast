use egui::{Color32, Response, Sense, StrokeKind, Ui, Widget};

use crate::{
    foundation::Size,
    style::resolve_control_metrics,
    theme::{CastTheme, theme_for_ui},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalendarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl CalendarDate {
    #[must_use]
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        let month = month.clamp(1, 12);
        let day = day.clamp(1, days_in_month(year, month));

        Self { year, month, day }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalendarMonth {
    pub year: i32,
    pub month: u8,
}

impl CalendarMonth {
    #[must_use]
    pub fn new(year: i32, month: u8) -> Self {
        Self {
            year,
            month: month.clamp(1, 12),
        }
    }

    #[must_use]
    pub fn from_date(date: CalendarDate) -> Self {
        Self::new(date.year, date.month)
    }

    pub fn shift(&mut self, delta: i32) {
        let zero_based = self.year * 12 + i32::from(self.month) - 1 + delta;
        self.year = zero_based.div_euclid(12);
        self.month = (zero_based.rem_euclid(12) + 1) as u8;
    }

    #[must_use]
    pub fn label(self) -> String {
        format!("{} {}", month_name(self.month), self.year)
    }
}

#[derive(Debug)]
pub struct Calendar<'a> {
    selected: &'a mut CalendarDate,
    visible_month: &'a mut CalendarMonth,
    size: Size,
    width: Option<f32>,
}

impl<'a> Calendar<'a> {
    #[must_use]
    pub fn new(selected: &'a mut CalendarDate, visible_month: &'a mut CalendarMonth) -> Self {
        Self {
            selected,
            visible_month,
            size: Size::Medium,
            width: None,
        }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(220.0));
        self
    }
}

impl Widget for Calendar<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let metrics = resolve_control_metrics(&theme, self.size);
        let width = self.width.unwrap_or(292.0);
        let cell_size = calendar_cell_size(&theme, self.size);
        let header_height = metrics.min_height;
        let weekday_height = theme.typography.caption.size + theme.spacing.sm;
        let height = theme.spacing.sm
            + header_height
            + theme.spacing.sm
            + weekday_height
            + cell_size * 6.0
            + theme.spacing.sm;
        let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), Sense::hover());
        let mut combined = response;

        if ui.is_rect_visible(rect) {
            ui.painter().rect(
                rect,
                egui::CornerRadius::same(theme.radius.lg.round() as u8),
                theme.colors.surface,
                egui::Stroke::new(theme.stroke.sm, theme.colors.border),
                StrokeKind::Outside,
            );

            let content = rect.shrink(theme.spacing.sm);
            let header =
                egui::Rect::from_min_size(content.min, egui::vec2(content.width(), header_height));
            paint_calendar_header(ui, &theme, header, self.visible_month, &mut combined);

            let weekdays_top = header.max.y + theme.spacing.sm;
            paint_weekdays(ui, &theme, content, weekdays_top, weekday_height);

            let grid_top = weekdays_top + weekday_height;
            paint_calendar_days(
                ui,
                &theme,
                egui::Rect::from_min_max(
                    egui::pos2(content.min.x, grid_top),
                    egui::pos2(content.max.x, content.max.y),
                ),
                cell_size,
                self.selected,
                *self.visible_month,
                &mut combined,
            );
        }

        combined
    }
}

fn paint_calendar_header(
    ui: &mut Ui,
    theme: &CastTheme,
    rect: egui::Rect,
    month: &mut CalendarMonth,
    combined: &mut Response,
) {
    let button_side = rect.height().min(30.0);
    let previous_rect = egui::Rect::from_min_size(rect.min, egui::vec2(button_side, button_side));
    let next_rect = egui::Rect::from_min_size(
        egui::pos2(rect.max.x - button_side, rect.min.y),
        egui::vec2(button_side, button_side),
    );

    let previous = calendar_icon_button(ui, theme, previous_rect, "<", "previous_month");
    if previous.clicked() {
        month.shift(-1);
        combined.mark_changed();
    }
    *combined = combined.clone().union(previous);

    let next = calendar_icon_button(ui, theme, next_rect, ">", "next_month");
    if next.clicked() {
        month.shift(1);
        combined.mark_changed();
    }
    *combined = combined.clone().union(next);

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        month.label(),
        theme.typography.body_strong.clone(),
        theme.colors.text,
    );
}

fn calendar_icon_button(
    ui: &Ui,
    theme: &CastTheme,
    rect: egui::Rect,
    label: &'static str,
    id_salt: &'static str,
) -> Response {
    let response = ui.interact(rect, ui.id().with(id_salt), Sense::click());
    let hovered = response.hovered();
    let pressed = response.is_pointer_button_down_on();
    let fill = if pressed {
        theme.colors.surface_raised
    } else if hovered {
        theme.colors.surface_muted
    } else {
        Color32::TRANSPARENT
    };

    ui.painter().rect(
        rect,
        egui::CornerRadius::same(theme.radius.md.round() as u8),
        fill,
        egui::Stroke::new(theme.stroke.sm, Color32::TRANSPARENT),
        StrokeKind::Outside,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        theme.typography.button.clone(),
        theme.colors.text_muted,
    );

    response
}

fn paint_weekdays(ui: &Ui, theme: &CastTheme, content: egui::Rect, top: f32, weekday_height: f32) {
    let cell_width = content.width() / 7.0;
    for (index, weekday) in ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
        .iter()
        .enumerate()
    {
        let rect = egui::Rect::from_min_size(
            egui::pos2(content.min.x + index as f32 * cell_width, top),
            egui::vec2(cell_width, weekday_height),
        );
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            *weekday,
            theme.typography.caption.clone(),
            theme.colors.text_subtle,
        );
    }
}

fn paint_calendar_days(
    ui: &mut Ui,
    theme: &CastTheme,
    grid: egui::Rect,
    cell_size: f32,
    selected: &mut CalendarDate,
    month: CalendarMonth,
    combined: &mut Response,
) {
    let cell_width = grid.width() / 7.0;
    let first_weekday = weekday_monday_index(month.year, month.month, 1) as i32;
    let days = days_in_month(month.year, month.month) as i32;

    for row in 0..6 {
        for column in 0..7 {
            let day = row * 7 + column - first_weekday + 1;
            if !(1..=days).contains(&day) {
                continue;
            }

            let rect = egui::Rect::from_center_size(
                egui::pos2(
                    grid.min.x + column as f32 * cell_width + cell_width / 2.0,
                    grid.min.y + row as f32 * cell_size + cell_size / 2.0,
                ),
                egui::vec2(cell_width.min(cell_size), cell_size),
            );
            let date = CalendarDate::new(month.year, month.month, day as u8);
            let day_response = ui.interact(
                rect,
                ui.id()
                    .with(("cast_calendar_day", month.year, month.month, day)),
                Sense::click(),
            );
            let is_selected = *selected == date;
            let hovered = day_response.hovered();
            let pressed = day_response.is_pointer_button_down_on();
            let fill = calendar_day_fill(theme, is_selected, hovered, pressed);
            let fg = if is_selected {
                theme.colors.primary_family.emphasis
            } else if hovered {
                theme.colors.text
            } else {
                theme.colors.text_muted
            };

            ui.painter().rect(
                rect.shrink(2.0),
                egui::CornerRadius::same(theme.radius.md.round() as u8),
                fill,
                egui::Stroke::new(
                    theme.stroke.sm,
                    if is_selected {
                        theme.colors.primary_family.border
                    } else {
                        Color32::TRANSPARENT
                    },
                ),
                StrokeKind::Outside,
            );
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                day.to_string(),
                theme.typography.small.clone(),
                fg,
            );

            if day_response.clicked() && *selected != date {
                *selected = date;
                combined.mark_changed();
            }
            *combined = combined.clone().union(day_response);
        }
    }
}

fn calendar_day_fill(theme: &CastTheme, selected: bool, hovered: bool, pressed: bool) -> Color32 {
    if selected {
        theme.colors.primary_family.subtle
    } else if pressed {
        theme.colors.surface_raised
    } else if hovered {
        theme.colors.surface_muted
    } else {
        Color32::TRANSPARENT
    }
}

fn calendar_cell_size(theme: &CastTheme, size: Size) -> f32 {
    match size {
        Size::Small => theme.controls.min_height - 4.0,
        Size::Medium => theme.controls.min_height + 2.0,
        Size::Large => theme.controls.min_height + 8.0,
    }
    .max(28.0)
}

fn month_name(month: u8) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "January",
    }
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 31,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn weekday_monday_index(year: i32, month: u8, day: u8) -> u8 {
    let offsets = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut year = year;
    let month_index = month.clamp(1, 12) as usize;
    if month_index < 3 {
        year -= 1;
    }
    let sunday_index =
        (year + year / 4 - year / 100 + year / 400 + offsets[month_index - 1] + i32::from(day))
            .rem_euclid(7);

    ((sunday_index + 6) % 7) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calendar_date_clamps_invalid_parts() {
        assert_eq!(
            CalendarDate::new(2026, 13, 99),
            CalendarDate::new(2026, 12, 31)
        );
        assert_eq!(CalendarDate::new(2025, 2, 29).day, 28);
        assert_eq!(CalendarDate::new(2024, 2, 29).day, 29);
    }

    #[test]
    fn calendar_month_shift_crosses_year_boundaries() {
        let mut month = CalendarMonth::new(2026, 1);
        month.shift(-1);
        assert_eq!(month, CalendarMonth::new(2025, 12));
        month.shift(2);
        assert_eq!(month, CalendarMonth::new(2026, 2));
    }

    #[test]
    fn weekday_uses_monday_index() {
        assert_eq!(weekday_monday_index(2026, 6, 1), 0);
        assert_eq!(weekday_monday_index(2026, 5, 31), 6);
    }
}
