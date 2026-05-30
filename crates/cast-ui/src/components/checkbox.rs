use egui::{
    Color32, Response, Sense, StrokeKind, Ui, Widget,
    text::{LayoutJob, TextFormat},
};

use crate::{
    color::{mix_with_transparent, with_alpha},
    foundation::{Orientation, Size},
    style::resolve_control_metrics,
    theme::{CastTheme, theme_for_ui},
};

#[derive(Debug)]
pub struct Checkbox<'a> {
    checked: &'a mut bool,
    label: String,
    size: Size,
    indeterminate: bool,
    enabled: bool,
}

impl<'a> Checkbox<'a> {
    #[must_use]
    pub fn new(checked: &'a mut bool, label: impl Into<String>) -> Self {
        Self {
            checked,
            label: label.into(),
            size: Size::Medium,
            indeterminate: false,
            enabled: true,
        }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
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

impl Widget for Checkbox<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let selected = *self.checked || self.indeterminate;
        let mut response = choice_row(
            ui,
            &self.label,
            self.size,
            self.enabled,
            selected,
            ChoiceKind::Checkbox {
                indeterminate: self.indeterminate,
            },
        );

        if self.enabled && response.clicked() {
            *self.checked = if self.indeterminate {
                true
            } else {
                !*self.checked
            };
            response.mark_changed();
        }

        response
    }
}

#[derive(Debug)]
pub struct Radio<'a, T> {
    selected: &'a mut T,
    value: T,
    label: String,
    size: Size,
    enabled: bool,
}

impl<'a, T> Radio<'a, T>
where
    T: PartialEq + Clone,
{
    #[must_use]
    pub fn new(selected: &'a mut T, value: T, label: impl Into<String>) -> Self {
        Self {
            selected,
            value,
            label: label.into(),
            size: Size::Medium,
            enabled: true,
        }
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

impl<T> Widget for Radio<'_, T>
where
    T: PartialEq + Clone,
{
    fn ui(self, ui: &mut Ui) -> Response {
        let selected = *self.selected == self.value;
        let mut response = choice_row(
            ui,
            &self.label,
            self.size,
            self.enabled,
            selected,
            ChoiceKind::Radio,
        );

        if self.enabled && response.clicked() && !selected {
            *self.selected = self.value.clone();
            response.mark_changed();
        }

        response
    }
}

#[derive(Debug)]
pub struct RadioGroup<'a, T> {
    selected: &'a mut T,
    options: Vec<(T, String)>,
    size: Size,
    orientation: Orientation,
    enabled: bool,
}

impl<'a, T> RadioGroup<'a, T>
where
    T: PartialEq + Clone,
{
    #[must_use]
    pub fn new<I, L>(selected: &'a mut T, options: I) -> Self
    where
        I: IntoIterator<Item = (T, L)>,
        L: Into<String>,
    {
        Self {
            selected,
            options: options
                .into_iter()
                .map(|(value, label)| (value, label.into()))
                .collect(),
            size: Size::Medium,
            orientation: Orientation::Horizontal,
            enabled: true,
        }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    #[must_use]
    pub fn vertical(mut self) -> Self {
        self.orientation = Orientation::Vertical;
        self
    }

    #[must_use]
    pub fn horizontal(mut self) -> Self {
        self.orientation = Orientation::Horizontal;
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

impl<T> Widget for RadioGroup<'_, T>
where
    T: PartialEq + Clone,
{
    fn ui(self, ui: &mut Ui) -> Response {
        let RadioGroup {
            selected,
            options,
            size,
            orientation,
            enabled,
        } = self;

        let inner = if orientation == Orientation::Horizontal {
            ui.horizontal_wrapped(|ui| radio_group_content(ui, selected, options, size, enabled))
        } else {
            ui.vertical(|ui| radio_group_content(ui, selected, options, size, enabled))
        };
        let mut response = inner.response;

        if inner.inner {
            response.mark_changed();
        }

        response
    }
}

fn radio_group_content<T>(
    ui: &mut Ui,
    selected: &mut T,
    options: Vec<(T, String)>,
    size: Size,
    enabled: bool,
) -> bool
where
    T: PartialEq + Clone,
{
    let mut changed = false;

    for (value, label) in options {
        let response = ui.add(
            Radio::new(selected, value, label)
                .size(size)
                .enabled(enabled),
        );
        changed |= response.changed();
    }

    changed
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ChoiceKind {
    Checkbox { indeterminate: bool },
    Radio,
}

fn choice_row(
    ui: &mut Ui,
    label: &str,
    size: Size,
    enabled: bool,
    selected: bool,
    kind: ChoiceKind,
) -> Response {
    let theme = theme_for_ui(ui);
    let metrics = resolve_control_metrics(&theme, size);
    let mark_size = choice_mark_size(size);
    let has_label = !label.is_empty();
    let gap = choice_label_gap(&theme, label);
    let mut font_id = theme.typography.body.clone();
    font_id.size = metrics.text_size;
    let galley = ui.painter().layout_job(choice_layout_job(
        label.to_owned(),
        font_id,
        theme.typography.letter_spacing,
    ));
    let desired_size = egui::vec2(
        mark_size + gap + galley.size().x,
        galley.size().y.max(metrics.min_height - 6.0).max(mark_size),
    );
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(desired_size, sense);

    if ui.is_rect_visible(rect) {
        let mark_rect = egui::Rect::from_center_size(
            egui::pos2(rect.min.x + mark_size / 2.0, rect.center().y),
            egui::vec2(mark_size, mark_size),
        );
        paint_choice_mark(ui, &theme, mark_rect, &response, enabled, selected, kind);

        if has_label {
            let label_color = if enabled {
                theme.colors.text
            } else {
                theme.colors.text_subtle
            };
            ui.painter().galley(
                egui::pos2(
                    mark_rect.max.x + gap,
                    rect.center().y - galley.size().y / 2.0,
                ),
                galley,
                label_color,
            );
        }
    }

    response
}

fn paint_choice_mark(
    ui: &Ui,
    theme: &CastTheme,
    rect: egui::Rect,
    response: &Response,
    enabled: bool,
    selected: bool,
    kind: ChoiceKind,
) {
    let hovered = enabled && response.hovered();
    let pressed = enabled && response.is_pointer_button_down_on();
    let primary = theme.colors.primary_family.base;
    let radius = match kind {
        ChoiceKind::Checkbox { .. } => egui::CornerRadius::same((theme.radius.sm + 1.0) as u8),
        ChoiceKind::Radio => egui::CornerRadius::same((rect.width() / 2.0) as u8),
    };
    let fill = choice_fill(theme, selected, hovered, pressed, enabled, kind);
    let border = choice_border(theme, selected, hovered, pressed, enabled, kind);
    let border_width = choice_border_width(theme);

    if hovered || pressed {
        ui.painter().rect_filled(
            rect.expand(if pressed { 5.0 } else { 4.0 }),
            radius,
            mix_with_transparent(primary, if pressed { 0.14 } else { 0.09 }),
        );
    }

    match kind {
        ChoiceKind::Checkbox { .. } => {
            ui.painter().rect(
                rect,
                radius,
                fill,
                egui::Stroke::new(border_width, border),
                StrokeKind::Outside,
            );
        }
        ChoiceKind::Radio => {
            ui.painter()
                .circle_filled(rect.center(), rect.width() / 2.0, fill);
            ui.painter().circle_stroke(
                rect.center(),
                rect.width() / 2.0,
                egui::Stroke::new(border_width, border),
            );
        }
    }

    if !selected {
        return;
    }

    let mark_color = match kind {
        ChoiceKind::Checkbox { .. } if enabled => theme.colors.primary_family.fg,
        ChoiceKind::Checkbox { .. } => with_alpha(theme.colors.primary_family.fg, 180),
        ChoiceKind::Radio if enabled => theme.colors.primary_family.base,
        ChoiceKind::Radio => mix_with_transparent(theme.colors.primary_family.base, 0.55),
    };

    match kind {
        ChoiceKind::Checkbox { indeterminate } if indeterminate => {
            let y = rect.center().y;
            ui.painter().line_segment(
                [
                    egui::pos2(rect.min.x + rect.width() * 0.28, y),
                    egui::pos2(rect.max.x - rect.width() * 0.28, y),
                ],
                egui::Stroke::new(2.0, mark_color),
            );
        }
        ChoiceKind::Checkbox { .. } => {
            ui.painter().line_segment(
                [
                    egui::pos2(rect.min.x + rect.width() * 0.25, rect.center().y),
                    egui::pos2(
                        rect.min.x + rect.width() * 0.43,
                        rect.max.y - rect.height() * 0.30,
                    ),
                ],
                egui::Stroke::new(2.0, mark_color),
            );
            ui.painter().line_segment(
                [
                    egui::pos2(
                        rect.min.x + rect.width() * 0.43,
                        rect.max.y - rect.height() * 0.30,
                    ),
                    egui::pos2(
                        rect.max.x - rect.width() * 0.22,
                        rect.min.y + rect.height() * 0.30,
                    ),
                ],
                egui::Stroke::new(2.0, mark_color),
            );
        }
        ChoiceKind::Radio => {
            ui.painter()
                .circle_filled(rect.center(), rect.width() * 0.26, mark_color);
        }
    }
}

fn choice_fill(
    theme: &CastTheme,
    selected: bool,
    hovered: bool,
    pressed: bool,
    enabled: bool,
    kind: ChoiceKind,
) -> Color32 {
    if selected {
        match kind {
            ChoiceKind::Checkbox { .. } if enabled => theme.colors.primary_family.base,
            ChoiceKind::Checkbox { .. } => {
                mix_with_transparent(theme.colors.primary_family.base, 0.45)
            }
            ChoiceKind::Radio => theme.colors.surface,
        }
    } else if pressed {
        theme.colors.surface_raised
    } else if hovered {
        theme.colors.surface_muted
    } else {
        theme.colors.surface
    }
}

fn choice_border(
    theme: &CastTheme,
    selected: bool,
    hovered: bool,
    pressed: bool,
    enabled: bool,
    _kind: ChoiceKind,
) -> Color32 {
    if selected {
        if enabled {
            theme.colors.primary_family.base
        } else {
            mix_with_transparent(theme.colors.primary_family.base, 0.35)
        }
    } else if pressed {
        theme.colors.border_strong
    } else if hovered {
        mix_with_transparent(theme.colors.primary_family.base, 0.35)
    } else {
        theme.colors.border
    }
}

fn choice_border_width(theme: &CastTheme) -> f32 {
    theme.stroke.sm.max(1.5)
}

fn choice_mark_size(size: Size) -> f32 {
    match size {
        Size::Small => 14.0,
        Size::Medium => 16.0,
        Size::Large => 18.0,
    }
}

fn choice_label_gap(theme: &CastTheme, label: &str) -> f32 {
    if label.is_empty() {
        0.0
    } else {
        theme.spacing.sm
    }
}

fn choice_layout_job(text: String, font_id: egui::FontId, letter_spacing: f32) -> LayoutJob {
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
    fn choice_mark_sizes_scale() {
        assert!(choice_mark_size(Size::Small) < choice_mark_size(Size::Medium));
        assert!(choice_mark_size(Size::Medium) < choice_mark_size(Size::Large));
    }

    #[test]
    fn empty_choice_labels_do_not_reserve_label_gap() {
        let theme = CastTheme::light();

        assert_eq!(choice_label_gap(&theme, ""), 0.0);
        assert_eq!(choice_label_gap(&theme, "Enabled"), theme.spacing.sm);
    }

    #[test]
    fn selected_choices_use_primary_colors() {
        let theme = CastTheme::light();

        assert_eq!(
            choice_fill(
                &theme,
                true,
                false,
                false,
                true,
                ChoiceKind::Checkbox {
                    indeterminate: false
                }
            ),
            theme.colors.primary_family.base
        );
        assert_eq!(
            choice_border(
                &theme,
                true,
                false,
                false,
                true,
                ChoiceKind::Checkbox {
                    indeterminate: false
                }
            ),
            theme.colors.primary_family.base
        );
    }

    #[test]
    fn selected_radio_uses_surface_fill_with_primary_ring() {
        let theme = CastTheme::light();

        assert_eq!(
            choice_fill(&theme, true, false, false, true, ChoiceKind::Radio),
            theme.colors.surface
        );
        assert_eq!(
            choice_border(&theme, true, false, false, true, ChoiceKind::Radio),
            theme.colors.primary_family.base
        );
    }

    #[test]
    fn choice_border_width_has_a_crisp_floor() {
        let mut theme = CastTheme::dark();
        theme.stroke.sm = 1.0;

        assert_eq!(choice_border_width(&theme), 1.5);
    }

    #[test]
    fn radio_group_collects_options_and_defaults_horizontal() {
        let mut selected = 0;
        let group = RadioGroup::new(&mut selected, [(0, "Compact"), (1, "Comfortable")]);

        assert_eq!(group.options.len(), 2);
        assert_eq!(group.size, Size::Medium);
        assert_eq!(group.orientation, Orientation::Horizontal);
        assert!(group.enabled);
    }

    #[test]
    fn radio_group_can_be_vertical_and_disabled() {
        let mut selected = 0;
        let group = RadioGroup::new(&mut selected, [(0, "A"), (1, "B")])
            .vertical()
            .disabled()
            .size(Size::Small);

        assert_eq!(group.orientation, Orientation::Vertical);
        assert_eq!(group.size, Size::Small);
        assert!(!group.enabled);
    }
}
