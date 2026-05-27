use egui::{Response, RichText, Ui, Widget};

use crate::{foundation::Size, style::resolve_control_metrics, theme::theme_for_ui};

#[derive(Debug)]
pub struct Checkbox<'a> {
    checked: &'a mut bool,
    label: String,
    size: Size,
    indeterminate: bool,
}

impl<'a> Checkbox<'a> {
    #[must_use]
    pub fn new(checked: &'a mut bool, label: impl Into<String>) -> Self {
        Self {
            checked,
            label: label.into(),
            size: Size::Medium,
            indeterminate: false,
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
}

impl Widget for Checkbox<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let metrics = resolve_control_metrics(&theme, self.size);
        let label = RichText::new(self.label)
            .color(theme.colors.text)
            .family(theme.typography.body.family.clone())
            .size(metrics.text_size);

        ui.add(egui::Checkbox::new(self.checked, label).indeterminate(self.indeterminate))
    }
}
