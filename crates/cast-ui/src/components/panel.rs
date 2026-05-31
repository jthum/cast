use egui::{InnerResponse, Ui};

use crate::{
    components::card::{SurfaceSectionStyle, show_surface_sections_inside},
    style::{panel_frame, panel_shell_frame},
    theme::theme_for_ui,
};

#[derive(Clone, Debug, Default)]
pub struct Panel {
    sections: SurfaceSectionStyle,
}

impl Panel {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn section_style(mut self, sections: SurfaceSectionStyle) -> Self {
        self.sections = sections;
        self
    }

    #[must_use]
    pub fn muted_sections(mut self) -> Self {
        self.sections = SurfaceSectionStyle::muted();
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);

        panel_frame(&theme).show(ui, add_contents)
    }

    pub fn show_sections<R>(
        self,
        ui: &mut Ui,
        add_header: impl FnOnce(&mut Ui),
        add_contents: impl FnOnce(&mut Ui) -> R,
        add_footer: impl FnOnce(&mut Ui),
    ) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);

        panel_shell_frame(&theme).show(ui, |ui| {
            show_surface_sections_inside(
                ui,
                &theme,
                self.sections,
                theme.components.panel.padding,
                add_header,
                add_contents,
                add_footer,
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_can_opt_into_muted_sections() {
        let panel = Panel::new().muted_sections();

        assert_eq!(panel.sections, SurfaceSectionStyle::muted());
    }
}
