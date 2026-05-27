use egui::{Frame, InnerResponse, Ui};

use crate::theme::{CastTheme, current_theme};

#[derive(Clone, Debug, Default)]
pub struct Card;

impl Card {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = current_theme(ui.ctx()).unwrap_or_else(CastTheme::light);

        Frame::new()
            .fill(theme.colors.surface)
            .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
            .inner_margin(egui::Margin::same(theme.spacing.lg as i8))
            .show(ui, add_contents)
    }
}
