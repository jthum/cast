use std::sync::Arc;

use cast::{
    Alert, Badge, Button, Card, CastPaletteInput, CastTheme, Checkbox, Intent, Label, Link, Notice,
    Panel as CastPanel, SearchInput, SemanticColorTokens, Separator, Size, Switch, TextInput,
    ThemeMode, ThemeSeed, TypographyTokens, Variant,
    egui::{
        self, CentralPanel, Color32, Panel as EguiPanel, RichText, ScrollArea,
        scroll_area::ScrollBarVisibility,
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
    name: String,
    enabled: bool,
    notifications: bool,
    indeterminate: bool,
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
            name: String::from("Cast"),
            enabled: true,
            notifications: true,
            indeterminate: false,
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
                ui.heading("Cast Gallery");
                ui.separator();
                let mut mode = self.seed.mode;
                ui.selectable_value(&mut mode, ThemeMode::Light, "Light");
                ui.selectable_value(&mut mode, ThemeMode::Dark, "Dark");
                if mode != self.seed.mode {
                    self.set_mode(&ctx, mode);
                }
                ui.separator();
                ui.label("Zoom");
                if ui
                    .add(egui::Slider::new(&mut self.zoom, 0.9..=1.35).show_value(false))
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
                ScrollArea::vertical()
                    .id_salt("sidebar_scroll")
                    .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.heading("Components");
                        ui.separator();
                        ui.label("Foundation");
                        ui.label("Button");
                        ui.label("Badge");
                        ui.label("Card");
                        ui.label("Inputs");
                        ui.separator();
                        if show_theme_editor(ui, &mut self.seed) {
                            self.apply_theme(&ctx);
                        }
                    });
            });

        CentralPanel::default().show_inside(ui, |ui| {
            ScrollArea::vertical()
                .id_salt("main_scroll")
                .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.heading(RichText::new("Foundations").size(24.0));
                    ui.label("Runtime theme switching, live palette editing, semantic tokens, and initial primitives.");
                    ui.add_space(12.0);

                    show_theme_foundation(ui);
                    ui.add_space(12.0);
                    show_palette_preview(ui, &self.theme);
                    ui.add_space(12.0);
                    show_typography_gallery(ui, &self.theme);
                    ui.add_space(12.0);
                    show_typography_diagnostics(
                        ui,
                        &self.theme,
                        ctx.pixels_per_point(),
                        self.zoom,
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
                        &mut self.search,
                        &mut self.name,
                        &mut self.enabled,
                        &mut self.notifications,
                        &mut self.indeterminate,
                    );
                    ui.add_space(12.0);
                    show_raw_egui_controls(ui, &mut self.search, &mut self.enabled);
                });
        });
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
    ui.add(egui::Slider::new(value, range).text(label))
        .changed()
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
            changed |= ui.add(egui::Slider::new(value, range)).changed();
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
                            .color(theme.colors.text_subtle),
                    ),
                );
                ui.label(
                    RichText::new("Render row with mixed weight, muted text, and stable spacing.")
                        .font(theme.typography.body.clone())
                        .color(theme.colors.text),
                );
            });
        }
    });
}

fn diagnostic_row(ui: &mut egui::Ui, label: &str, value: impl Into<String>) {
    ui.label(RichText::new(label).size(11.0));
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
    ui.horizontal_wrapped(|ui| {
        ui.add_sized(
            [92.0, 18.0],
            egui::Label::new(RichText::new(label).size(11.0)),
        );
        ui.label(RichText::new(text).font(font).color(color));
    });
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

fn show_forms(
    ui: &mut egui::Ui,
    search: &mut String,
    name: &mut String,
    enabled: &mut bool,
    notifications: &mut bool,
    indeterminate: &mut bool,
) {
    Card::new().show(ui, |ui| {
        ui.heading("Forms");
        ui.horizontal(|ui| {
            ui.label("Name");
            ui.add(TextInput::new(name).hint_text("Project name").width(220.0));
        });
        ui.horizontal(|ui| {
            ui.label("Search");
            ui.add(SearchInput::new(search).width(220.0));
        });
        ui.horizontal(|ui| {
            ui.label("Outline");
            ui.add(
                TextInput::new(name)
                    .hint_text("Outline input")
                    .variant(Variant::Outline)
                    .width(220.0),
            );
        });
        ui.horizontal(|ui| {
            ui.label("Ghost");
            ui.add(
                TextInput::new(search)
                    .hint_text("Ghost input")
                    .variant(Variant::Ghost)
                    .width(220.0),
            );
        });
        ui.horizontal(|ui| {
            ui.label("Disabled");
            ui.add(
                TextInput::new(name)
                    .hint_text("Disabled input")
                    .disabled()
                    .width(220.0),
            );
        });
        ui.add(Separator::new().spacing(10.0));
        ui.horizontal_wrapped(|ui| {
            ui.add(Checkbox::new(enabled, "Enabled"));
            ui.add(Checkbox::new(indeterminate, "Mixed").indeterminate(true));
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
