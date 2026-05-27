use egui::{Response, TextEdit, Ui, Widget};

use crate::{
    foundation::Size,
    style::{input_frame, resolve_control_metrics},
    theme::theme_for_ui,
};

#[derive(Debug)]
pub struct TextInput<'a> {
    text: &'a mut String,
    hint_text: Option<String>,
    width: Option<f32>,
    size: Size,
}

impl<'a> TextInput<'a> {
    #[must_use]
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            hint_text: None,
            width: None,
            size: Size::Medium,
        }
    }

    #[must_use]
    pub fn hint_text(mut self, hint_text: impl Into<String>) -> Self {
        self.hint_text = Some(hint_text.into());
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Widget for TextInput<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let metrics = resolve_control_metrics(&theme, self.size);
        let mut edit = TextEdit::singleline(self.text)
            .frame(input_frame(&theme))
            .min_size(egui::vec2(0.0, metrics.min_height))
            .text_color(theme.colors.text);

        if let Some(hint_text) = self.hint_text {
            edit = edit.hint_text(hint_text);
        }

        if let Some(width) = self.width {
            edit = edit.desired_width(width);
        }

        ui.add(edit)
    }
}

#[derive(Debug)]
pub struct SearchInput<'a> {
    inner: TextInput<'a>,
}

impl<'a> SearchInput<'a> {
    #[must_use]
    pub fn new(text: &'a mut String) -> Self {
        Self {
            inner: TextInput::new(text).hint_text("Search"),
        }
    }

    #[must_use]
    pub fn hint_text(mut self, hint_text: impl Into<String>) -> Self {
        self.inner = self.inner.hint_text(hint_text);
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.inner = self.inner.width(width);
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.inner = self.inner.size(size);
        self
    }
}

impl Widget for SearchInput<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.inner.ui(ui)
    }
}
