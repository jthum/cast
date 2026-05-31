use cast::{
    Button, CastTheme, SegmentedControl, Size, Slider, ThemeMode, Variant,
    egui::{
        self, Color32, RichText, ScrollArea,
        scroll_area::{ScrollBarVisibility, ScrollSource},
    },
};
use std::hash::Hash;

#[derive(Clone, Copy, Debug)]
pub struct AppShellConfig<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub switcher_title: &'a str,
    pub switcher_meta: &'a str,
    pub nav_group: &'a str,
    pub nav_items: &'a [&'a str],
    pub status_group: &'a str,
    pub status_items: &'a [(&'a str, &'a str)],
}

#[derive(Clone, Copy, Debug)]
pub struct AppShellMetrics {
    pub compact_breakpoint: f32,
    pub sidebar_width: f32,
    pub compact_sidebar_width: f32,
    pub topbar_height: f32,
    pub content_margin: f32,
    pub compact_content_margin: f32,
    pub sidebar_margin: i8,
}

impl AppShellMetrics {
    #[must_use]
    pub fn is_compact(&self, available_width: f32) -> bool {
        available_width <= self.compact_breakpoint
    }

    #[must_use]
    pub fn content_margin_for_width(&self, available_width: f32) -> i8 {
        if self.is_compact(available_width) {
            self.compact_content_margin.round() as i8
        } else {
            self.content_margin.round() as i8
        }
    }

    #[must_use]
    pub fn topbar_margin_for_width(&self, available_width: f32) -> i8 {
        self.content_margin_for_width(available_width)
    }
}

impl Default for AppShellMetrics {
    fn default() -> Self {
        Self {
            compact_breakpoint: 900.0,
            sidebar_width: 248.0,
            compact_sidebar_width: 320.0,
            topbar_height: 68.0,
            content_margin: 28.0,
            compact_content_margin: 16.0,
            sidebar_margin: 18,
        }
    }
}

impl Default for AppShellConfig<'static> {
    fn default() -> Self {
        Self {
            title: "Cast",
            subtitle: "Themeable egui components",
            switcher_title: "Cast UI",
            switcher_meta: "v0.1",
            nav_group: "Workspace",
            nav_items: &[
                "Workbench",
                "Foundations",
                "Components",
                "Agent components",
                "Theme lab",
            ],
            status_group: "Status",
            status_items: &[("Runtime theme", "Live"), ("Components", "Ready")],
        }
    }
}

pub fn show_shell_top_bar(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    seed: &mut cast::ThemeSeed,
    zoom: &mut f32,
) -> bool {
    show_app_top_bar(ui, ctx, "Cast Gallery", seed, zoom)
}

pub fn show_shell_top_bar_with_sidebar_button(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    seed: &mut cast::ThemeSeed,
    zoom: &mut f32,
) -> (bool, bool) {
    show_app_top_bar_with_sidebar_button(ui, ctx, "Cast Gallery", seed, zoom)
}

pub fn show_app_top_bar(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    title: &str,
    seed: &mut cast::ThemeSeed,
    zoom: &mut f32,
) -> bool {
    show_app_top_bar_inner(ui, ctx, title, seed, zoom, None)
}

pub fn show_app_top_bar_with_sidebar_button(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    title: &str,
    seed: &mut cast::ThemeSeed,
    zoom: &mut f32,
) -> (bool, bool) {
    let mut sidebar_requested = false;
    let changed = show_app_top_bar_inner(ui, ctx, title, seed, zoom, Some(&mut sidebar_requested));
    (changed, sidebar_requested)
}

fn show_app_top_bar_inner(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    title: &str,
    seed: &mut cast::ThemeSeed,
    zoom: &mut f32,
    mut sidebar_requested: Option<&mut bool>,
) -> bool {
    let mut changed = false;
    let compact = ui.available_width() < 560.0;
    let title = if compact { "Cast" } else { title };
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), 32.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            if let Some(requested) = sidebar_requested.as_deref_mut() {
                if ui
                    .add(
                        Button::new("Menu")
                            .leading_icon("☰")
                            .variant(Variant::Outline)
                            .size(Size::Small),
                    )
                    .clicked()
                {
                    *requested = true;
                }
                ui.separator();
            }

            ui.label(title);
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
            if !compact {
                ui.label("Zoom");
            }
            let slider_width = if compact { 86.0 } else { 118.0 };
            if ui
                .add(
                    Slider::new(zoom, 0.9..=1.35)
                        .show_value(false)
                        .width(slider_width),
                )
                .changed()
            {
                ctx.set_zoom_factor(*zoom);
            }
        },
    );
    changed
}

pub fn show_shell_sidebar(ui: &mut egui::Ui, theme: &CastTheme, selected: &mut usize) {
    show_app_sidebar(ui, theme, &AppShellConfig::default(), selected);
}

pub fn show_app_sidebar(
    ui: &mut egui::Ui,
    theme: &CastTheme,
    config: &AppShellConfig<'_>,
    selected: &mut usize,
) {
    ui.add_space(6.0);
    ui.label(
        RichText::new(config.title)
            .strong()
            .size(theme.typography.heading.size + 2.0)
            .color(Color32::WHITE),
    );
    ui.label(RichText::new(config.subtitle).color(sidebar_muted_text()));
    ui.add_space(18.0);

    sidebar_workspace_switcher(ui, theme, config.switcher_title, config.switcher_meta);
    ui.add_space(18.0);
    sidebar_group_label(ui, config.nav_group);
    for (index, label) in config.nav_items.iter().enumerate() {
        if sidebar_nav_item(ui, theme, label, *selected == index).clicked() {
            *selected = index;
        }
    }

    ui.add_space(18.0);
    sidebar_group_label(ui, config.status_group);
    for (label, value) in config.status_items {
        sidebar_status_row(ui, theme, label, value);
    }
}

pub fn shell_sidebar_fill(theme: &CastTheme) -> Color32 {
    match theme.mode {
        ThemeMode::Light => shell_mix(theme.colors.primary_family.base, Color32::BLACK, 0.78),
        ThemeMode::Dark => shell_mix(theme.colors.surface, Color32::BLACK, 0.40),
    }
}

pub fn cast_scroll_area(id: impl Hash, theme: &CastTheme) -> ScrollArea {
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

pub fn cast_page_scroll_area(id: impl Hash, theme: &CastTheme) -> ScrollArea {
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

fn sidebar_workspace_switcher(ui: &mut egui::Ui, theme: &CastTheme, title: &str, meta: &str) {
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
            title,
            theme.typography.button.clone(),
            Color32::WHITE,
        );
        ui.painter().text(
            rect.right_center() - egui::vec2(14.0, 0.0),
            egui::Align2::RIGHT_CENTER,
            meta,
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
    fn page_scroll_area_uses_stable_ids() {
        let theme = CastTheme::light();

        let _ = cast_page_scroll_area(("main_scroll", 2usize, 1usize), &theme);
    }

    #[test]
    fn app_shell_config_defaults_match_gallery_shell() {
        let config = AppShellConfig::default();

        assert_eq!(config.title, "Cast");
        assert_eq!(config.nav_items[3], "Agent components");
        assert_eq!(config.status_items[0], ("Runtime theme", "Live"));
    }

    #[test]
    fn app_shell_metrics_switch_to_compact_layout_at_breakpoint() {
        let metrics = AppShellMetrics::default();

        assert!(!metrics.is_compact(metrics.compact_breakpoint + 1.0));
        assert!(metrics.is_compact(metrics.compact_breakpoint));
        assert_eq!(
            metrics.content_margin_for_width(metrics.compact_breakpoint - 1.0),
            metrics.compact_content_margin as i8
        );
    }
}
