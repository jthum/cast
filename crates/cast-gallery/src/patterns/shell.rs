use cast::{
    CastTheme, SegmentedControl, Size, Slider, ThemeMode,
    egui::{
        self, Color32, RichText, ScrollArea,
        scroll_area::{ScrollBarVisibility, ScrollSource},
    },
};

pub fn show_shell_top_bar(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    seed: &mut cast::ThemeSeed,
    zoom: &mut f32,
) -> bool {
    let mut changed = false;
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), 32.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.label("Cast Gallery");
            ui.separator();

            let mut mode_index = match seed.mode {
                ThemeMode::Light => 0,
                ThemeMode::Dark => 1,
            };
            let previous_mode_index = mode_index;
            ui.add(SegmentedControl::new(&mut mode_index, ["Light", "Dark"]).size(Size::Small));
            if mode_index != previous_mode_index {
                seed.mode = if mode_index == 0 {
                    ThemeMode::Light
                } else {
                    ThemeMode::Dark
                };
                changed = true;
            }

            ui.separator();
            ui.label("Zoom");
            if ui
                .add(Slider::new(zoom, 0.9..=1.35).show_value(false).width(118.0))
                .changed()
            {
                ctx.set_zoom_factor(*zoom);
            }
        },
    );
    changed
}

pub fn show_shell_sidebar(ui: &mut egui::Ui, theme: &CastTheme, selected: &mut usize) {
    ui.add_space(6.0);
    ui.label(
        RichText::new("Cast")
            .strong()
            .size(theme.typography.heading.size + 2.0)
            .color(Color32::WHITE),
    );
    ui.label(RichText::new("Themeable egui components").color(sidebar_muted_text()));
    ui.add_space(18.0);

    sidebar_workspace_switcher(ui, theme);
    ui.add_space(18.0);
    sidebar_group_label(ui, "Workspace");
    for (index, label) in ["Workbench", "Foundations", "Components", "Theme lab"]
        .iter()
        .enumerate()
    {
        if sidebar_nav_item(ui, theme, label, *selected == index).clicked() {
            *selected = index;
        }
    }

    ui.add_space(18.0);
    sidebar_group_label(ui, "Status");
    sidebar_status_row(ui, theme, "Runtime theme", "Live");
    sidebar_status_row(ui, theme, "Components", "Ready");
}

pub fn shell_sidebar_fill(theme: &CastTheme) -> Color32 {
    match theme.mode {
        ThemeMode::Light => shell_mix(theme.colors.primary_family.base, Color32::BLACK, 0.78),
        ThemeMode::Dark => shell_mix(theme.colors.surface, Color32::BLACK, 0.40),
    }
}

pub fn shell_rule_color(theme: &CastTheme) -> Color32 {
    match theme.mode {
        ThemeMode::Light => shell_mix(theme.colors.border, theme.colors.surface, 0.36),
        ThemeMode::Dark => shell_mix(theme.colors.border, theme.colors.surface, 0.16),
    }
}

pub fn cast_scroll_area(id: &'static str, theme: &CastTheme) -> ScrollArea {
    ScrollArea::vertical()
        .id_salt(id)
        .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
        .scroll_source(ScrollSource {
            scroll_bar: true,
            drag: theme.scroll.drag_to_scroll,
            mouse_wheel: true,
        })
        .wheel_scroll_multiplier(egui::vec2(1.0, theme.scroll.wheel_multiplier))
}

fn sidebar_workspace_switcher(ui: &mut egui::Ui, theme: &CastTheme) {
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 44.0), egui::Sense::hover());
    if ui.is_rect_visible(rect) {
        ui.painter().rect(
            rect,
            egui::CornerRadius::same(theme.radius.md.round() as u8),
            Color32::from_rgba_unmultiplied(255, 255, 255, 20),
            egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 28)),
            egui::StrokeKind::Outside,
        );
        ui.painter().text(
            rect.left_center() + egui::vec2(14.0, 0.0),
            egui::Align2::LEFT_CENTER,
            "Cast UI",
            theme.typography.button.clone(),
            Color32::WHITE,
        );
        ui.painter().text(
            rect.right_center() - egui::vec2(14.0, 0.0),
            egui::Align2::RIGHT_CENTER,
            "v0.1",
            theme.typography.caption.clone(),
            sidebar_muted_text(),
        );
    }
}

fn sidebar_group_label(ui: &mut egui::Ui, label: &str) {
    ui.add_space(4.0);
    ui.label(
        RichText::new(label)
            .size(12.0)
            .color(Color32::from_rgba_unmultiplied(255, 255, 255, 128)),
    );
    ui.add_space(4.0);
}

fn sidebar_nav_item(
    ui: &mut egui::Ui,
    theme: &CastTheme,
    label: &str,
    selected: bool,
) -> egui::Response {
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, 36.0), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let fill = if selected {
            Color32::from_rgba_unmultiplied(255, 255, 255, 44)
        } else if response.hovered() {
            Color32::from_rgba_unmultiplied(255, 255, 255, 24)
        } else {
            Color32::TRANSPARENT
        };
        let fg = if selected {
            Color32::WHITE
        } else {
            Color32::from_rgba_unmultiplied(255, 255, 255, 190)
        };
        ui.painter().rect_filled(
            rect.shrink2(egui::vec2(0.0, 2.0)),
            egui::CornerRadius::same(theme.radius.md.round() as u8),
            fill,
        );
        ui.painter().text(
            rect.left_center() + egui::vec2(14.0, 0.0),
            egui::Align2::LEFT_CENTER,
            label,
            theme.typography.button.clone(),
            fg,
        );
    }
    response
}

fn sidebar_status_row(ui: &mut egui::Ui, theme: &CastTheme, label: &str, value: &str) {
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 28.0), egui::Sense::hover());
    if ui.is_rect_visible(rect) {
        ui.painter().text(
            rect.left_center() + egui::vec2(14.0, 0.0),
            egui::Align2::LEFT_CENTER,
            label,
            theme.typography.caption.clone(),
            sidebar_muted_text(),
        );
        ui.painter().text(
            rect.right_center() - egui::vec2(14.0, 0.0),
            egui::Align2::RIGHT_CENTER,
            value,
            theme.typography.caption.clone(),
            Color32::from_rgba_unmultiplied(255, 255, 255, 210),
        );
    }
}

fn sidebar_muted_text() -> Color32 {
    Color32::from_rgba_unmultiplied(255, 255, 255, 150)
}

fn shell_mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let channel = |a: u8, b: u8| f32::from(a).mul_add(1.0 - t, f32::from(b) * t).round() as u8;
    Color32::from_rgb(
        channel(a.r(), b.r()),
        channel(a.g(), b.g()),
        channel(a.b(), b.b()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_sidebar_fill_is_primary_driven_in_light_mode() {
        let theme = CastTheme::light();

        assert_eq!(
            shell_sidebar_fill(&theme),
            shell_mix(theme.colors.primary_family.base, Color32::BLACK, 0.78)
        );
    }

    #[test]
    fn shell_rule_color_is_derived_from_theme_border() {
        let theme = CastTheme::light();

        assert_eq!(
            shell_rule_color(&theme),
            shell_mix(theme.colors.border, theme.colors.surface, 0.36)
        );
    }
}
