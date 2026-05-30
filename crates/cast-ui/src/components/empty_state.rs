use egui::{InnerResponse, RichText, Ui};

use crate::{
    foundation::Intent,
    theme::{CastTheme, theme_for_ui},
};

#[derive(Clone, Debug)]
pub struct EmptyState {
    title: String,
    body: Option<String>,
    icon: Option<String>,
    intent: Intent,
}

impl EmptyState {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: None,
            icon: None,
            intent: Intent::Neutral,
        }
    }

    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    #[must_use]
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    #[must_use]
    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_actions: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);
        ui.vertical_centered(|ui| {
            ui.spacing_mut().item_spacing.y = theme.spacing.sm;
            ui.add_space(theme.spacing.md);
            if let Some(icon) = self.icon {
                paint_empty_state_icon(ui, &theme, self.intent, icon);
            }
            ui.label(
                RichText::new(self.title)
                    .family(theme.typography.heading_sm.family.clone())
                    .size(theme.typography.heading_sm.size)
                    .color(theme.colors.text)
                    .extra_letter_spacing(theme.typography.letter_spacing),
            );
            if let Some(body) = self.body {
                ui.label(
                    RichText::new(body)
                        .family(theme.typography.small.family.clone())
                        .size(theme.typography.small.size)
                        .color(theme.colors.text_muted)
                        .extra_letter_spacing(theme.typography.letter_spacing),
                );
            }
            ui.add_space(theme.spacing.xs);
            let inner = add_actions(ui);
            ui.add_space(theme.spacing.md);
            inner
        })
    }
}

fn paint_empty_state_icon(ui: &mut Ui, theme: &CastTheme, intent: Intent, icon: String) {
    let side = 42.0;
    let (rect, _) = ui.allocate_exact_size(egui::Vec2::splat(side), egui::Sense::hover());
    let family = match intent {
        Intent::Neutral => (theme.colors.text_muted, theme.colors.text_muted),
        Intent::Primary => (
            theme.colors.primary_family.base,
            theme.colors.primary_family.emphasis,
        ),
        Intent::Secondary => (
            theme.colors.secondary_family.base,
            theme.colors.secondary_family.emphasis,
        ),
        Intent::Success => (
            theme.colors.success_family.base,
            theme.colors.success_family.emphasis,
        ),
        Intent::Warning => (
            theme.colors.warning_family.base,
            theme.colors.warning_family.emphasis,
        ),
        Intent::Danger => (
            theme.colors.danger_family.base,
            theme.colors.danger_family.emphasis,
        ),
        Intent::Info => (
            theme.colors.info_family.base,
            theme.colors.info_family.emphasis,
        ),
    };

    ui.painter().circle_filled(
        rect.center(),
        side / 2.0,
        crate::mix_with_transparent(family.0, 0.08),
    );
    ui.painter().circle_stroke(
        rect.center(),
        side / 2.0,
        egui::Stroke::new(theme.stroke.sm, crate::mix_with_transparent(family.0, 0.24)),
    );
    let galley = ui
        .painter()
        .layout_no_wrap(icon, theme.typography.button.clone(), family.1);
    ui.painter()
        .galley(rect.center() - galley.size() / 2.0, galley, family.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_state_defaults_to_neutral_without_body_or_icon() {
        let state = EmptyState::new("No results");

        assert_eq!(state.intent, Intent::Neutral);
        assert!(state.body.is_none());
        assert!(state.icon.is_none());
    }
}
