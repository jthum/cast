use egui::{Response, RichText, Ui, Widget};

use crate::{foundation::Size, style::resolve_control_metrics, theme::theme_for_ui};

#[derive(Clone, Debug)]
pub struct Link {
    label: String,
    url: Option<String>,
    size: Size,
}

impl Link {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            url: None,
            size: Size::Medium,
        }
    }

    #[must_use]
    pub fn to(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Widget for Link {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let metrics = resolve_control_metrics(&theme, self.size);
        let text = RichText::new(self.label)
            .color(theme.colors.link)
            .size(metrics.text_size);

        if let Some(url) = self.url {
            ui.hyperlink_to(text, url)
        } else {
            ui.add(egui::Link::new(text))
        }
    }
}
