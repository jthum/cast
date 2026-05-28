use std::sync::Arc;

use cast::{
    Alert, Badge, Button, Card, CastPaletteInput, CastTheme, Checkbox, Intent, Label, Link,
    NavList, Notice, Panel as CastPanel, Radio, SearchInput, SegmentedControl, SemanticColorTokens,
    Separator, Size, Slider, Switch, Tabs, TextInput, ThemeMode, ThemeSeed, TypographyTokens,
    Variant,
    egui::{
        self, CentralPanel, Color32, Panel as EguiPanel, RichText, ScrollArea,
        scroll_area::{ScrollBarVisibility, ScrollSource},
    },
};

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Cast Gallery",
        native_options,
        Box::new(|cc| {
            cast::install_cast_fonts(&cc.egui_ctx);
            let app = CastGallery::new();
            cast::set_theme(&cc.egui_ctx, app.theme.clone());
            Ok(Box::new(app))
        }),
    )
}

struct CastGallery {
    theme: CastTheme,
    seed: ThemeSeed,
    zoom: f32,
    search: String,
    command: String,
    name: String,
    handle: String,
    enabled: bool,
    notifications: bool,
    indeterminate: bool,
    form_density: usize,
    foundation_tab: usize,
    workflow_segment: usize,
    sidebar_section: usize,
}

impl CastGallery {
    fn new() -> Self {
        let mode = ThemeMode::Light;
        let seed = ThemeSeed::for_mode(mode).with_typography(TypographyTokens::cast());
        let theme = seed.clone().resolve();

        Self {
            theme,
            seed,
            zoom: 1.0,
            search: String::new(),
            command: String::from("Refine the component gallery into an app-like surface"),
            name: String::from("Cast"),
            handle: String::new(),
            enabled: true,
            notifications: true,
            indeterminate: false,
            form_density: 1,
            foundation_tab: 0,
            workflow_segment: 0,
            sidebar_section: 0,
        }
    }

    fn set_mode(&mut self, ctx: &egui::Context, mode: ThemeMode) {
        self.seed.mode = mode;
        self.apply_theme(ctx);
    }

    fn apply_theme(&mut self, ctx: &egui::Context) {
        self.theme = self.seed.clone().resolve();
        cast::set_theme(ctx, self.theme.clone());
    }
}

impl eframe::App for CastGallery {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        ctx.set_zoom_factor(self.zoom);

        EguiPanel::top("top_bar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Cast");
                ui.add(Badge::new("Gallery").intent(Intent::Info));
                ui.separator();
                let mut mode_index = match self.seed.mode {
                    ThemeMode::Light => 0,
                    ThemeMode::Dark => 1,
                };
                let previous_mode_index = mode_index;
                ui.add(SegmentedControl::new(&mut mode_index, ["Light", "Dark"]).size(Size::Small));
                if mode_index != previous_mode_index {
                    let mode = if mode_index == 0 {
                        ThemeMode::Light
                    } else {
                        ThemeMode::Dark
                    };
                    self.set_mode(&ctx, mode);
                }
                ui.separator();
                ui.label("Zoom");
                if ui
                    .add(
                        Slider::new(&mut self.zoom, 0.9..=1.35)
                            .show_value(false)
                            .width(140.0),
                    )
                    .changed()
                {
                    ctx.set_zoom_factor(self.zoom);
                }
            });
        });

        EguiPanel::left("sidebar")
            .resizable(false)
            .default_size(220.0)
            .show_inside(ui, |ui| {
                cast_scroll_area("sidebar_scroll", &self.theme)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.heading("Workspace");
                        ui.add_space(6.0);
                        ui.add(
                            SearchInput::new(&mut self.search)
                                .hint_text("Search components")
                                .width(184.0),
                        );
                        ui.add_space(10.0);
                        ui.add(NavList::new(
                            &mut self.sidebar_section,
                            ["Workbench", "Foundations", "Components", "Theme lab"],
                        ));
                        ui.separator();
                        if show_theme_editor(ui, &mut self.seed) {
                            self.apply_theme(&ctx);
                        }
                    });
            });

        CentralPanel::default().show_inside(ui, |ui| {
            cast_scroll_area("main_scroll", &self.theme)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    show_workspace_view(
                        ui,
                        self.sidebar_section,
                        &self.theme,
                        ctx.pixels_per_point(),
                        self.zoom,
                        &mut self.command,
                        &mut self.search,
                        &mut self.name,
                        &mut self.handle,
                        &mut self.enabled,
                        &mut self.notifications,
                        &mut self.indeterminate,
                        &mut self.form_density,
                        &mut self.foundation_tab,
                        &mut self.workflow_segment,
                    );
                });
        });
    }
}

fn cast_scroll_area(id: &'static str, theme: &CastTheme) -> ScrollArea {
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

#[allow(clippy::too_many_arguments)]
fn show_workspace_view(
    ui: &mut egui::Ui,
    section: usize,
    theme: &CastTheme,
    pixels_per_point: f32,
    zoom: f32,
    command: &mut String,
    search: &mut String,
    name: &mut String,
    handle: &mut String,
    enabled: &mut bool,
    notifications: &mut bool,
    indeterminate: &mut bool,
    form_density: &mut usize,
    foundation_tab: &mut usize,
    workflow_segment: &mut usize,
) {
    match section {
        0 => {
            workspace_header(
                ui,
                "Agent workspace",
                "A composed Cast surface with navigation, forms, status, and feedback.",
                Intent::Primary,
            );
            ui.add_space(12.0);
            show_workbench_preview(ui, theme, command, workflow_segment);
            ui.add_space(12.0);
            show_navigation_layout(ui, foundation_tab, workflow_segment);
            ui.add_space(12.0);
            show_forms(
                ui,
                search,
                name,
                handle,
                enabled,
                notifications,
                indeterminate,
                form_density,
            );
        }
        1 => {
            workspace_header(
                ui,
                "Foundations",
                "Runtime theme switching, live palette editing, semantic tokens, and typography.",
                Intent::Info,
            );
            ui.add_space(12.0);
            show_theme_foundation(ui);
            ui.add_space(12.0);
            show_palette_preview(ui, theme);
            ui.add_space(12.0);
            show_typography_gallery(ui, theme);
            ui.add_space(12.0);
            show_typography_diagnostics(ui, theme, pixels_per_point, zoom);
        }
        2 => {
            workspace_header(
                ui,
                "Components",
                "Current primitives with states, variants, forms, and baseline egui comparisons.",
                Intent::Secondary,
            );
            ui.add_space(12.0);
            show_override_preview(ui);
            ui.add_space(12.0);
            show_buttons_and_badges(ui);
            ui.add_space(12.0);
            show_surfaces(ui);
            ui.add_space(12.0);
            show_text_and_feedback(ui);
            ui.add_space(12.0);
            show_forms(
                ui,
                search,
                name,
                handle,
                enabled,
                notifications,
                indeterminate,
                form_density,
            );
            ui.add_space(12.0);
            show_raw_egui_controls(ui, search, enabled);
        }
        _ => {
            workspace_header(
                ui,
                "Theme lab",
                "A focused view for token derivation, live overrides, type diagnostics, and palette checks.",
                Intent::Success,
            );
            ui.add_space(12.0);
            show_palette_preview(ui, theme);
            ui.add_space(12.0);
            show_typography_diagnostics(ui, theme, pixels_per_point, zoom);
            ui.add_space(12.0);
            show_override_preview(ui);
        }
    }
}

fn workspace_header(ui: &mut egui::Ui, title: &str, subtitle: &str, intent: Intent) {
    let theme = cast::theme_for_ui(ui);
    ui.horizontal_wrapped(|ui| {
        ui.heading(RichText::new(title).size(24.0));
        ui.add(Badge::new("Live").intent(intent));
    });
    ui.label(
        RichText::new(subtitle)
            .font(theme.typography.body.clone())
            .color(theme.colors.text_muted)
            .extra_letter_spacing(theme.typography.letter_spacing),
    );
}

fn show_workbench_preview(
    ui: &mut egui::Ui,
    theme: &CastTheme,
    command: &mut String,
    workflow_segment: &mut usize,
) {
    ui.columns(2, |columns| {
        CastPanel::new().show(&mut columns[0], |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.heading("Run composer");
                ui.add(Badge::new("Ready").intent(Intent::Success));
            });
            ui.add_space(8.0);
            ui.add(
                TextInput::new(command)
                    .label("Instruction")
                    .hint_text("Ask Cast to refine a surface")
                    .help_text(
                        "Previewing field anatomy, status, and actions in one composed pane.",
                    )
                    .width(ui.available_width()),
            );
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                ui.add(Button::new("Run").leading_icon("[>]"));
                ui.add(
                    Button::new("Review")
                        .intent(Intent::Secondary)
                        .variant(Variant::Outline),
                );
                ui.add(Button::new("Save preset").variant(Variant::Ghost));
            });
            ui.add(Separator::new().spacing(12.0));
            activity_row(ui, "01", "Inspect theme tokens", "Done", Intent::Success);
            activity_row(
                ui,
                "02",
                "Tune input and navigation states",
                "Active",
                Intent::Info,
            );
            activity_row(
                ui,
                "03",
                "Review widget-specific feedback",
                "Next",
                Intent::Neutral,
            );
        });

        CastPanel::new().show(&mut columns[1], |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.heading("Interface state");
                ui.add(SegmentedControl::new(
                    workflow_segment,
                    ["Design", "Build", "Ship"],
                ));
            });
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                metric_tile(ui, "Accent", "Primary", theme.colors.primary_family.base);
                metric_tile(
                    ui,
                    "Radius",
                    format!("{:.0}px", theme.radius.md),
                    theme.colors.border,
                );
                metric_tile(
                    ui,
                    "Type",
                    format!("{:.0}px", theme.typography.body.size),
                    theme.colors.secondary_family.base,
                );
            });
            ui.add(Separator::new().spacing(12.0));
            ui.add(
                Alert::new("Theme-safe by default")
                    .body("The preview is using the same runtime tokens exposed in the editor.")
                    .intent(Intent::Info),
            );
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                ui.add(Badge::new("Accessible").intent(Intent::Success));
                ui.add(Badge::new("Immediate mode").intent(Intent::Secondary));
                ui.add(Badge::new("Runtime theme").intent(Intent::Primary));
            });
        });
    });
}

fn activity_row(ui: &mut egui::Ui, number: &str, label: &str, status: &str, intent: Intent) {
    let theme = cast::theme_for_ui(ui);
    ui.horizontal_wrapped(|ui| {
        ui.add_sized(
            [28.0, 22.0],
            egui::Label::new(
                RichText::new(number)
                    .font(theme.typography.caption.clone())
                    .color(theme.colors.text_subtle)
                    .extra_letter_spacing(theme.typography.letter_spacing),
            ),
        );
        ui.label(
            RichText::new(label)
                .font(theme.typography.body.clone())
                .color(theme.colors.text)
                .extra_letter_spacing(theme.typography.letter_spacing),
        );
        ui.add(Badge::new(status).intent(intent).size(Size::Small));
    });
}

fn metric_tile(ui: &mut egui::Ui, label: &str, value: impl Into<String>, color: Color32) {
    let theme = cast::theme_for_ui(ui);
    let width = 96.0;
    let height = 58.0;
    let (rect, _response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());

    if ui.is_rect_visible(rect) {
        ui.painter().rect(
            rect,
            egui::CornerRadius::same(theme.radius.md as u8),
            theme.colors.surface,
            egui::Stroke::new(theme.stroke.sm, theme.colors.border),
            egui::StrokeKind::Outside,
        );
        let swatch = egui::Rect::from_min_size(
            rect.min + egui::vec2(theme.spacing.sm, theme.spacing.sm),
            egui::vec2(10.0, 10.0),
        );
        ui.painter()
            .rect_filled(swatch, egui::CornerRadius::same(2), color);
        ui.painter().text(
            rect.min + egui::vec2(theme.spacing.sm + 16.0, theme.spacing.xs + 2.0),
            egui::Align2::LEFT_TOP,
            label,
            theme.typography.caption.clone(),
            theme.colors.text_subtle,
        );
        ui.painter().text(
            rect.min + egui::vec2(theme.spacing.sm, 30.0),
            egui::Align2::LEFT_TOP,
            value.into(),
            theme.typography.strong.clone(),
            theme.colors.text,
        );
    }
}

fn show_theme_editor(ui: &mut egui::Ui, seed: &mut ThemeSeed) -> bool {
    let mut changed = false;
    ui.heading("Theme");

    egui::CollapsingHeader::new("Palette")
        .default_open(true)
        .show(ui, |ui| changed |= show_palette_editor(ui, seed));
    egui::CollapsingHeader::new("Tokens")
        .default_open(true)
        .show(ui, |ui| changed |= show_token_editor(ui, seed));
    egui::CollapsingHeader::new("Typography")
        .default_open(false)
        .show(ui, |ui| changed |= show_typography_editor(ui, seed));
    egui::CollapsingHeader::new("Motion")
        .default_open(false)
        .show(ui, |ui| changed |= show_motion_editor(ui, seed));
    egui::CollapsingHeader::new("Presets")
        .default_open(false)
        .show(ui, |ui| changed |= show_preset_editor(ui, seed));
    egui::CollapsingHeader::new("Overrides")
        .default_open(false)
        .show(ui, |ui| changed |= show_override_editor(ui, seed));

    ui.horizontal(|ui| {
        if ui.button("Reset").clicked() {
            *seed = ThemeSeed::for_mode(seed.mode).with_typography(TypographyTokens::cast());
            changed = true;
        }
        if ui.button("Primary only").clicked() {
            seed.palette = CastPaletteInput::from_primary(seed.palette.primary);
            changed = true;
        }
    });

    changed
}

fn show_palette_editor(ui: &mut egui::Ui, seed: &mut ThemeSeed) -> bool {
    let mut changed = false;
    changed |= color_row(ui, "Primary", &mut seed.palette.primary);
    changed |= optional_color_row(
        ui,
        "Secondary",
        &mut seed.palette.secondary,
        CastPaletteInput::default_for(seed.mode)
            .secondary
            .unwrap_or(Color32::from_rgb(124, 58, 237)),
    );
    changed |= optional_color_row(
        ui,
        "Neutral",
        &mut seed.palette.neutral,
        CastPaletteInput::default_for(seed.mode)
            .neutral
            .unwrap_or(Color32::from_rgb(100, 116, 139)),
    );
    changed |= optional_color_row(
        ui,
        "Success",
        &mut seed.palette.success,
        CastPaletteInput::default_for(seed.mode)
            .success
            .unwrap_or(Color32::from_rgb(22, 163, 74)),
    );
    changed |= optional_color_row(
        ui,
        "Warning",
        &mut seed.palette.warning,
        CastPaletteInput::default_for(seed.mode)
            .warning
            .unwrap_or(Color32::from_rgb(217, 119, 6)),
    );
    changed |= optional_color_row(
        ui,
        "Danger",
        &mut seed.palette.danger,
        CastPaletteInput::default_for(seed.mode)
            .danger
            .unwrap_or(Color32::from_rgb(220, 38, 38)),
    );
    changed |= optional_color_row(
        ui,
        "Info",
        &mut seed.palette.info,
        CastPaletteInput::default_for(seed.mode)
            .info
            .unwrap_or(Color32::from_rgb(8, 145, 178)),
    );
    changed
}

fn show_token_editor(ui: &mut egui::Ui, seed: &mut ThemeSeed) -> bool {
    let mut changed = false;
    let spacing_changed = theme_slider(ui, "Spacing", &mut seed.spacing.md, 8.0..=20.0);
    changed |= spacing_changed;
    if spacing_changed {
        seed.spacing.xs = seed.spacing.md / 3.0;
        seed.spacing.sm = seed.spacing.md * 2.0 / 3.0;
        seed.spacing.lg = seed.spacing.md * 4.0 / 3.0;
        seed.spacing.xl = seed.spacing.md * 2.0;
    }
    let radius_changed = theme_slider(ui, "Radius", &mut seed.radius.md, 0.0..=16.0);
    changed |= radius_changed;
    if radius_changed {
        seed.set_radius(seed.radius.md);
    }
    let stroke_changed = theme_slider(ui, "Border", &mut seed.stroke.sm, 0.0..=3.0);
    changed |= stroke_changed;
    if stroke_changed {
        seed.stroke.md = seed.stroke.sm + 0.5;
        seed.stroke.lg = seed.stroke.sm + 1.0;
    }
    let controls_changed = theme_slider(ui, "Control", &mut seed.controls.min_height, 26.0..=44.0);
    changed |= controls_changed;
    if controls_changed {
        seed.set_density(seed.controls.min_height, seed.spacing.md);
    }
    changed
}

fn show_typography_editor(ui: &mut egui::Ui, seed: &mut ThemeSeed) -> bool {
    let mut changed = false;

    ui.horizontal_wrapped(|ui| {
        if ui.button("Cast").clicked() {
            seed.typography = TypographyTokens::cast().with_body_size(seed.typography.body.size);
            changed = true;
        }
        if ui.button("Inter").clicked() {
            seed.typography = TypographyTokens::inter().with_body_size(seed.typography.body.size);
            changed = true;
        }
        if ui.button("System").clicked() {
            seed.typography = TypographyTokens::default().with_body_size(seed.typography.body.size);
            changed = true;
        }
        if ui.button("Compact").clicked() {
            seed.typography.set_body_size(13.0);
            changed = true;
        }
        if ui.button("Comfortable").clicked() {
            seed.typography.set_body_size(14.0);
            changed = true;
        }
        if ui.button("Large").clicked() {
            seed.typography.set_body_size(16.0);
            changed = true;
        }
    });

    let mut body_size = seed.typography.body.size;
    if theme_slider(ui, "Body size", &mut body_size, 12.0..=20.0) {
        seed.typography.set_body_size(body_size);
        changed = true;
    }
    changed |= theme_slider(
        ui,
        "Letter spacing",
        &mut seed.typography.letter_spacing,
        -0.25..=0.25,
    );

    if let Some(family) = font_family_selector(ui, "Body", &seed.typography.body.family) {
        seed.typography.set_body_family(family);
        changed = true;
    }
    if let Some(family) = font_family_selector(ui, "Heading", &seed.typography.heading.family) {
        seed.typography.set_heading_family(family);
        changed = true;
    }
    if let Some(family) = font_family_selector(ui, "Controls", &seed.typography.button.family) {
        seed.typography.set_button_family(family);
        changed = true;
    }
    if let Some(family) = font_family_selector(ui, "Strong", &seed.typography.strong.family) {
        seed.typography.set_strong_family(family);
        changed = true;
    }
    if let Some(family) = font_family_selector(ui, "Code", &seed.typography.code.family) {
        seed.typography.set_code_family(family);
        changed = true;
    }

    ui.add_space(4.0);
    ui.monospace(cast::FontStack::google_fonts_css2_url_for_names(&[
        "Inter",
        "JetBrains Mono",
    ]));

    changed
}

fn show_motion_editor(ui: &mut egui::Ui, seed: &mut ThemeSeed) -> bool {
    let mut changed = false;
    changed |= ui
        .checkbox(&mut seed.animation.enabled, "Animations")
        .changed();
    changed |= ui
        .checkbox(&mut seed.animation.reduced_motion, "Reduced motion")
        .changed();
    changed |= theme_slider(
        ui,
        "Duration",
        &mut seed.animation.duration_scale,
        0.0..=2.0,
    );
    changed |= ui
        .checkbox(&mut seed.scroll.drag_to_scroll, "Drag to scroll")
        .changed();
    changed |= theme_slider(
        ui,
        "Touchpad speed",
        &mut seed.scroll.wheel_multiplier,
        0.75..=4.0,
    );
    changed |= theme_slider(
        ui,
        "Wheel line speed",
        &mut seed.scroll.line_scroll_speed,
        20.0..=80.0,
    );
    changed
}

fn show_preset_editor(ui: &mut egui::Ui, seed: &mut ThemeSeed) -> bool {
    let mut changed = false;
    ui.horizontal_wrapped(|ui| {
        if ui.button("Compact").clicked() {
            seed.set_density(28.0, 10.0);
            changed = true;
        }
        if ui.button("Comfortable").clicked() {
            seed.set_density(36.0, 14.0);
            changed = true;
        }
        if ui.button("Sharp").clicked() {
            seed.set_radius(2.0);
            changed = true;
        }
        if ui.button("Soft").clicked() {
            seed.set_radius(10.0);
            changed = true;
        }
    });
    changed
}

fn show_override_editor(ui: &mut egui::Ui, seed: &mut ThemeSeed) -> bool {
    let mut changed = false;
    changed |= optional_theme_slider(
        ui,
        "Button radius",
        &mut seed.component_overrides.button.radius,
        seed.radius.md,
        0.0..=20.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Button border",
        &mut seed.component_overrides.button.border_width,
        seed.stroke.sm,
        0.0..=4.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Badge height",
        &mut seed.component_overrides.badge.min_height,
        seed.controls.min_height - 6.0,
        16.0..=44.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Input radius",
        &mut seed.component_overrides.input.radius,
        seed.radius.md,
        0.0..=20.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Input height",
        &mut seed.component_overrides.input.min_height,
        seed.controls.min_height,
        24.0..=52.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Card padding",
        &mut seed.component_overrides.card.padding,
        seed.spacing.lg,
        8.0..=32.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Card radius",
        &mut seed.component_overrides.card.radius,
        seed.radius.lg,
        0.0..=24.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Panel padding",
        &mut seed.component_overrides.panel.padding,
        seed.spacing.lg,
        8.0..=32.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Alert padding",
        &mut seed.component_overrides.alert.padding,
        seed.spacing.md,
        8.0..=32.0,
    );
    if !seed.component_overrides.is_empty() && ui.button("Clear overrides").clicked() {
        seed.component_overrides = Default::default();
        changed = true;
    }
    changed
}

fn theme_slider(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
) -> bool {
    ui.add(Slider::new(value, range).text(label)).changed()
}

fn optional_theme_slider(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Option<f32>,
    fallback: f32,
    range: std::ops::RangeInclusive<f32>,
) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        let mut enabled = value.is_some();
        if ui.checkbox(&mut enabled, label).changed() {
            *value = enabled.then_some(fallback);
            changed = true;
        }

        if let Some(value) = value {
            changed |= ui
                .add(Slider::new(value, range).show_value(false).width(148.0))
                .changed();
        }
    });

    changed
}

fn font_family_selector(
    ui: &mut egui::Ui,
    label: &str,
    current: &egui::FontFamily,
) -> Option<egui::FontFamily> {
    let mut selected = current.clone();
    ui.horizontal(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt(format!("font_family_{label}"))
            .selected_text(font_family_label(&selected))
            .show_ui(ui, |ui| {
                font_family_option(
                    ui,
                    &mut selected,
                    egui::FontFamily::Proportional,
                    "System UI",
                );
                font_family_option(
                    ui,
                    &mut selected,
                    egui::FontFamily::Monospace,
                    "System Mono",
                );
                font_family_option(
                    ui,
                    &mut selected,
                    inter_family(cast::FontStack::INTER_BODY_FAMILY),
                    "Inter Regular",
                );
                font_family_option(
                    ui,
                    &mut selected,
                    inter_family(cast::FontStack::INTER_BUTTON_FAMILY),
                    "Inter Medium",
                );
                font_family_option(
                    ui,
                    &mut selected,
                    inter_family(cast::FontStack::INTER_STRONG_FAMILY),
                    "Inter SemiBold",
                );
                font_family_option(
                    ui,
                    &mut selected,
                    inter_family(cast::FontStack::JETBRAINS_MONO_FAMILY),
                    "JetBrains Mono",
                );
            });
    });

    (selected != *current).then_some(selected)
}

fn font_family_option(
    ui: &mut egui::Ui,
    selected: &mut egui::FontFamily,
    family: egui::FontFamily,
    label: &str,
) {
    ui.selectable_value(selected, family, label);
}

fn inter_family(name: &'static str) -> egui::FontFamily {
    egui::FontFamily::Name(Arc::from(name))
}

fn font_family_label(family: &egui::FontFamily) -> String {
    match family {
        egui::FontFamily::Proportional => "System UI".to_owned(),
        egui::FontFamily::Monospace => "System Mono".to_owned(),
        egui::FontFamily::Name(name) if name.as_ref() == cast::FontStack::INTER_BODY_FAMILY => {
            "Inter Regular".to_owned()
        }
        egui::FontFamily::Name(name) if name.as_ref() == cast::FontStack::INTER_BUTTON_FAMILY => {
            "Inter Medium".to_owned()
        }
        egui::FontFamily::Name(name) if name.as_ref() == cast::FontStack::INTER_STRONG_FAMILY => {
            "Inter SemiBold".to_owned()
        }
        egui::FontFamily::Name(name) if name.as_ref() == cast::FontStack::JETBRAINS_MONO_FAMILY => {
            "JetBrains Mono".to_owned()
        }
        egui::FontFamily::Name(name) => name.to_string(),
    }
}

fn color_row(ui: &mut egui::Ui, label: &str, color: &mut Color32) -> bool {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.color_edit_button_srgba(color).changed()
    })
    .inner
}

fn optional_color_row(
    ui: &mut egui::Ui,
    label: &str,
    color: &mut Option<Color32>,
    fallback: Color32,
) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        let mut enabled = color.is_some();
        if ui.checkbox(&mut enabled, label).changed() {
            *color = enabled.then_some(fallback);
            changed = true;
        }

        if let Some(color) = color {
            changed |= ui.color_edit_button_srgba(color).changed();
        }
    });

    changed
}

fn show_theme_foundation(ui: &mut egui::Ui) {
    Card::new().show(ui, |ui| {
        ui.heading("Theme boundary");
        ui.label("Cast widgets resolve semantic styling from CastTheme.");
        ui.label("Raw egui widgets inherit the derived egui::Style fallback.");
        ui.horizontal_wrapped(|ui| {
            ui.add(Badge::new("CastTheme").intent(Intent::Primary));
            ui.add(Badge::new("egui::Style fallback").intent(Intent::Neutral));
            ui.add(Badge::new("runtime switching").intent(Intent::Info));
            ui.add(Badge::new("secondary").intent(Intent::Secondary));
        });
    });
}

fn show_palette_preview(ui: &mut egui::Ui, theme: &CastTheme) {
    Card::new().show(ui, |ui| {
        ui.heading("Derived palette");
        palette_family_row(ui, "Primary", theme.colors.primary_family);
        palette_family_row(ui, "Secondary", theme.colors.secondary_family);
        palette_family_row(ui, "Success", theme.colors.success_family);
        palette_family_row(ui, "Warning", theme.colors.warning_family);
        palette_family_row(ui, "Danger", theme.colors.danger_family);
        palette_family_row(ui, "Info", theme.colors.info_family);
    });
}

fn show_typography_gallery(ui: &mut egui::Ui, theme: &CastTheme) {
    Card::new().show(ui, |ui| {
        ui.heading("Typography");
        typography_sample(
            ui,
            "Heading large",
            "Build dense Rust interfaces with readable text.",
            theme.typography.heading_lg.clone(),
            theme.colors.text,
        );
        typography_sample(
            ui,
            "Heading",
            "Themeable components for immediate-mode apps.",
            theme.typography.heading.clone(),
            theme.colors.text,
        );
        typography_sample(
            ui,
            "Heading small",
            "Forms, surfaces, feedback, and navigation.",
            theme.typography.heading_sm.clone(),
            theme.colors.text,
        );
        typography_sample(
            ui,
            "Body",
            "Cast uses role-based font tokens so apps can choose separate faces for body text, headings, controls, and code.",
            theme.typography.body.clone(),
            theme.colors.text,
        );
        typography_sample(
            ui,
            "Body strong",
            "Important text can use a stronger face without changing size.",
            theme.typography.body_strong.clone(),
            theme.colors.text,
        );
        typography_sample(
            ui,
            "Small",
            "Secondary details should remain legible at small sizes.",
            theme.typography.small.clone(),
            theme.colors.text_muted,
        );
        typography_sample(
            ui,
            "Caption",
            "Caption text, metadata, and dense row annotations.",
            theme.typography.caption.clone(),
            theme.colors.text_subtle,
        );
        typography_sample(
            ui,
            "Code",
            "let theme = ThemeSeed::for_mode(mode).with_typography(TypographyTokens::inter());",
            theme.typography.code.clone(),
            theme.colors.text,
        );
    });
}

fn show_typography_diagnostics(
    ui: &mut egui::Ui,
    theme: &CastTheme,
    pixels_per_point: f32,
    zoom: f32,
) {
    Card::new().show(ui, |ui| {
        ui.heading("Text diagnostics");
        egui::Grid::new("typography_diagnostics_grid")
            .num_columns(2)
            .spacing(egui::vec2(12.0, 4.0))
            .show(ui, |ui| {
                diagnostic_row(ui, "Zoom", format!("{zoom:.2}"));
                diagnostic_row(ui, "Pixels/point", format!("{pixels_per_point:.2}"));
                diagnostic_row(
                    ui,
                    "Body size",
                    format!("{:.1}", theme.typography.body.size),
                );
                diagnostic_row(
                    ui,
                    "Caption size",
                    format!("{:.1}", theme.typography.caption.size),
                );
                diagnostic_row(
                    ui,
                    "Code size",
                    format!("{:.1}", theme.typography.code.size),
                );
                diagnostic_row(
                    ui,
                    "Letter spacing",
                    format!("{:.2}", theme.typography.letter_spacing),
                );
                diagnostic_row(
                    ui,
                    "Touchpad speed",
                    format!("{:.2}", theme.scroll.wheel_multiplier),
                );
                diagnostic_row(
                    ui,
                    "Wheel line speed",
                    format!("{:.1}", theme.scroll.line_scroll_speed),
                );
                diagnostic_row(
                    ui,
                    "Body family",
                    font_family_label(&theme.typography.body.family),
                );
                diagnostic_row(
                    ui,
                    "Code family",
                    font_family_label(&theme.typography.code.family),
                );
            });

        ui.add(Separator::new().spacing(10.0));
        typography_sample(
            ui,
            "Small",
            "Small text: abcdefghijklmnopqrstuvwxyz 0123456789",
            theme.typography.small.clone(),
            theme.colors.text,
        );
        typography_sample(
            ui,
            "Tracking",
            "Tighter letter spacing is a typography token, not a renderer fix.",
            theme.typography.body.clone(),
            theme.colors.text,
        );
        typography_sample(
            ui,
            "Caption",
            "Dense metadata: 2026-05-28 14:32:08 / queued / retry 02",
            theme.typography.caption.clone(),
            theme.colors.text_muted,
        );
        typography_sample(
            ui,
            "Mono",
            "fn main() { println!(\"Cast\"); }",
            theme.typography.code.clone(),
            theme.colors.text,
        );

        ui.add_space(6.0);
        for index in 0..3 {
            ui.horizontal(|ui| {
                ui.add_sized(
                    [28.0, 18.0],
                    egui::Label::new(
                        RichText::new(format!("{:02}", index + 1))
                            .font(theme.typography.caption.clone())
                            .color(theme.colors.text_subtle)
                            .extra_letter_spacing(theme.typography.letter_spacing),
                    ),
                );
                ui.label(
                    RichText::new("Render row with mixed weight, muted text, and stable spacing.")
                        .font(theme.typography.body.clone())
                        .color(theme.colors.text)
                        .extra_letter_spacing(theme.typography.letter_spacing),
                );
            });
        }
    });
}

fn diagnostic_row(ui: &mut egui::Ui, label: &str, value: impl Into<String>) {
    let letter_spacing = theme_letter_spacing(ui);
    ui.label(
        RichText::new(label)
            .size(11.0)
            .extra_letter_spacing(letter_spacing),
    );
    ui.label(value.into());
    ui.end_row();
}

fn typography_sample(
    ui: &mut egui::Ui,
    label: &str,
    text: &str,
    font: egui::FontId,
    color: Color32,
) {
    let letter_spacing = theme_letter_spacing(ui);
    ui.horizontal_wrapped(|ui| {
        ui.add_sized(
            [92.0, 18.0],
            egui::Label::new(
                RichText::new(label)
                    .size(11.0)
                    .extra_letter_spacing(letter_spacing),
            ),
        );
        ui.label(
            RichText::new(text)
                .font(font)
                .color(color)
                .extra_letter_spacing(letter_spacing),
        );
    });
}

fn theme_letter_spacing(ui: &egui::Ui) -> f32 {
    cast::theme_for_ui(ui).typography.letter_spacing
}

fn palette_family_row(ui: &mut egui::Ui, label: &str, family: SemanticColorTokens) {
    ui.horizontal(|ui| {
        ui.label(label);
        color_swatch(ui, family.base, "base");
        color_swatch(ui, family.subtle, "subtle");
        color_swatch(ui, family.muted, "muted");
        color_swatch(ui, family.border, "border");
        color_swatch(ui, family.hover, "hover");
        color_swatch(ui, family.active, "active");
    });
}

fn color_swatch(ui: &mut egui::Ui, color: Color32, label: &str) {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(24.0, 18.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 3, color);
    response.on_hover_text(label);
}

fn show_navigation_layout(
    ui: &mut egui::Ui,
    foundation_tab: &mut usize,
    workflow_segment: &mut usize,
) {
    Card::new().show(ui, |ui| {
        ui.heading("Navigation and layout");
        ui.add(Tabs::new(
            foundation_tab,
            ["Overview", "Components", "Theme", "Diagnostics"],
        ));
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            ui.add(SegmentedControl::new(
                workflow_segment,
                ["Design", "Build", "Ship"],
            ));
            ui.add(Badge::new(match *workflow_segment {
                0 => "Design review",
                1 => "Implementation",
                _ => "Release check",
            }));
        });
        ui.add_space(10.0);
        CastPanel::new().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(
                    Badge::new(match *foundation_tab {
                        0 => "Overview",
                        1 => "Components",
                        2 => "Theme",
                        _ => "Diagnostics",
                    })
                    .intent(Intent::Info),
                );
                ui.label(match *foundation_tab {
                    0 => "A compact route bar for switching between app sections.",
                    1 => "Tabs and segmented controls share sizing, tracking, and colors.",
                    2 => "Navigation follows runtime theme changes without extra setup.",
                    _ => "Focused state and hover affordances are painted by Cast.",
                });
            });
        });
    });
}

fn show_buttons_and_badges(ui: &mut egui::Ui) {
    Card::new().show(ui, |ui| {
        ui.heading("Buttons");
        ui.horizontal_wrapped(|ui| {
            ui.add(Button::new("Primary"));
            ui.add(Button::new("Secondary").intent(Intent::Secondary));
            ui.add(Button::new("Success").intent(Intent::Success));
            ui.add(Button::new("Warning").intent(Intent::Warning));
            ui.add(Button::new("Danger").intent(Intent::Danger));
            ui.add(
                Button::new("Outline")
                    .intent(Intent::Primary)
                    .variant(Variant::Outline),
            );
            ui.add(
                Button::new("Ghost")
                    .intent(Intent::Primary)
                    .variant(Variant::Ghost),
            );
        });

        ui.add_space(8.0);
        ui.heading("States");
        ui.horizontal_wrapped(|ui| {
            ui.add(Button::new("With icon").leading_icon("[+]"));
            ui.add(Button::new("Next").trailing_icon("[>]"));
            ui.add(Button::new("Saving").loading(true));
            ui.add(Button::new("Disabled").disabled());
        });

        ui.add_space(8.0);
        ui.heading("Sizes");
        ui.horizontal_wrapped(|ui| {
            ui.add(Button::new("Small").size(Size::Small));
            ui.add(Button::new("Medium").size(Size::Medium));
            ui.add(Button::new("Large").size(Size::Large));
        });

        ui.add_space(8.0);
        ui.heading("Badges");
        ui.horizontal_wrapped(|ui| {
            ui.add(Badge::new("Neutral"));
            ui.add(Badge::new("Primary").intent(Intent::Primary));
            ui.add(Badge::new("Secondary").intent(Intent::Secondary));
            ui.add(Badge::new("Success").intent(Intent::Success));
            ui.add(Badge::new("Warning").intent(Intent::Warning));
            ui.add(Badge::new("Danger").intent(Intent::Danger));
            ui.add(Badge::new("Info").intent(Intent::Info));
            ui.add(
                Badge::new("Outline")
                    .intent(Intent::Primary)
                    .variant(Variant::Outline),
            );
        });
    });
}

fn show_override_preview(ui: &mut egui::Ui) {
    let mut preview_input = String::from("Input");

    Card::new().show(ui, |ui| {
        ui.heading("Live override preview");
        ui.horizontal_wrapped(|ui| {
            ui.add(Button::new("Button").leading_icon("[+]"));
            ui.add(Badge::new("Badge").intent(Intent::Info));
            ui.add(TextInput::new(&mut preview_input).width(160.0));
        });
        ui.add_space(8.0);
        CastPanel::new().show(ui, |ui| {
            ui.label("Panel padding and radius update here.");
        });
        ui.add_space(8.0);
        ui.add(Alert::new("Alert preview").body("Alert padding and radius update here."));
    });
}

fn show_surfaces(ui: &mut egui::Ui) {
    Card::new().show(ui, |ui| {
        ui.heading("Surfaces");
        ui.label("Card frames primary content. Panel frames secondary or raised content.");
        ui.add(Separator::new());
        CastPanel::new().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(Badge::new("Panel").intent(Intent::Neutral));
                ui.label("A raised surface for dense app UI regions.");
            });
        });
    });
}

fn show_text_and_feedback(ui: &mut egui::Ui) {
    Card::new().show(ui, |ui| {
        ui.heading("Text and feedback");
        ui.horizontal_wrapped(|ui| {
            ui.add(Label::new("Default label"));
            ui.add(Label::new("Muted label").muted());
            ui.add(Label::new("Small label").size(Size::Small));
            ui.add(Link::new("Action link"));
            ui.add(Link::new("egui").to("https://github.com/emilk/egui"));
        });
        ui.add(Separator::new().spacing(10.0));
        ui.add(
            Alert::new("Build succeeded")
                .body("The latest component pass compiled and passed focused checks.")
                .intent(Intent::Success),
        );
        ui.add_space(8.0);
        ui.add(
            Alert::new("Review warning")
                .body("Palette derivation still needs dedicated contrast work.")
                .intent(Intent::Warning),
        );
        ui.add_space(8.0);
        ui.add(Notice::new("Neutral notice").body("Notices use the same feedback foundation."));
    });
}

#[allow(clippy::too_many_arguments)]
fn show_forms(
    ui: &mut egui::Ui,
    search: &mut String,
    name: &mut String,
    handle: &mut String,
    enabled: &mut bool,
    notifications: &mut bool,
    indeterminate: &mut bool,
    form_density: &mut usize,
) {
    Card::new().show(ui, |ui| {
        ui.heading("Forms");
        ui.horizontal_wrapped(|ui| {
            ui.add(
                TextInput::new(name)
                    .label("Project name")
                    .hint_text("Project name")
                    .help_text("Shown in window titles and saved presets.")
                    .width(240.0),
            );
            ui.add(
                SearchInput::new(search)
                    .label("Search")
                    .help_text("Filters the current gallery view.")
                    .width(240.0),
            );
        });
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            ui.add(
                TextInput::new(handle)
                    .label("Handle")
                    .hint_text("theme-handle")
                    .variant(Variant::Subtle)
                    .error_text("Required before publishing.")
                    .width(220.0),
            );
            ui.add(
                TextInput::new(name)
                    .label("Outline")
                    .hint_text("Outline input")
                    .variant(Variant::Outline)
                    .success_text("Looks ready.")
                    .width(220.0),
            );
            ui.add(
                TextInput::new(search)
                    .label("Ghost")
                    .hint_text("Ghost input")
                    .variant(Variant::Ghost)
                    .warning_text("Use sparingly in dense forms.")
                    .width(220.0),
            );
            ui.add(
                TextInput::new(name)
                    .label("Disabled")
                    .hint_text("Disabled input")
                    .disabled()
                    .help_text("Disabled state remains legible.")
                    .width(220.0),
            );
        });
        ui.add(Separator::new().spacing(10.0));
        ui.horizontal_wrapped(|ui| {
            ui.add(Checkbox::new(enabled, "Enabled"));
            ui.add(Checkbox::new(indeterminate, "Mixed").indeterminate(true));
            ui.add(Checkbox::new(notifications, "Disabled").disabled());
        });
        ui.horizontal_wrapped(|ui| {
            ui.add(Radio::new(form_density, 0, "Compact"));
            ui.add(Radio::new(form_density, 1, "Comfortable"));
            ui.add(Radio::new(form_density, 2, "Spacious"));
        });
        ui.horizontal(|ui| {
            ui.add(Switch::new(notifications));
            ui.label("Notifications");
        });
        ui.horizontal_wrapped(|ui| {
            ui.add(Switch::new(enabled).size(Size::Small));
            ui.add(Switch::new(enabled).size(Size::Medium));
            ui.add(Switch::new(enabled).size(Size::Large));
        });
    });
}

fn show_raw_egui_controls(ui: &mut egui::Ui, search: &mut String, enabled: &mut bool) {
    Card::new().show(ui, |ui| {
        ui.heading("Raw egui controls");
        ui.horizontal(|ui| {
            ui.label("Search");
            ui.text_edit_singleline(search);
        });
        ui.checkbox(enabled, "Enabled");
        ui.horizontal_wrapped(|ui| {
            let _ = ui.button("egui button");
            ui.hyperlink_to("egui link", "https://github.com/emilk/egui");
        });
    });
}
