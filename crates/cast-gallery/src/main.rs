use std::sync::Arc;

mod patterns;

use patterns::command_palette::{CommandPaletteState, show_command_palette};
use patterns::entity_table_with_details::{
    EntityRecord, EntityTableState, show_entity_table_with_details,
};
use patterns::related_activity::show_related_activity;
use patterns::shell::{
    cast_scroll_area, shell_rule_color, shell_sidebar_fill, show_shell_sidebar, show_shell_top_bar,
};

use cast::{
    Alert, Badge, Button, Card, CastPaletteInput, CastTheme, Checkbox, Dialog, Dropdown, Intent,
    Label, Link, MenuItem, Notice, Panel as CastPanel, Popover, Radio, SearchInput,
    SegmentedControl, SemanticColorTokens, Separator, Size, Slider, Switch, Tabs, TextInput,
    ThemeMode, ThemeSeed, Tooltip, TypographyTokens, Variant,
    egui::{self, CentralPanel, Color32, Panel as EguiPanel, RichText},
};

const LEAD_COUNT: usize = 24;

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
    lead_search: String,
    command: String,
    name: String,
    handle: String,
    enabled: bool,
    notifications: bool,
    indeterminate: bool,
    form_density: usize,
    menu_choice: usize,
    dialog_open: bool,
    command_palette: CommandPaletteState,
    related_activity_open: bool,
    related_activity_group: Option<usize>,
    lead_selected: [bool; LEAD_COUNT],
    lead_expanded: [bool; LEAD_COUNT],
    lead_date_filter: usize,
    lead_user_filter: usize,
    lead_status_filter: usize,
    lead_payment_filter: usize,
    lead_rows_per_page: usize,
    lead_page: usize,
    lead_exported_count: Option<usize>,
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
            lead_search: String::new(),
            command: String::from("Refine the component gallery into an app-like surface"),
            name: String::from("Cast"),
            handle: String::new(),
            enabled: true,
            notifications: true,
            indeterminate: false,
            form_density: 1,
            menu_choice: 0,
            dialog_open: false,
            command_palette: CommandPaletteState::default(),
            related_activity_open: false,
            related_activity_group: None,
            lead_selected: [false; LEAD_COUNT],
            lead_expanded: [false; LEAD_COUNT],
            lead_date_filter: 1,
            lead_user_filter: 0,
            lead_status_filter: 0,
            lead_payment_filter: 0,
            lead_rows_per_page: 0,
            lead_page: 0,
            lead_exported_count: None,
            foundation_tab: 0,
            workflow_segment: 0,
            sidebar_section: 0,
        }
    }

    fn apply_theme(&mut self, ctx: &egui::Context) {
        self.theme = self.seed.clone().resolve();
        cast::set_theme(ctx, self.theme.clone());
    }

    fn apply_command_palette_action(&mut self, action: &str) -> bool {
        match action {
            "open-workspace" => self.sidebar_section = 0,
            "show-components" => self.sidebar_section = 2,
            "theme-lab" => self.sidebar_section = 1,
            "export-table" => {
                self.lead_exported_count = Some(
                    self.lead_selected
                        .iter()
                        .filter(|selected| **selected)
                        .count(),
                );
            }
            "review-diagnostics" => self.sidebar_section = 1,
            "toggle-mode" => {
                let next = if self.seed.mode == ThemeMode::Light {
                    ThemeMode::Dark
                } else {
                    ThemeMode::Light
                };
                self.seed = self.seed.clone().with_mode(next);
                return true;
            }
            "reset-theme" => {
                self.seed =
                    ThemeSeed::for_mode(self.seed.mode).with_typography(TypographyTokens::cast());
                return true;
            }
            _ => {}
        }

        false
    }
}

impl eframe::App for CastGallery {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        ctx.set_zoom_factor(self.zoom);
        if ctx.input_mut(|input| input.consume_key(egui::Modifiers::COMMAND, egui::Key::K)) {
            self.command_palette.open = true;
        }

        EguiPanel::left("sidebar")
            .resizable(false)
            .default_size(248.0)
            .frame(
                egui::Frame::new()
                    .fill(shell_sidebar_fill(&self.theme))
                    .inner_margin(egui::Margin::symmetric(18, 18)),
            )
            .show_inside(ui, |ui| {
                cast_scroll_area("sidebar_scroll", &self.theme)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        show_shell_sidebar(ui, &self.theme, &mut self.sidebar_section);
                    });
            });

        let mut theme_changed = false;
        CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(self.theme.colors.background)
                    .inner_margin(egui::Margin::same(0)),
            )
            .show_inside(ui, |ui| {
                egui::Frame::new()
                    .fill(self.theme.colors.surface)
                    .inner_margin(egui::Margin::symmetric(28, 18))
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        theme_changed |=
                            show_shell_top_bar(ui, &ctx, &mut self.seed, &mut self.zoom);
                    });
                ui.painter().line_segment(
                    [
                        egui::pos2(ui.min_rect().min.x, ui.cursor().top()),
                        egui::pos2(ui.min_rect().max.x, ui.cursor().top()),
                    ],
                    egui::Stroke::new(self.theme.stroke.sm, shell_rule_color(&self.theme)),
                );

                egui::Frame::new()
                    .fill(self.theme.colors.background)
                    .inner_margin(egui::Margin::symmetric(28, 24))
                    .show(ui, |ui| {
                        cast_scroll_area("main_scroll", &self.theme)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                theme_changed |= show_workspace_view(
                                    ui,
                                    self.sidebar_section,
                                    &self.theme,
                                    &mut self.seed,
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
                                    &mut self.menu_choice,
                                    &mut self.dialog_open,
                                    &mut self.command_palette,
                                    &mut self.lead_search,
                                    &mut self.related_activity_open,
                                    &mut self.related_activity_group,
                                    &mut self.lead_selected,
                                    &mut self.lead_expanded,
                                    &mut self.lead_date_filter,
                                    &mut self.lead_user_filter,
                                    &mut self.lead_status_filter,
                                    &mut self.lead_payment_filter,
                                    &mut self.lead_rows_per_page,
                                    &mut self.lead_page,
                                    &mut self.lead_exported_count,
                                    &mut self.foundation_tab,
                                    &mut self.workflow_segment,
                                );
                            });
                    });
            });

        if let Some(action) = show_command_palette(&ctx, &mut self.command_palette) {
            theme_changed |= self.apply_command_palette_action(action);
        }

        if theme_changed {
            self.apply_theme(&ctx);
            ctx.set_zoom_factor(self.zoom);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn show_workspace_view(
    ui: &mut egui::Ui,
    section: usize,
    theme: &CastTheme,
    seed: &mut ThemeSeed,
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
    menu_choice: &mut usize,
    dialog_open: &mut bool,
    command_palette: &mut CommandPaletteState,
    lead_search: &mut String,
    related_activity_open: &mut bool,
    related_activity_group: &mut Option<usize>,
    lead_selected: &mut [bool; LEAD_COUNT],
    lead_expanded: &mut [bool; LEAD_COUNT],
    lead_date_filter: &mut usize,
    lead_user_filter: &mut usize,
    lead_status_filter: &mut usize,
    lead_payment_filter: &mut usize,
    lead_rows_per_page: &mut usize,
    lead_page: &mut usize,
    lead_exported_count: &mut Option<usize>,
    foundation_tab: &mut usize,
    workflow_segment: &mut usize,
) -> bool {
    let mut theme_changed = false;
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
            show_menus(ui, menu_choice, dialog_open, command_palette);
            ui.add_space(12.0);
            show_lists_and_tables(
                ui,
                lead_search,
                related_activity_open,
                related_activity_group,
                lead_selected,
                lead_expanded,
                lead_date_filter,
                lead_user_filter,
                lead_status_filter,
                lead_payment_filter,
                lead_rows_per_page,
                lead_page,
                lead_exported_count,
            );
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
            Card::new().show(ui, |ui| {
                if show_theme_editor(ui, seed) {
                    theme_changed = true;
                }
            });
            ui.add_space(12.0);
            show_palette_preview(ui, theme);
            ui.add_space(12.0);
            show_typography_diagnostics(ui, theme, pixels_per_point, zoom);
            ui.add_space(12.0);
            show_override_preview(ui);
        }
    };
    theme_changed
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
        "Badge radius",
        &mut seed.component_overrides.badge.radius,
        seed.radius.md * 2.0,
        0.0..=28.0,
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
            Tooltip::new("Tooltips inherit Cast theme colors, type, radius, and elevation.")
                .title("Tooltip")
                .show(ui, |ui| {
                    ui.add(
                        Button::new("Hover details")
                            .intent(Intent::Neutral)
                            .variant(Variant::Outline),
                    );
                });
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

fn show_menus(
    ui: &mut egui::Ui,
    menu_choice: &mut usize,
    dialog_open: &mut bool,
    command_palette: &mut CommandPaletteState,
) {
    Card::new().show(ui, |ui| {
        ui.heading("Menus and dropdowns");
        ui.horizontal_wrapped(|ui| {
            ui.add(
                Dropdown::new(
                    menu_choice,
                    ["Overview", "Components", "Theme lab", "Diagnostics"],
                )
                .width(220.0),
            );
            ui.add(
                Dropdown::new(menu_choice, ["Small", "Medium", "Large"])
                    .size(Size::Small)
                    .width(144.0),
            );
        });
        ui.add_space(8.0);
        CastPanel::new().show(ui, |ui| {
            ui.label("Menu items");
            ui.add_space(4.0);
            if ui
                .add(MenuItem::new("Open command palette").selected(true))
                .clicked()
            {
                command_palette.open = true;
            }
            ui.add(MenuItem::new("Duplicate theme"));
            ui.add(MenuItem::new("Archive preset").intent(Intent::Warning));
            ui.add(MenuItem::new("Delete preset").intent(Intent::Danger));
            ui.add(MenuItem::new("Unavailable action").disabled());
        });
        ui.add_space(8.0);
        Popover::new()
            .title("Popover")
            .body("A richer anchored overlay for compact settings and contextual actions.")
            .width(280.0)
            .show(
                ui,
                |ui| {
                    ui.add(
                        Button::new("Open popover")
                            .intent(Intent::Neutral)
                            .variant(Variant::Outline),
                    )
                },
                |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.add(Badge::new("Anchored").intent(Intent::Info));
                        ui.add(Badge::new("Composable").intent(Intent::Secondary));
                    });
                    ui.add_space(6.0);
                    ui.add(Button::new("Apply").size(Size::Small));
                },
            );
        ui.add_space(8.0);
        if ui
            .add(
                Button::new("Open dialog")
                    .intent(Intent::Neutral)
                    .variant(Variant::Outline),
            )
            .clicked()
        {
            *dialog_open = true;
        }
    });

    Dialog::new(dialog_open, "gallery_dialog")
        .title("Dialog")
        .description("A blocking surface for focused decisions, confirmations, and short forms.")
        .width(440.0)
        .show(ui.ctx(), |ui, dialog| {
            ui.label("Use dialogs when the surrounding workspace should pause until the user makes a choice.");
            ui.add_space(12.0);
            ui.horizontal_wrapped(|ui| {
                ui.add(Badge::new("Esc closes").intent(Intent::Neutral));
                ui.add(Badge::new("Backdrop closes").intent(Intent::Info));
            });
            ui.add_space(18.0);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.add(Button::new("Confirm").size(Size::Small)).clicked() {
                    dialog.close();
                }
                if ui
                    .add(
                        Button::new("Cancel")
                            .intent(Intent::Neutral)
                            .variant(Variant::Outline)
                            .size(Size::Small),
                    )
                    .clicked()
                {
                    dialog.close();
                }
            });
        });
}

#[allow(clippy::too_many_arguments)]
fn show_lists_and_tables(
    ui: &mut egui::Ui,
    _lead_search: &mut String,
    related_activity_open: &mut bool,
    related_activity_group: &mut Option<usize>,
    lead_selected: &mut [bool; LEAD_COUNT],
    lead_expanded: &mut [bool; LEAD_COUNT],
    _date_filter: &mut usize,
    _user_filter: &mut usize,
    _status_filter: &mut usize,
    _payment_filter: &mut usize,
    rows_per_page: &mut usize,
    page: &mut usize,
    exported_count: &mut Option<usize>,
) {
    Card::new().show(ui, |ui| {
        show_entity_table_with_details(
            ui,
            &LEADS,
            EntityTableState {
                selected: lead_selected,
                expanded: lead_expanded,
                rows_per_page,
                page,
                exported_count,
            },
        );
        ui.add_space(12.0);
        show_related_activity(ui, related_activity_open, related_activity_group);
    });
}

type LeadRecord = EntityRecord;

#[cfg(test)]
const LEAD_USER_FILTERS: [&str; 6] = [
    "All users",
    "Sarah P.",
    "Alex W.",
    "Jane D.",
    "John S.",
    "Ali M.",
];
#[cfg(test)]
const LEAD_STATUS_FILTERS: [&str; 7] = [
    "Any status",
    "Won",
    "Call booked",
    "Qualified",
    "Unqualified",
    "Lost",
    "No show",
];
#[cfg(test)]
const LEAD_PAYMENT_FILTERS: [&str; 4] = ["All payments", "Paid", "Pending", "No value"];

const LEADS: [LeadRecord; 24] = [
    LeadRecord {
        name: "Sarah Parker",
        status: "Won",
        interest: "Interested",
        source: "Sponsored ad",
        deal_value: "$ 2,500.00",
        payment: "Paid",
        assigned_to: "Sarah P.",
        interacted: "2 days ago",
        days_ago: 2,
    },
    LeadRecord {
        name: "Mike Brown",
        status: "Call booked",
        interest: "Broke",
        source: "Direct message",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Alex W.",
        interacted: "3 hours ago",
        days_ago: 0,
    },
    LeadRecord {
        name: "Linda Chen",
        status: "Unqualified",
        interest: "Achiever",
        source: "Story replies",
        deal_value: "N/A",
        payment: "No value",
        assigned_to: "Jane D.",
        interacted: "1 week ago",
        days_ago: 7,
    },
    LeadRecord {
        name: "David Lee",
        status: "Won",
        interest: "Interested",
        source: "Story replies",
        deal_value: "$ 5,000.00",
        payment: "Paid",
        assigned_to: "John S.",
        interacted: "4 days ago",
        days_ago: 4,
    },
    LeadRecord {
        name: "Emily White",
        status: "No show",
        interest: "Interested",
        source: "Direct message",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Ali M.",
        interacted: "15 mins ago",
        days_ago: 0,
    },
    LeadRecord {
        name: "Jessica Wong",
        status: "Won",
        interest: "Interested",
        source: "Sponsored ad",
        deal_value: "$ 3,000.00",
        payment: "Paid",
        assigned_to: "Sarah P.",
        interacted: "1 week ago",
        days_ago: 7,
    },
    LeadRecord {
        name: "Kevin Harris",
        status: "Qualified",
        interest: "N/A",
        source: "Story replies",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Jane D.",
        interacted: "1 day ago",
        days_ago: 1,
    },
    LeadRecord {
        name: "Tom Clark",
        status: "Lost",
        interest: "Broke",
        source: "Direct message",
        deal_value: "N/A",
        payment: "No value",
        assigned_to: "John S.",
        interacted: "6 days ago",
        days_ago: 6,
    },
    LeadRecord {
        name: "Laura Lewis",
        status: "Qualified",
        interest: "Achiever",
        source: "Story replies",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Ali M.",
        interacted: "30 mins ago",
        days_ago: 0,
    },
    LeadRecord {
        name: "Brian Walker",
        status: "Call booked",
        interest: "Interested",
        source: "Story replies",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Alex W.",
        interacted: "2 days ago",
        days_ago: 2,
    },
    LeadRecord {
        name: "Daniel Hall",
        status: "Won",
        interest: "Interested",
        source: "Direct message",
        deal_value: "$ 1,500.00",
        payment: "Paid",
        assigned_to: "John S.",
        interacted: "5 days ago",
        days_ago: 5,
    },
    LeadRecord {
        name: "Nancy Allen",
        status: "Unqualified",
        interest: "Interested",
        source: "Sponsored ad",
        deal_value: "N/A",
        payment: "No value",
        assigned_to: "Jane D.",
        interacted: "2 weeks ago",
        days_ago: 14,
    },
    LeadRecord {
        name: "Paul Young",
        status: "Qualified",
        interest: "N/A",
        source: "Story replies",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Ali M.",
        interacted: "1 hour ago",
        days_ago: 0,
    },
    LeadRecord {
        name: "Sandra King",
        status: "Won",
        interest: "Broke",
        source: "Direct message",
        deal_value: "$ 4,200.00",
        payment: "Paid",
        assigned_to: "Jane D.",
        interacted: "6 days ago",
        days_ago: 6,
    },
    LeadRecord {
        name: "Omar Diaz",
        status: "Call booked",
        interest: "Interested",
        source: "Sponsored ad",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Sarah P.",
        interacted: "20 mins ago",
        days_ago: 0,
    },
    LeadRecord {
        name: "Priya Nair",
        status: "Lost",
        interest: "Achiever",
        source: "Story replies",
        deal_value: "N/A",
        payment: "No value",
        assigned_to: "Alex W.",
        interacted: "8 days ago",
        days_ago: 8,
    },
    LeadRecord {
        name: "Hannah Brooks",
        status: "Won",
        interest: "Interested",
        source: "Direct message",
        deal_value: "$ 6,750.00",
        payment: "Paid",
        assigned_to: "John S.",
        interacted: "1 day ago",
        days_ago: 1,
    },
    LeadRecord {
        name: "Victor Stone",
        status: "No show",
        interest: "Broke",
        source: "Sponsored ad",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Ali M.",
        interacted: "12 hours ago",
        days_ago: 0,
    },
    LeadRecord {
        name: "Mina Patel",
        status: "Qualified",
        interest: "Achiever",
        source: "Story replies",
        deal_value: "$ 900.00",
        payment: "Paid",
        assigned_to: "Jane D.",
        interacted: "3 days ago",
        days_ago: 3,
    },
    LeadRecord {
        name: "Ethan Ross",
        status: "Unqualified",
        interest: "Interested",
        source: "Direct message",
        deal_value: "N/A",
        payment: "No value",
        assigned_to: "Alex W.",
        interacted: "2 weeks ago",
        days_ago: 14,
    },
    LeadRecord {
        name: "Grace Kim",
        status: "Won",
        interest: "Interested",
        source: "Story replies",
        deal_value: "$ 2,200.00",
        payment: "Paid",
        assigned_to: "Sarah P.",
        interacted: "5 days ago",
        days_ago: 5,
    },
    LeadRecord {
        name: "Noah Smith",
        status: "Call booked",
        interest: "N/A",
        source: "Sponsored ad",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "John S.",
        interacted: "4 hours ago",
        days_ago: 0,
    },
    LeadRecord {
        name: "Iris Chen",
        status: "Lost",
        interest: "Broke",
        source: "Direct message",
        deal_value: "N/A",
        payment: "No value",
        assigned_to: "Ali M.",
        interacted: "9 days ago",
        days_ago: 9,
    },
    LeadRecord {
        name: "Leo Martin",
        status: "Qualified",
        interest: "Interested",
        source: "Story replies",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Jane D.",
        interacted: "7 days ago",
        days_ago: 7,
    },
];

const _: [(); LEAD_COUNT] = [(); LEADS.len()];

#[cfg(test)]
fn filtered_leads(
    query: &str,
    date_filter: usize,
    user_filter: usize,
    status_filter: usize,
    payment_filter: usize,
) -> Vec<&'static LeadRecord> {
    let query = query.trim().to_lowercase();
    LEADS
        .iter()
        .filter(|lead| lead_matches_date_filter(lead, date_filter))
        .filter(|lead| lead_matches_choice(lead.assigned_to, LEAD_USER_FILTERS, user_filter))
        .filter(|lead| lead_matches_choice(lead.status, LEAD_STATUS_FILTERS, status_filter))
        .filter(|lead| lead_matches_choice(lead.payment, LEAD_PAYMENT_FILTERS, payment_filter))
        .filter(|lead| {
            query.is_empty()
                || [
                    lead.name,
                    lead.status,
                    lead.interest,
                    lead.source,
                    lead.deal_value,
                    lead.assigned_to,
                    lead.interacted,
                ]
                .iter()
                .any(|value| value.to_lowercase().contains(&query))
        })
        .collect()
}

#[cfg(test)]
fn lead_matches_date_filter(lead: &LeadRecord, filter: usize) -> bool {
    match filter {
        1 => lead.days_ago <= 7,
        2 => lead.days_ago == 0,
        _ => true,
    }
}

#[cfg(test)]
fn lead_matches_choice<const N: usize>(value: &str, labels: [&str; N], index: usize) -> bool {
    index == 0 || labels.get(index).is_some_and(|label| *label == value)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lead_filters_apply_status_and_payment() {
        let filtered = filtered_leads("", 0, 0, 1, 1);

        assert!(!filtered.is_empty());
        assert!(filtered.iter().all(|lead| lead.status == "Won"));
        assert!(filtered.iter().all(|lead| lead.payment == "Paid"));
    }

    #[test]
    fn lead_search_matches_visible_table_fields() {
        let filtered = filtered_leads("alex", 0, 0, 0, 0);

        assert!(!filtered.is_empty());
        assert!(filtered.iter().all(|lead| lead.assigned_to == "Alex W."));
    }

    #[test]
    fn last_twenty_four_hours_filter_uses_recent_rows() {
        let filtered = filtered_leads("", 2, 0, 0, 0);

        assert!(!filtered.is_empty());
        assert!(filtered.iter().all(|lead| lead.days_ago == 0));
    }

    #[test]
    fn rows_per_page_limit_is_state_backed() {
        assert_eq!(
            patterns::entity_table_with_details::rows_per_page_limit(0),
            5
        );
        assert_eq!(
            patterns::entity_table_with_details::rows_per_page_limit(1),
            10
        );
        assert_eq!(
            patterns::entity_table_with_details::rows_per_page_limit(2),
            25
        );
    }
}
