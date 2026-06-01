use cast::{
    Button, CastTheme, Size, ThemeMode, Variant,
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
            topbar_height: 60.0,
            content_margin: 28.0,
            compact_content_margin: 16.0,
            sidebar_margin: 14,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SidebarChildRoute<'a> {
    pub route: usize,
    pub label: &'a str,
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
        }
    }
}

pub fn show_shell_top_bar(
    ui: &mut egui::Ui,
    seed: &mut cast::ThemeSeed,
    follows_system_theme: &mut bool,
) -> bool {
    show_app_top_bar(ui, "Cast Gallery", seed, follows_system_theme)
}

pub fn show_shell_top_bar_with_sidebar_button(
    ui: &mut egui::Ui,
    seed: &mut cast::ThemeSeed,
    follows_system_theme: &mut bool,
) -> (bool, bool) {
    show_app_top_bar_with_sidebar_button(ui, "Cast Gallery", seed, follows_system_theme)
}

pub fn show_app_top_bar(
    ui: &mut egui::Ui,
    title: &str,
    seed: &mut cast::ThemeSeed,
    follows_system_theme: &mut bool,
) -> bool {
    show_app_top_bar_inner(ui, title, seed, follows_system_theme, None)
}

pub fn show_app_top_bar_with_sidebar_button(
    ui: &mut egui::Ui,
    title: &str,
    seed: &mut cast::ThemeSeed,
    follows_system_theme: &mut bool,
) -> (bool, bool) {
    let mut sidebar_requested = false;
    let changed = show_app_top_bar_inner(
        ui,
        title,
        seed,
        follows_system_theme,
        Some(&mut sidebar_requested),
    );
    (changed, sidebar_requested)
}

fn show_app_top_bar_inner(
    ui: &mut egui::Ui,
    title: &str,
    seed: &mut cast::ThemeSeed,
    follows_system_theme: &mut bool,
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
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if theme_mode_toggle(ui, seed.mode, *follows_system_theme).clicked() {
                    seed.mode = match seed.mode {
                        ThemeMode::Light => ThemeMode::Dark,
                        ThemeMode::Dark => ThemeMode::Light,
                    };
                    *follows_system_theme = false;
                    changed = true;
                }
            });
        },
    );
    changed
}

fn theme_mode_toggle(
    ui: &mut egui::Ui,
    mode: ThemeMode,
    follows_system_theme: bool,
) -> egui::Response {
    let size = egui::vec2(32.0, 32.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let theme = cast::theme_for_ui(ui);
    let hovered = response.hovered();
    let fill = if hovered {
        theme.colors.surface_muted
    } else {
        Color32::TRANSPARENT
    };
    let stroke = if hovered {
        egui::Stroke::new(theme.stroke.sm, theme.colors.border)
    } else {
        egui::Stroke::new(
            theme.stroke.sm,
            cast::mix_with_transparent(theme.colors.border, 0.55),
        )
    };

    if ui.is_rect_visible(rect) {
        ui.painter().rect(
            rect,
            egui::CornerRadius::same(theme.radius.md.round() as u8),
            fill,
            stroke,
            egui::StrokeKind::Outside,
        );

        match mode {
            ThemeMode::Light => paint_sun_icon(ui, rect.center(), theme.colors.text),
            ThemeMode::Dark => paint_moon_icon(
                ui,
                rect.center(),
                theme.colors.text,
                if hovered {
                    theme.colors.surface_muted
                } else {
                    theme.colors.surface
                },
            ),
        }
    }

    let mode_label = match mode {
        ThemeMode::Light => "light",
        ThemeMode::Dark => "dark",
    };
    let tooltip = if follows_system_theme {
        format!("Using system {mode_label} theme. Click to switch manually.")
    } else {
        format!("Using {mode_label} theme. Click to switch.")
    };
    response.on_hover_text(tooltip)
}

fn paint_sun_icon(ui: &egui::Ui, center: egui::Pos2, color: Color32) {
    let stroke = egui::Stroke::new(1.5, color);
    ui.painter().circle_stroke(center, 4.2, stroke);
    for index in 0..8 {
        let angle = index as f32 * std::f32::consts::TAU / 8.0;
        let direction = egui::vec2(angle.cos(), angle.sin());
        ui.painter().line_segment(
            [center + direction * 7.0, center + direction * 10.0],
            stroke,
        );
    }
}

fn paint_moon_icon(ui: &egui::Ui, center: egui::Pos2, color: Color32, cutout: Color32) {
    let painter = ui.painter();
    painter.circle_filled(center + egui::vec2(1.0, 0.0), 8.0, color);
    painter.circle_filled(center + egui::vec2(4.5, -2.0), 7.8, cutout);
}

#[allow(dead_code)]
pub fn show_shell_sidebar(ui: &mut egui::Ui, theme: &CastTheme, selected: &mut usize) {
    show_app_sidebar(ui, theme, &AppShellConfig::default(), selected);
}

pub fn show_shell_sidebar_tree(
    ui: &mut egui::Ui,
    theme: &CastTheme,
    selected: &mut usize,
    components_open: &mut bool,
    component_children: &[SidebarChildRoute<'_>],
) {
    show_app_sidebar_tree(
        ui,
        theme,
        &AppShellConfig::default(),
        selected,
        components_open,
        component_children,
    );
}

#[allow(dead_code)]
pub fn show_shell_sidebar_drawer(
    ctx: &egui::Context,
    theme: &CastTheme,
    open: &mut bool,
    selected: &mut usize,
    metrics: AppShellMetrics,
) {
    let id = egui::Id::new("cast_gallery_sidebar_drawer");
    let slide_progress = drawer_animation_progress(ctx, id.with("slide"), *open, theme);

    if !*open && slide_progress <= 0.001 {
        return;
    }

    let screen_rect = ctx.content_rect();
    let width = metrics
        .compact_sidebar_width
        .min((screen_rect.width() - theme.spacing.md).max(260.0));
    let offset = -width * (1.0 - slide_progress.clamp(0.0, 1.0));
    let mut close_requested = false;

    let backdrop_response = egui::Area::new(id.with("backdrop"))
        .order(egui::Order::Middle)
        .fixed_pos(screen_rect.min)
        .show(ctx, |ui| {
            let alpha = match theme.mode {
                ThemeMode::Light => 92,
                ThemeMode::Dark => 148,
            };
            let backdrop_alpha = (alpha as f32 * slide_progress.clamp(0.0, 1.0)).round() as u8;
            ui.painter()
                .rect_filled(screen_rect, 0.0, Color32::from_black_alpha(backdrop_alpha));
            let (_, response) = ui.allocate_exact_size(screen_rect.size(), egui::Sense::click());
            response
        });

    if *open && backdrop_response.inner.clicked() {
        close_requested = true;
    }

    egui::Area::new(id)
        .order(egui::Order::Foreground)
        .fixed_pos(screen_rect.min + egui::vec2(offset, 0.0))
        .show(ctx, |ui| {
            let margin = f32::from(metrics.sidebar_margin) * 2.0;
            let inner_size = egui::vec2(
                (width - margin).max(0.0),
                (screen_rect.height() - margin).max(0.0),
            );

            egui::Frame::new()
                .fill(shell_sidebar_fill(theme))
                .stroke(egui::Stroke::NONE)
                .inner_margin(egui::Margin::symmetric(
                    metrics.sidebar_margin,
                    metrics.sidebar_margin,
                ))
                .show(ui, |ui| {
                    ui.set_min_size(inner_size);
                    ui.set_max_size(inner_size);
                    cast_scroll_area("compact_sidebar_scroll", theme)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            let previous_section = *selected;
                            show_shell_sidebar(ui, theme, selected);
                            if *selected != previous_section {
                                close_requested = true;
                            }
                        });
                });
        });

    if close_requested {
        *open = false;
    }
}

pub fn show_shell_sidebar_drawer_tree(
    ctx: &egui::Context,
    theme: &CastTheme,
    open: &mut bool,
    selected: &mut usize,
    components_open: &mut bool,
    component_children: &[SidebarChildRoute<'_>],
    metrics: AppShellMetrics,
) {
    let id = egui::Id::new("cast_gallery_sidebar_drawer");
    let slide_progress = drawer_animation_progress(ctx, id.with("slide"), *open, theme);

    if !*open && slide_progress <= 0.001 {
        return;
    }

    let screen_rect = ctx.content_rect();
    let width = metrics
        .compact_sidebar_width
        .min((screen_rect.width() - theme.spacing.md).max(260.0));
    let offset = -width * (1.0 - slide_progress.clamp(0.0, 1.0));
    let mut close_requested = false;

    let backdrop_response = egui::Area::new(id.with("backdrop"))
        .order(egui::Order::Middle)
        .fixed_pos(screen_rect.min)
        .show(ctx, |ui| {
            let alpha = match theme.mode {
                ThemeMode::Light => 92,
                ThemeMode::Dark => 148,
            };
            let backdrop_alpha = (alpha as f32 * slide_progress.clamp(0.0, 1.0)).round() as u8;
            ui.painter()
                .rect_filled(screen_rect, 0.0, Color32::from_black_alpha(backdrop_alpha));
            let (_, response) = ui.allocate_exact_size(screen_rect.size(), egui::Sense::click());
            response
        });

    if *open && backdrop_response.inner.clicked() {
        close_requested = true;
    }

    egui::Area::new(id)
        .order(egui::Order::Foreground)
        .fixed_pos(screen_rect.min + egui::vec2(offset, 0.0))
        .show(ctx, |ui| {
            let margin = f32::from(metrics.sidebar_margin) * 2.0;
            let inner_size = egui::vec2(
                (width - margin).max(0.0),
                (screen_rect.height() - margin).max(0.0),
            );

            egui::Frame::new()
                .fill(shell_sidebar_fill(theme))
                .stroke(egui::Stroke::NONE)
                .inner_margin(egui::Margin::symmetric(
                    metrics.sidebar_margin,
                    metrics.sidebar_margin,
                ))
                .show(ui, |ui| {
                    ui.set_min_size(inner_size);
                    ui.set_max_size(inner_size);
                    cast_scroll_area("compact_sidebar_scroll", theme)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            let previous_section = *selected;
                            show_shell_sidebar_tree(
                                ui,
                                theme,
                                selected,
                                components_open,
                                component_children,
                            );
                            if *selected != previous_section {
                                close_requested = true;
                            }
                        });
                });
        });

    if close_requested {
        *open = false;
    }
}

fn drawer_animation_seconds(theme: &CastTheme) -> f32 {
    if theme.animation.should_animate() {
        theme.animation.normal_seconds() * 1.5
    } else {
        0.0
    }
}

fn drawer_animation_progress(
    ctx: &egui::Context,
    id: egui::Id,
    open: bool,
    theme: &CastTheme,
) -> f32 {
    let target = if open { 1.0 } else { 0.0 };
    let previous = ctx.data(|data| data.get_temp::<f32>(id).unwrap_or(0.0));
    let duration = drawer_animation_seconds(theme);
    let next = if duration <= 0.0 {
        target
    } else {
        let dt = ctx.input(|input| input.stable_dt.clamp(1.0 / 240.0, 0.05));
        move_toward(previous, target, dt / duration)
    };

    if (next - target).abs() > 0.001 {
        ctx.request_repaint();
    }

    ctx.data_mut(|data| data.insert_temp(id, next));
    egui::emath::easing::cubic_out(next.clamp(0.0, 1.0))
}

fn move_toward(value: f32, target: f32, step: f32) -> f32 {
    if value < target {
        (value + step).min(target)
    } else {
        (value - step).max(target)
    }
}

#[allow(dead_code)]
pub fn show_app_sidebar(
    ui: &mut egui::Ui,
    theme: &CastTheme,
    config: &AppShellConfig<'_>,
    selected: &mut usize,
) {
    ui.add_space(2.0);
    ui.label(
        RichText::new(config.title)
            .strong()
            .size(theme.typography.heading.size + 2.0)
            .color(Color32::WHITE),
    );
    ui.label(RichText::new(config.subtitle).color(sidebar_muted_text()));
    ui.add_space(10.0);

    sidebar_workspace_switcher(ui, theme, config.switcher_title, config.switcher_meta);
    ui.add_space(10.0);
    sidebar_group_label(ui, config.nav_group);
    for (index, label) in config.nav_items.iter().enumerate() {
        if sidebar_nav_item(ui, theme, label, *selected == index).clicked() {
            *selected = index;
        }
    }
}

pub fn show_app_sidebar_tree(
    ui: &mut egui::Ui,
    theme: &CastTheme,
    config: &AppShellConfig<'_>,
    selected: &mut usize,
    components_open: &mut bool,
    component_children: &[SidebarChildRoute<'_>],
) {
    ui.add_space(2.0);
    ui.label(
        RichText::new(config.title)
            .strong()
            .size(theme.typography.heading.size + 2.0)
            .color(Color32::WHITE),
    );
    ui.label(RichText::new(config.subtitle).color(sidebar_muted_text()));
    ui.add_space(10.0);

    sidebar_workspace_switcher(ui, theme, config.switcher_title, config.switcher_meta);
    ui.add_space(10.0);
    sidebar_group_label(ui, config.nav_group);
    let component_child_selected = component_children
        .iter()
        .any(|child| child.route == *selected);
    for (index, label) in config.nav_items.iter().enumerate() {
        if index == 2 {
            if sidebar_parent_nav_item(
                ui,
                theme,
                label,
                *selected == index || component_child_selected,
                *components_open,
            )
            .clicked()
            {
                *components_open = !*components_open;
                *selected = index;
            }
            if *components_open || component_child_selected {
                show_sidebar_children(ui, theme, selected, component_children);
            }
        } else if sidebar_nav_item(ui, theme, label, *selected == index).clicked() {
            *selected = index;
        }
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
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 36.0), egui::Sense::hover());
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
    let theme = cast::theme_for_ui(ui);
    ui.add_space(theme.stroke.lg);
    let label = label.to_ascii_uppercase();
    ui.label(
        RichText::new(label)
            .size(theme.typography.caption.size)
            .color(Color32::from_rgba_unmultiplied(255, 255, 255, 128)),
    );
    ui.add_space(theme.stroke.lg);
}

fn sidebar_nav_item(
    ui: &mut egui::Ui,
    theme: &CastTheme,
    label: &str,
    selected: bool,
) -> egui::Response {
    let metrics = sidebar_nav_metrics(theme);
    let width = ui.available_width();
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, metrics.item_height), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let paint_rect = sidebar_nav_paint_rect(rect, metrics);
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
            paint_rect.shrink2(egui::vec2(0.0, metrics.vertical_shrink)),
            egui::CornerRadius::same(theme.radius.md.round() as u8),
            fill,
        );
        ui.painter().text(
            paint_rect.left_center() + egui::vec2(metrics.item_inset, 0.0),
            egui::Align2::LEFT_CENTER,
            label,
            theme.typography.button.clone(),
            fg,
        );
    }
    response
}

fn sidebar_parent_nav_item(
    ui: &mut egui::Ui,
    theme: &CastTheme,
    label: &str,
    selected: bool,
    open: bool,
) -> egui::Response {
    let response = sidebar_nav_item(ui, theme, label, selected);
    if ui.is_rect_visible(response.rect) {
        let metrics = sidebar_nav_metrics(theme);
        let paint_rect = sidebar_nav_paint_rect(response.rect, metrics);
        paint_sidebar_caret(
            ui,
            paint_rect,
            metrics,
            open,
            Color32::from_rgba_unmultiplied(255, 255, 255, 170),
        );
    }
    response
}

fn paint_sidebar_caret(
    ui: &egui::Ui,
    rect: egui::Rect,
    metrics: SidebarNavMetrics,
    open: bool,
    color: Color32,
) {
    let center = rect.right_center() - egui::vec2(metrics.caret_inset, 0.0);
    let size = metrics.caret_size;
    let stroke = egui::Stroke::new(metrics.caret_stroke, color);
    if open {
        ui.painter().line_segment(
            [
                center + egui::vec2(-size, -size * 0.5),
                center + egui::vec2(0.0, size * 0.5),
            ],
            stroke,
        );
        ui.painter().line_segment(
            [
                center + egui::vec2(0.0, size * 0.5),
                center + egui::vec2(size, -size * 0.5),
            ],
            stroke,
        );
    } else {
        ui.painter().line_segment(
            [
                center + egui::vec2(-size * 0.5, -size),
                center + egui::vec2(size * 0.5, 0.0),
            ],
            stroke,
        );
        ui.painter().line_segment(
            [
                center + egui::vec2(size * 0.5, 0.0),
                center + egui::vec2(-size * 0.5, size),
            ],
            stroke,
        );
    }
}

fn show_sidebar_children(
    ui: &mut egui::Ui,
    theme: &CastTheme,
    selected: &mut usize,
    children: &[SidebarChildRoute<'_>],
) {
    if children.is_empty() {
        return;
    }

    let top = ui.cursor().min.y;
    for child in children {
        if sidebar_child_item(ui, theme, child.label, *selected == child.route).clicked() {
            *selected = child.route;
        }
    }
    let bottom = ui.cursor().min.y;
    let metrics = sidebar_nav_metrics(theme);
    let line_x = ui.min_rect().min.x + metrics.child_line_x;
    ui.painter().vline(
        line_x,
        top + metrics.item_gap..=bottom - metrics.item_gap,
        egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 42)),
    );
}

fn sidebar_child_item(
    ui: &mut egui::Ui,
    theme: &CastTheme,
    label: &str,
    selected: bool,
) -> egui::Response {
    let metrics = sidebar_nav_metrics(theme);
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(width, metrics.child_item_height),
        egui::Sense::click(),
    );
    if ui.is_rect_visible(rect) {
        let paint_rect = sidebar_child_paint_rect(rect, metrics);
        let fill = if selected {
            Color32::from_rgba_unmultiplied(255, 255, 255, 34)
        } else if response.hovered() {
            Color32::from_rgba_unmultiplied(255, 255, 255, 18)
        } else {
            Color32::TRANSPARENT
        };
        let fg = if selected {
            Color32::WHITE
        } else {
            Color32::from_rgba_unmultiplied(255, 255, 255, 174)
        };
        ui.painter().rect_filled(
            paint_rect.shrink2(egui::vec2(0.0, metrics.vertical_shrink)),
            egui::CornerRadius::same(theme.radius.sm.round() as u8),
            fill,
        );
        ui.painter().text(
            paint_rect.left_center() + egui::vec2(metrics.child_text_inset, 0.0),
            egui::Align2::LEFT_CENTER,
            label,
            theme.typography.small.clone(),
            fg,
        );
    }
    response
}

#[derive(Clone, Copy, Debug)]
struct SidebarNavMetrics {
    item_height: f32,
    child_item_height: f32,
    item_inset: f32,
    child_paint_start: f32,
    child_text_inset: f32,
    child_line_x: f32,
    caret_inset: f32,
    caret_size: f32,
    caret_stroke: f32,
    item_gap: f32,
    vertical_shrink: f32,
}

fn sidebar_nav_metrics(theme: &CastTheme) -> SidebarNavMetrics {
    let item_gap = theme.stroke.sm;
    SidebarNavMetrics {
        item_height: (theme.controls.min_height - theme.spacing.xs)
            .max(theme.typography.button.size + theme.spacing.sm),
        child_item_height: (theme.controls.min_height - theme.spacing.sm)
            .max(theme.typography.small.size + theme.spacing.sm),
        item_inset: theme.spacing.sm,
        child_line_x: theme.spacing.sm,
        child_paint_start: theme.spacing.sm + theme.spacing.xs,
        child_text_inset: theme.spacing.sm,
        caret_inset: theme.spacing.lg,
        caret_size: theme.spacing.xs,
        caret_stroke: theme.stroke.md,
        item_gap,
        vertical_shrink: item_gap * 0.5,
    }
}

fn sidebar_nav_paint_rect(rect: egui::Rect, metrics: SidebarNavMetrics) -> egui::Rect {
    rect.translate(egui::vec2(-metrics.item_inset, 0.0))
}

fn sidebar_child_paint_rect(rect: egui::Rect, metrics: SidebarNavMetrics) -> egui::Rect {
    egui::Rect::from_min_max(
        rect.min + egui::vec2(metrics.child_paint_start, 0.0),
        rect.max - egui::vec2(metrics.item_inset, 0.0),
    )
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
        assert_eq!(config.nav_items.len(), 5);
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

    #[test]
    fn sidebar_nav_metrics_follow_theme_tokens() {
        let theme = CastTheme::light();
        let metrics = sidebar_nav_metrics(&theme);

        assert_eq!(
            metrics.item_height,
            theme.controls.min_height - theme.spacing.xs
        );
        assert_eq!(
            metrics.child_item_height,
            theme.controls.min_height - theme.spacing.sm
        );
        assert_eq!(metrics.item_inset, theme.spacing.sm);
        assert_eq!(
            metrics.child_paint_start,
            theme.spacing.sm + theme.spacing.xs
        );
        assert_eq!(metrics.caret_inset, theme.spacing.lg);
        assert_eq!(metrics.caret_size, theme.spacing.xs);
    }

    #[test]
    fn sidebar_child_rect_aligns_with_parent_trailing_edge() {
        let theme = CastTheme::light();
        let metrics = sidebar_nav_metrics(&theme);
        let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(200.0, 28.0));
        let parent = sidebar_nav_paint_rect(rect, metrics);
        let child = sidebar_child_paint_rect(rect, metrics);

        assert_eq!(child.max.x, parent.max.x);
        assert!(child.min.x > metrics.child_line_x);
    }

    #[test]
    fn drawer_animation_is_longer_than_standard_motion() {
        let theme = CastTheme::light();

        assert!(drawer_animation_seconds(&theme) > theme.animation.normal_seconds());
    }
}
