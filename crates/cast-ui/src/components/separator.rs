use egui::{Response, Sense, Ui, Widget};

use crate::{foundation::Orientation, theme::theme_for_ui};

#[derive(Clone, Debug)]
pub struct Separator {
    orientation: Orientation,
    spacing: Option<f32>,
}

impl Default for Separator {
    fn default() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            spacing: None,
        }
    }
}

impl Separator {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn horizontal() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            spacing: None,
        }
    }

    #[must_use]
    pub fn vertical() -> Self {
        Self {
            orientation: Orientation::Vertical,
            spacing: None,
        }
    }

    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing);
        self
    }
}

impl Widget for Separator {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let spacing = self.spacing.unwrap_or(theme.spacing.md);
        let available = ui.available_size_before_wrap();
        let size = match self.orientation {
            Orientation::Horizontal => egui::vec2(available.x, spacing),
            Orientation::Vertical => {
                egui::vec2(spacing, available.y.max(theme.controls.min_height))
            }
        };
        let (rect, response) = ui.allocate_exact_size(size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let stroke = egui::Stroke::new(theme.stroke.sm, theme.colors.border);
            match self.orientation {
                Orientation::Horizontal => {
                    ui.painter()
                        .hline(rect.left()..=rect.right(), rect.center().y, stroke);
                }
                Orientation::Vertical => {
                    ui.painter()
                        .vline(rect.center().x, rect.top()..=rect.bottom(), stroke);
                }
            }
        }

        response
    }
}
