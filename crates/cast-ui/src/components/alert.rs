use egui::{Response, RichText, Ui, Widget};

use crate::{
    foundation::{Intent, Variant},
    style::{alert_frame, resolve_component_style, resolve_intent_colors},
    theme::theme_for_ui,
};

#[derive(Clone, Debug)]
pub struct Alert {
    title: String,
    body: Option<String>,
    intent: Intent,
}

impl Alert {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: None,
            intent: Intent::Info,
        }
    }

    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }
}

impl Widget for Alert {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        let subtle =
            resolve_component_style(&theme, self.intent, Variant::Subtle, Default::default());
        let solid = resolve_intent_colors(&theme, self.intent, Variant::Solid);

        alert_frame(&theme, solid.border)
            .fill(subtle.colors.fill)
            .show(ui, |ui| {
                ui.label(
                    RichText::new(self.title)
                        .color(subtle.colors.fg)
                        .family(theme.typography.strong.family.clone())
                        .size(theme.typography.body.size),
                );
                if let Some(body) = self.body {
                    ui.label(
                        RichText::new(body)
                            .color(theme.colors.text_muted)
                            .family(theme.typography.small.family.clone())
                            .size(theme.typography.small.size),
                    );
                }
            })
            .response
    }
}

#[derive(Clone, Debug)]
pub struct Notice {
    inner: Alert,
}

impl Notice {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            inner: Alert::new(title).intent(Intent::Neutral),
        }
    }

    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.inner = self.inner.body(body);
        self
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.inner = self.inner.intent(intent);
        self
    }
}

impl Widget for Notice {
    fn ui(self, ui: &mut Ui) -> Response {
        self.inner.ui(ui)
    }
}
