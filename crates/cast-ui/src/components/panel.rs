use egui::{InnerResponse, Ui};

use crate::{style::panel_frame, theme::theme_for_ui};

#[derive(Clone, Debug, Default)]
pub struct Panel;

impl Panel {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);

        panel_frame(&theme).show(ui, add_contents)
    }
}
