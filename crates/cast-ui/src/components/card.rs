use egui::{InnerResponse, Ui};

use crate::{style::card_frame, theme::theme_for_ui};

#[derive(Clone, Debug, Default)]
pub struct Card;

impl Card {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);

        card_frame(&theme).show(ui, add_contents)
    }
}
