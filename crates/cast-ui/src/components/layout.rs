use egui::{InnerResponse, Ui};

use crate::theme::theme_for_ui;

#[derive(Clone, Copy, Debug)]
pub struct ControlGroup {
    gap: Option<f32>,
    padding: Option<f32>,
    width: Option<f32>,
    wrap: bool,
}

impl ControlGroup {
    #[must_use]
    pub fn new() -> Self {
        Self {
            gap: None,
            padding: None,
            width: None,
            wrap: true,
        }
    }

    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = Some(gap.max(0.0));
        self
    }

    #[must_use]
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding.max(0.0));
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(120.0));
        self
    }

    #[must_use]
    pub fn nowrap(mut self) -> Self {
        self.wrap = false;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);
        let gap = self.gap.unwrap_or(theme.spacing.xs);
        let padding = self.padding.unwrap_or(theme.spacing.xs);

        egui::Frame::new()
            .fill(theme.colors.surface)
            .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
            .corner_radius(egui::CornerRadius::same(theme.radius.md.round() as u8))
            .inner_margin(egui::Margin::same(padding as i8))
            .show(ui, |ui| {
                if let Some(width) = self.width {
                    ui.set_width(width);
                    ui.set_max_width(width);
                }
                let previous_spacing = ui.spacing().item_spacing;
                ui.spacing_mut().item_spacing = egui::vec2(gap, gap);
                let inner = if self.wrap {
                    ui.horizontal_wrapped(add_contents)
                } else {
                    ui.horizontal(add_contents)
                };
                ui.spacing_mut().item_spacing = previous_spacing;
                inner.inner
            })
    }
}

impl Default for ControlGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ResponsiveColumns {
    breakpoint: f32,
    min_column_width: f32,
    gap: Option<f32>,
}

impl ResponsiveColumns {
    #[must_use]
    pub fn new() -> Self {
        Self {
            breakpoint: 720.0,
            min_column_width: 260.0,
            gap: None,
        }
    }

    #[must_use]
    pub fn breakpoint(mut self, breakpoint: f32) -> Self {
        self.breakpoint = breakpoint.max(240.0);
        self
    }

    #[must_use]
    pub fn min_column_width(mut self, width: f32) -> Self {
        self.min_column_width = width.max(120.0);
        self
    }

    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = Some(gap.max(0.0));
        self
    }

    pub fn show<L, R>(
        self,
        ui: &mut Ui,
        left: L,
        right: R,
    ) -> InnerResponse<(InnerResponse<()>, InnerResponse<()>)>
    where
        L: FnOnce(&mut Ui),
        R: FnOnce(&mut Ui),
    {
        let theme = theme_for_ui(ui);
        let gap = self.gap.unwrap_or(theme.spacing.md);
        let available = ui.available_width();

        ui.vertical(|ui| {
            if available < self.breakpoint {
                let left_response = ui.vertical(left);
                ui.add_space(gap);
                let right_response = ui.vertical(right);
                (left_response, right_response)
            } else {
                let column_width = ((available - gap) / 2.0).max(self.min_column_width);
                let mut left_response = None;
                let mut right_response = None;

                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    left_response = Some(ui.vertical(|ui| {
                        ui.set_width(column_width);
                        ui.set_max_width(column_width);
                        left(ui);
                    }));
                    ui.add_space(gap);
                    right_response = Some(ui.vertical(|ui| {
                        ui.set_width(column_width);
                        ui.set_max_width(column_width);
                        right(ui);
                    }));
                });

                (
                    left_response.expect("left column is always rendered"),
                    right_response.expect("right column is always rendered"),
                )
            }
        })
    }
}

impl Default for ResponsiveColumns {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn responsive_columns_have_reasonable_defaults() {
        let columns = ResponsiveColumns::new()
            .breakpoint(120.0)
            .min_column_width(10.0);

        assert_eq!(columns.breakpoint, 240.0);
        assert_eq!(columns.min_column_width, 120.0);
    }

    #[test]
    fn control_group_defaults_to_wrapped_contents() {
        let group = ControlGroup::new().width(80.0).gap(2.0).padding(3.0);

        assert_eq!(group.width, Some(120.0));
        assert_eq!(group.gap, Some(2.0));
        assert_eq!(group.padding, Some(3.0));
        assert!(group.wrap);
    }
}
