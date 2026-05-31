use egui::{Color32, CornerRadius, InnerResponse, Margin, Ui};

use crate::{
    style::{card_frame, card_shell_frame},
    theme::{CastTheme, theme_for_ui},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceChrome {
    Flat,
    Muted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SurfaceSectionStyle {
    pub header: SurfaceChrome,
    pub footer: SurfaceChrome,
    pub dividers: bool,
}

impl SurfaceSectionStyle {
    #[must_use]
    pub fn flat() -> Self {
        Self {
            header: SurfaceChrome::Flat,
            footer: SurfaceChrome::Flat,
            dividers: false,
        }
    }

    #[must_use]
    pub fn muted() -> Self {
        Self {
            header: SurfaceChrome::Muted,
            footer: SurfaceChrome::Muted,
            dividers: true,
        }
    }
}

impl Default for SurfaceSectionStyle {
    fn default() -> Self {
        Self::flat()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Card {
    sections: SurfaceSectionStyle,
}

impl Card {
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

        card_frame(&theme).show(ui, add_contents)
    }

    pub fn show_with_header<R>(
        self,
        ui: &mut Ui,
        add_header: impl FnOnce(&mut Ui),
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);

        card_shell_frame(&theme).show(ui, |ui| {
            show_surface_sections_optional(
                ui,
                &theme,
                self.sections,
                theme.components.section.padding,
                Some(add_header),
                add_contents,
                None::<fn(&mut Ui)>,
            )
        })
    }

    pub fn show_with_footer<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
        add_footer: impl FnOnce(&mut Ui),
    ) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);

        card_shell_frame(&theme).show(ui, |ui| {
            show_surface_sections_optional(
                ui,
                &theme,
                self.sections,
                theme.components.section.padding,
                None::<fn(&mut Ui)>,
                add_contents,
                Some(add_footer),
            )
        })
    }

    pub fn show_sections<R>(
        self,
        ui: &mut Ui,
        add_header: impl FnOnce(&mut Ui),
        add_contents: impl FnOnce(&mut Ui) -> R,
        add_footer: impl FnOnce(&mut Ui),
    ) -> InnerResponse<R> {
        let theme = theme_for_ui(ui);

        card_shell_frame(&theme).show(ui, |ui| {
            show_surface_sections_inside(
                ui,
                &theme,
                self.sections,
                theme.components.section.padding,
                add_header,
                add_contents,
                add_footer,
            )
        })
    }
}

pub(crate) fn show_surface_sections_inside<R>(
    ui: &mut Ui,
    theme: &CastTheme,
    sections: SurfaceSectionStyle,
    padding: f32,
    add_header: impl FnOnce(&mut Ui),
    add_contents: impl FnOnce(&mut Ui) -> R,
    add_footer: impl FnOnce(&mut Ui),
) -> R {
    show_surface_sections_optional(
        ui,
        theme,
        sections,
        padding,
        Some(add_header),
        add_contents,
        Some(add_footer),
    )
}

pub(crate) fn show_surface_sections_optional<R, H, B, F>(
    ui: &mut Ui,
    theme: &CastTheme,
    sections: SurfaceSectionStyle,
    padding: f32,
    add_header: Option<H>,
    add_contents: B,
    add_footer: Option<F>,
) -> R
where
    H: FnOnce(&mut Ui),
    B: FnOnce(&mut Ui) -> R,
    F: FnOnce(&mut Ui),
{
    show_surface_sections_optional_with_radius(
        ui,
        theme,
        sections,
        padding,
        theme.components.card.radius,
        add_header,
        add_contents,
        add_footer,
    )
}

pub(crate) fn show_surface_sections_inside_with_radius<R>(
    ui: &mut Ui,
    theme: &CastTheme,
    sections: SurfaceSectionStyle,
    padding: f32,
    radius: f32,
    add_header: impl FnOnce(&mut Ui),
    add_contents: impl FnOnce(&mut Ui) -> R,
    add_footer: impl FnOnce(&mut Ui),
) -> R {
    show_surface_sections_optional_with_radius(
        ui,
        theme,
        sections,
        padding,
        radius,
        Some(add_header),
        add_contents,
        Some(add_footer),
    )
}

pub(crate) fn show_surface_sections_optional_with_radius<R, H, B, F>(
    ui: &mut Ui,
    theme: &CastTheme,
    sections: SurfaceSectionStyle,
    padding: f32,
    radius: f32,
    add_header: Option<H>,
    add_contents: B,
    add_footer: Option<F>,
) -> R
where
    H: FnOnce(&mut Ui),
    B: FnOnce(&mut Ui) -> R,
    F: FnOnce(&mut Ui),
{
    let previous_spacing = ui.spacing().item_spacing;
    ui.spacing_mut().item_spacing.y = 0.0;

    let has_header = add_header.is_some();
    let has_footer = add_footer.is_some();

    if let Some(add_header) = add_header {
        let header = show_surface_section_with_radius(
            ui,
            theme,
            sections.header,
            padding,
            surface_section_radius(radius, true, !has_footer),
            add_header,
        );
        if sections.dividers {
            paint_section_divider(ui, theme, header.response.rect, header.response.rect.max.y);
        }
    }

    let body = show_surface_section_with_radius(
        ui,
        theme,
        SurfaceChrome::Flat,
        padding,
        surface_section_radius(radius, !has_header, !has_footer),
        add_contents,
    )
    .inner;

    if let Some(add_footer) = add_footer {
        let footer = show_surface_section_with_radius(
            ui,
            theme,
            sections.footer,
            padding,
            surface_section_radius(radius, !has_header, true),
            add_footer,
        );
        if sections.dividers {
            paint_section_divider(ui, theme, footer.response.rect, footer.response.rect.min.y);
        }
    }

    ui.spacing_mut().item_spacing = previous_spacing;
    body
}

pub(crate) fn show_surface_section<R>(
    ui: &mut Ui,
    theme: &CastTheme,
    chrome: SurfaceChrome,
    padding: f32,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    show_surface_section_with_radius(ui, theme, chrome, padding, CornerRadius::ZERO, add_contents)
}

pub(crate) fn show_surface_section_with_radius<R>(
    ui: &mut Ui,
    theme: &CastTheme,
    chrome: SurfaceChrome,
    padding: f32,
    corner_radius: CornerRadius,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    egui::Frame::new()
        .fill(surface_chrome_fill(theme, chrome))
        .corner_radius(corner_radius)
        .inner_margin(Margin::same(padding as i8))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            add_contents(ui)
        })
}

fn surface_chrome_fill(theme: &CastTheme, chrome: SurfaceChrome) -> Color32 {
    match chrome {
        SurfaceChrome::Flat => Color32::TRANSPARENT,
        SurfaceChrome::Muted => theme.components.section.muted_fill,
    }
}

pub(crate) fn surface_section_radius(radius: f32, top: bool, bottom: bool) -> CornerRadius {
    let radius = radius.round() as u8;
    CornerRadius {
        nw: if top { radius } else { 0 },
        ne: if top { radius } else { 0 },
        sw: if bottom { radius } else { 0 },
        se: if bottom { radius } else { 0 },
    }
}

pub(crate) fn paint_section_divider(ui: &Ui, theme: &CastTheme, rect: egui::Rect, y: f32) {
    ui.painter().line_segment(
        [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
        egui::Stroke::new(
            theme.components.section.divider_width,
            theme.components.section.divider,
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_defaults_to_flat_sections() {
        let card = Card::new();

        assert_eq!(card.sections, SurfaceSectionStyle::flat());
    }

    #[test]
    fn muted_section_style_uses_chrome_and_dividers() {
        let style = SurfaceSectionStyle::muted();

        assert_eq!(style.header, SurfaceChrome::Muted);
        assert_eq!(style.footer, SurfaceChrome::Muted);
        assert!(style.dividers);
    }
}
