use egui::{InnerResponse, Ui};

use crate::{
    foundation::Size,
    style::{pop_contextual_control_size, push_contextual_control_size},
    theme::theme_for_ui,
};

#[derive(Clone, Copy, Debug)]
pub struct FilterBar {
    size: Size,
}

impl Default for FilterBar {
    fn default() -> Self {
        Self::new()
    }
}

impl FilterBar {
    #[must_use]
    pub fn new() -> Self {
        Self { size: Size::Small }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);
        let previous = push_contextual_control_size(ui, self.size);
        let response = ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(theme.spacing.xs, theme.spacing.xs);
            add_contents(ui)
        });
        pop_contextual_control_size(ui, previous);

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_bar_defaults_to_small_controls() {
        let bar = FilterBar::new();

        assert_eq!(bar.size, Size::Small);
    }

    #[test]
    fn filter_bar_size_can_be_overridden() {
        let bar = FilterBar::new().size(Size::Medium);

        assert_eq!(bar.size, Size::Medium);
    }
}
