use egui::{InnerResponse, Ui};

use crate::theme::theme_for_ui;

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
}
